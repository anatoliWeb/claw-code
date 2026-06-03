use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    generate_locale_catalog();

    // Get git SHA (short hash)
    let git_sha = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map_or_else(|| "unknown".to_string(), |s| s.trim().to_string());

    println!("cargo:rustc-env=GIT_SHA={git_sha}");

    // TARGET is always set by Cargo during build
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=TARGET={target}");

    // Build date from SOURCE_DATE_EPOCH (reproducible builds) or current UTC date.
    // Intentionally ignoring time component to keep output deterministic within a day.
    let build_date = std::env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|epoch| epoch.parse::<i64>().ok())
        .map(|_ts| {
            // Use SOURCE_DATE_EPOCH to derive date via chrono if available;
            // for simplicity we just use the env var as a signal and fall back
            // to build-time env. In practice CI sets this via workflow.
            std::env::var("BUILD_DATE").unwrap_or_else(|_| "unknown".to_string())
        })
        .or_else(|| std::env::var("BUILD_DATE").ok())
        .unwrap_or_else(|| {
            // Fall back to current date via `date` command
            Command::new("date")
                .args(["+%Y-%m-%d"])
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        String::from_utf8(o.stdout).ok()
                    } else {
                        None
                    }
                })
                .map_or_else(|| "unknown".to_string(), |s| s.trim().to_string())
        });
    println!("cargo:rustc-env=BUILD_DATE={build_date}");

    // Rerun if git state changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
}

fn generate_locale_catalog() {
    println!("cargo:rerun-if-changed=i18n");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let i18n_dir = Path::new(&manifest_dir).join("i18n");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR");
    let out_path = Path::new(&out_dir).join("locale_catalog.rs");

    let mut locales = Vec::new();
    if let Ok(entries) = fs::read_dir(&i18n_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("properties") {
                continue;
            }
            println!("cargo:rerun-if-changed={}", path.display());
            let Some(locale) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            let contents = fs::read_to_string(&path).expect("read locale file");
            let mut pairs = Vec::new();
            for line in contents.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let Some((key, value)) = line.split_once('=') else {
                    continue;
                };
                pairs.push((key.trim().to_string(), value.trim().to_string()));
            }
            pairs.sort_by(|left, right| left.0.cmp(&right.0));
            locales.push((locale.to_string(), pairs));
        }
    }
    locales.sort_by(|left, right| left.0.cmp(&right.0));

    let mut generated = String::from("static CLI_LOCALE_CATALOG: &[(&str, &[(&str, &str)])] = &[\n");
    for (locale, pairs) in locales {
        generated.push_str(&format!("    ({locale:?}, &[\n"));
        for (key, value) in pairs {
            generated.push_str(&format!("        ({key:?}, {value:?}),\n"));
        }
        generated.push_str("    ]),\n");
    }
    generated.push_str("];\n");

    fs::write(out_path, generated).expect("write generated locale catalog");
}
