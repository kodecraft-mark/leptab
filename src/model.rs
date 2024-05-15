use serde_json::Value;
use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize, Clone, Default)]
// pub struct CollectionCountResponse {
//     data: Vec<CollectionCount>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone, Default)]
// pub struct CollectionCount {
//     pub count: String,
// }

// impl CollectionCountResponse {
//     pub fn get_count(&self) -> u32 {
//         self.data.first().unwrap().count.parse::<u32>().unwrap_or_default()
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TableHeader {
    pub name: String,
    pub sort_name: String,
    pub display_name: String,
    pub is_currency: bool,
    pub currency: String,
    pub is_number_styled: bool,
    pub default_value: String,
    pub style_when_success: String,
    pub style_when_error: String,
    pub to_uppercase: bool,
}

impl TableHeader {
    pub fn new(name: &str, sort_name: &str, display_name: &str, is_currency: bool, currency: &str, is_number_styled: bool, default_value: &str, style_when_success: &str, style_when_error: &str, to_uppercase: bool) -> Self {
        Self {
            name: name.to_string(),
            sort_name: sort_name.to_string(),
            display_name: display_name.to_string(),
            is_currency,
            currency: currency.to_string(),
            is_number_styled,
            default_value: default_value.to_string(),
            style_when_success: style_when_success.to_string(),
            style_when_error: style_when_error.to_string(),
            to_uppercase
        }
    }

    pub fn find(&self, json_value: &Value) -> String {
        match json_value.get(&self.name) {
            Some(value) => {
                match value {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => self.default_value.clone(),
                }
            },
            None => self.default_value.clone(),
        }
    }
    pub fn find_currency(&self, json_value: &Value) -> String {
        match json_value.get(&self.currency) {
            Some(value) => {
                match value {
                    Value::String(s) => s.clone(),
                    _ => String::from(""),
                }
            },
            None => String::from(""),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DataDownload {
    pub is_allowed: bool,
    pub file_name: String,
    pub file_content: String,
}

impl DataDownload {
    pub fn new(is_allowed: bool, file_name: &str, file_content: &str) -> Self {
        Self {
            is_allowed,
            file_name: file_name.to_string(),
            file_content: file_content.to_string(),
        }
    }
}