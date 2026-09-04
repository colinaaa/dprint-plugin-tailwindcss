use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub remove_duplicates: bool,
    pub collapse_whitespace: bool,
    pub tailwind_attributes: Vec<String>,
    pub tailwind_functions: Vec<String>,
}
