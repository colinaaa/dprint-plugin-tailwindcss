#[allow(dead_code)]
use dprint_core::plugins::{FileMatchingInfo, PluginInfo};

#[allow(dead_code)]
pub const CONFIG_KEY: &str = "tailwindcss";

#[allow(dead_code)]
pub fn file_matching_info() -> FileMatchingInfo {
    FileMatchingInfo {
        file_extensions: vec![
            "html".to_string(),
            "htm".to_string(),
            "jsx".to_string(),
            "tsx".to_string(),
            "vue".to_string(),
            "svelte".to_string(),
            "astro".to_string(),
            "css".to_string(),
            "scss".to_string(),
            "less".to_string(),
        ],
        file_names: vec![],
    }
}

#[allow(dead_code)]
pub fn plugin_info() -> PluginInfo {
    let version = env!("CARGO_PKG_VERSION");
    PluginInfo {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: version.to_string(),
        config_key: CONFIG_KEY.to_string(),
        help_url: "https://github.com/colinaaa/dprint-plugin-tailwindcss".to_string(),
        config_schema_url: format!(
            "https://plugins.dprint.dev/colinaaa/dprint-plugin-tailwindcss/{version}/schema.json"
        ),
        update_url: None,
    }
}
