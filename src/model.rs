use serde_json::Value;
use serde::{Deserialize, Serialize};

/// A struct representing a table header with extra data
/// 
/// # Arguments
/// 
/// * `name` - The name of the column in the JSON
/// * `sort_name` - The name of the column to sort by (can be the name of the column in database to be use in sorting)
/// * `display_name` - The name of the column to display in the table
/// * `is_currency` - A boolean to check if the column is a currency (if true you can append a certain column of currency)
/// * `currency` - The name of the column of a currency
/// * `is_number_styled` - A boolean to check if the column is a number (if true style is "text-success" else "text-error")
/// * `default_value` - use this if the data is not parseable
/// * `style_when_success` - Add "text-success" to this column if the value is equals to this value
/// * `style_when_error` - Add "text-error" to this column if the value is equals to this value
/// * `to_uppercase` - A boolean to check if the column is a string (if true the value will be converted to uppercase)

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
    pub prefix: Option<String>,
}

impl TableHeader {
    pub fn new(name: &str, sort_name: &str, display_name: &str, is_currency: bool, currency: &str, is_number_styled: bool, default_value: &str, style_when_success: &str, style_when_error: &str, to_uppercase: bool, prefix: Option<String>) -> Self {
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
            to_uppercase,
            prefix,
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

/// A struct for download data request, using parameters that are common, implementation is base on user
/// 
/// # Arguments
/// 
/// * `table_name` - The name of the table to be downloaded
/// * `filter` - The filter to be used in downloading
/// * `fields` - The fields to be downloaded
/// * `search` - The search to be used in downloading
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct DownloadDataRequest {
    pub table_name: String,
    pub filter: String,
    pub fields: String,
    pub search: String,
}