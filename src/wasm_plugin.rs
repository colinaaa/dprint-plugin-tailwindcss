use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
use dprint_core::generate_plugin_code;
use dprint_core::plugins::CheckConfigUpdatesMessage;
use dprint_core::plugins::ConfigChange;
use dprint_core::plugins::FormatError;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::PluginResolveConfigurationResult;
use dprint_core::plugins::SyncFormatRequest;
use dprint_core::plugins::SyncHostFormatRequest;
use dprint_core::plugins::SyncPluginHandler;

use crate::configuration;
use crate::configuration::Configuration;
use crate::format_text_with_range;
use crate::plugin_info::file_matching_info;
use crate::plugin_info::plugin_info;

struct TailwindcssPluginHandler;

impl SyncPluginHandler<Configuration> for TailwindcssPluginHandler {
    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        let config = configuration::resolve_config(config, global_config);
        PluginResolveConfigurationResult {
            config: config.config,
            diagnostics: config.diagnostics,
            file_matching: file_matching_info(),
        }
    }

    fn check_config_updates(
        &self,
        _message: CheckConfigUpdatesMessage,
    ) -> Result<Vec<ConfigChange>, FormatError> {
        Ok(Vec::new())
    }

    fn plugin_info(&mut self) -> PluginInfo {
        plugin_info()
    }

    fn license_text(&mut self) -> String {
        std::str::from_utf8(include_bytes!("../LICENSE"))
            .unwrap()
            .into()
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        let file_text = String::from_utf8(request.file_bytes)?;
        format_text_with_range(request.file_path, &file_text, request.range, request.config)
            .map(|maybe_text| maybe_text.map(|text| text.into_bytes()))
            .map_err(FormatError::new)
    }
}

generate_plugin_code!(TailwindcssPluginHandler, TailwindcssPluginHandler);
