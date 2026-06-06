use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn main() {
    generate_locale_catalog();

    let git_sha =
        command_output("git", &["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".to_string());
    let git_sha_short = command_output("git", &["rev-parse", "--short=12", "HEAD"])
        .or_else(|| git_sha.get(..git_sha.len().min(12)).map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string());
    let git_dirty = command_output("git", &["status", "--porcelain"])
        .map(|status| (!status.trim().is_empty()).to_string())
        .unwrap_or_else(|| "false".to_string());
    let git_branch = command_output("git", &["branch", "--show-current"])
        .unwrap_or_else(|| "unknown".to_string());
    let git_commit_date = command_output("git", &["show", "-s", "--format=%cI", "HEAD"])
        .unwrap_or_else(|| "unknown".to_string());
    let git_commit_timestamp = command_output("git", &["show", "-s", "--format=%ct", "HEAD"])
        .unwrap_or_else(|| "unknown".to_string());
    let rustc_version =
        command_output("rustc", &["--version"]).unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_SHA={git_sha}");
    println!("cargo:rustc-env=GIT_SHA_SHORT={git_sha_short}");
    println!("cargo:rustc-env=GIT_DIRTY={git_dirty}");
    println!("cargo:rustc-env=GIT_BRANCH={git_branch}");
    println!("cargo:rustc-env=GIT_COMMIT_DATE={git_commit_date}");
    println!("cargo:rustc-env=GIT_COMMIT_TIMESTAMP={git_commit_timestamp}");
    println!("cargo:rustc-env=RUSTC_VERSION={rustc_version}");

    // TARGET is always set by Cargo during build.
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    println!("cargo:rustc-env=TARGET={target}");

    let build_date = env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|epoch| epoch.parse::<i64>().ok())
        .map(|_ts| env::var("BUILD_DATE").unwrap_or_else(|_| "unknown".to_string()))
        .or_else(|| env::var("BUILD_DATE").ok())
        .unwrap_or_else(|| {
            command_output("date", &["+%Y-%m-%d"]).unwrap_or_else(|| "unknown".to_string())
        });
    println!("cargo:rustc-env=BUILD_DATE={build_date}");

    // Rerun if git state changes. Paths are relative to this package root.
    println!("cargo:rerun-if-changed=../../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../../.git/refs");
    println!("cargo:rerun-if-changed=../../../.git/index");
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
