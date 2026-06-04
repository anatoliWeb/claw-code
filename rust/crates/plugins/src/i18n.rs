use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::PluginManifest;

const DEFAULT_LOCALE: &str = "en";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PluginI18nCatalog {
    translations: BTreeMap<String, String>,
    warnings: Vec<String>,
}

impl PluginI18nCatalog {
    #[must_use]
    pub fn lookup(&self, key: &str) -> Option<&str> {
        self.translations
            .get(key)
            .map(String::as_str)
            .filter(|value| !value.trim().is_empty())
    }

    #[must_use]
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
}

#[must_use]
pub fn load_plugin_i18n(plugin_root: &Path, lang: &str) -> PluginI18nCatalog {
    let locale = normalize_locale(lang);
    let mut catalog = PluginI18nCatalog::default();

    load_locale_into(plugin_root, DEFAULT_LOCALE, &mut catalog);
    if locale != DEFAULT_LOCALE {
        load_locale_into(plugin_root, &locale, &mut catalog);
    }

    catalog
}

pub fn translate_plugin_manifest(manifest: &mut PluginManifest, catalog: &PluginI18nCatalog) {
    replace_if_translated(
        &mut manifest.description,
        catalog.lookup("plugin.description"),
    );

    for tool in &mut manifest.tools {
        let tool_prefix = format!("tool.{}", tool.name);
        replace_if_translated(
            &mut tool.description,
            catalog.lookup(&format!("{tool_prefix}.description")),
        );
        translate_input_schema(&mut tool.input_schema, &tool_prefix, catalog);
    }

    for command in &mut manifest.commands {
        replace_if_translated(
            &mut command.description,
            catalog.lookup(&format!("command.{}.description", command.name)),
        );
    }
}

fn normalize_locale(lang: &str) -> String {
    let primary = lang
        .trim()
        .to_ascii_lowercase()
        .split(['-', '_'])
        .next()
        .unwrap_or_default()
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect::<String>();
    if primary.is_empty() {
        DEFAULT_LOCALE.to_string()
    } else {
        primary
    }
}

fn load_locale_into(plugin_root: &Path, locale: &str, catalog: &mut PluginI18nCatalog) {
    let path = plugin_root.join("i18n").join(format!("{locale}.properties"));
    if !path.exists() {
        return;
    }

    match read_properties(&path) {
        Ok(translations) => catalog.translations.extend(translations),
        Err(error) => catalog.warnings.push(format!(
            "failed to load plugin translations from `{}`: {error}",
            path.display()
        )),
    }
}

fn read_properties(path: &Path) -> Result<BTreeMap<String, String>, String> {
    let contents =
        fs::read_to_string(path).map_err(|error| format!("unable to read file: {error}"))?;
    parse_properties(&contents, path)
}

fn parse_properties(contents: &str, path: &Path) -> Result<BTreeMap<String, String>, String> {
    let mut translations = BTreeMap::new();
    for (index, raw_line) in contents.lines().enumerate() {
        let line = raw_line.trim_start_matches('\u{feff}').trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!(
                "invalid properties syntax at {}:{}; expected key=value",
                path.display(),
                index + 1
            ));
        };
        let key = key.trim();
        if key.is_empty() {
            return Err(format!(
                "invalid properties syntax at {}:{}; key cannot be empty",
                path.display(),
                index + 1
            ));
        }
        translations.insert(key.to_string(), value.trim().to_string());
    }
    Ok(translations)
}

fn replace_if_translated(value: &mut String, translated: Option<&str>) {
    if let Some(translated) = translated {
        value.clear();
        value.push_str(translated);
    }
}

fn translate_input_schema(
    schema: &mut Value,
    tool_prefix: &str,
    catalog: &PluginI18nCatalog,
) {
    if schema.get("description").is_some() {
        if let Some(translated) = catalog.lookup(&format!("{tool_prefix}.input.description")) {
            schema["description"] = Value::String(translated.to_string());
        }
    }

    translate_schema_properties(schema, tool_prefix, "", catalog);
}

fn translate_schema_properties(
    schema: &mut Value,
    tool_prefix: &str,
    parent_path: &str,
    catalog: &PluginI18nCatalog,
) {
    let Some(properties) = schema.get_mut("properties").and_then(Value::as_object_mut) else {
        return;
    };

    for (field_name, field_schema) in properties {
        let field_path = if parent_path.is_empty() {
            field_name.clone()
        } else {
            format!("{parent_path}.{field_name}")
        };
        let key = format!("{tool_prefix}.input.{field_path}.description");
        if field_schema.get("description").is_some() {
            if let Some(translated) = catalog.lookup(&key) {
                field_schema["description"] = Value::String(translated.to_string());
            }
        }
        translate_schema_properties(field_schema, tool_prefix, &field_path, catalog);
    }
}
