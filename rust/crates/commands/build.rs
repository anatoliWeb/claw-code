use std::env;
use std::fs;
use std::path::Path;

fn main() {
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

    let mut generated = String::from(
        "static LOCALE_CATALOG: &[(&str, &[(&str, &str)])] = &[\n",
    );
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
