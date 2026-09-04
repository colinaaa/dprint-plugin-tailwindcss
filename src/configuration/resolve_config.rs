use dprint_core::configuration::*;

use super::Configuration;

pub fn resolve_config(
    config: ConfigKeyMap,
    _global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
    let mut diagnostics = Vec::new();
    let mut config = config;

    let tailwind_attributes: Vec<String> = get_nullable_vec(
        &mut config,
        "tailwindAttributes",
        |value, _index, diagnostics| match value {
            ConfigKeyValue::String(s) => Some(s),
            _ => {
                diagnostics.push(ConfigurationDiagnostic {
                    property_name: "tailwindAttributes".to_string(),
                    message: "Expected a string value in array.".to_string(),
                });
                None
            }
        },
        &mut diagnostics,
    )
    .unwrap_or_default();

    let tailwind_functions: Vec<String> = get_nullable_vec(
        &mut config,
        "tailwindFunctions",
        |value, _index, diagnostics| match value {
            ConfigKeyValue::String(s) => Some(s),
            _ => {
                diagnostics.push(ConfigurationDiagnostic {
                    property_name: "tailwindFunctions".to_string(),
                    message: "Expected a string value in array.".to_string(),
                });
                None
            }
        },
        &mut diagnostics,
    )
    .unwrap_or_default();

    let resolved_config = Configuration {
        remove_duplicates: get_value(&mut config, "removeDuplicates", true, &mut diagnostics),
        collapse_whitespace: get_value(&mut config, "collapseWhitespace", true, &mut diagnostics),
        tailwind_attributes,
        tailwind_functions,
    };

    diagnostics.extend(get_unknown_property_diagnostics(config));

    ResolveConfigurationResult {
        config: resolved_config,
        diagnostics,
    }
}

#[cfg(test)]
mod tests {
    use dprint_core::configuration::{ConfigKeyMap, ConfigKeyValue, GlobalConfiguration};

    use super::resolve_config;

    #[test]
    fn defaults_to_remove_duplicates_true_and_collapse_whitespace_true() {
        let result = resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        assert!(result.diagnostics.is_empty());
        assert!(result.config.remove_duplicates);
        assert!(result.config.collapse_whitespace);
        assert!(result.config.tailwind_attributes.is_empty());
        assert!(result.config.tailwind_functions.is_empty());
    }

    #[test]
    fn reads_config_values_and_reports_unknown_properties() {
        let result = resolve_config(
            ConfigKeyMap::from([
                ("removeDuplicates".to_string(), false.into()),
                ("collapseWhitespace".to_string(), false.into()),
                ("unknownProp".to_string(), true.into()),
            ]),
            &GlobalConfiguration::default(),
        );
        assert!(!result.config.remove_duplicates);
        assert!(!result.config.collapse_whitespace);
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].property_name, "unknownProp");
    }

    #[test]
    fn reads_tailwind_attributes_and_functions() {
        let result = resolve_config(
            ConfigKeyMap::from([
                (
                    "tailwindAttributes".to_string(),
                    ConfigKeyValue::Array(vec![
                        ConfigKeyValue::String("myClass".to_string()),
                        ConfigKeyValue::String("/data-tw-.*/".to_string()),
                    ]),
                ),
                (
                    "tailwindFunctions".to_string(),
                    ConfigKeyValue::Array(vec![
                        ConfigKeyValue::String("clsx".to_string()),
                        ConfigKeyValue::String("cn".to_string()),
                    ]),
                ),
            ]),
            &GlobalConfiguration::default(),
        );
        assert!(result.diagnostics.is_empty());
        assert_eq!(
            result.config.tailwind_attributes,
            vec!["myClass", "/data-tw-.*/"]
        );
        assert_eq!(result.config.tailwind_functions, vec!["clsx", "cn"]);
    }
}
