/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SanitizerSanitizerStatus {
    #[serde(rename = "breakingCharsCount", skip_serializing_if = "Option::is_none")]
    pub breaking_chars_count: Option<i32>,
    #[serde(rename = "filteredWordCountsByLevel", skip_serializing_if = "Option::is_none")]
    pub filtered_word_counts_by_level: Option<::std::collections::HashMap<String, i32>>,
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "projectedCharsCount", skip_serializing_if = "Option::is_none")]
    pub projected_chars_count: Option<i32>,
    #[serde(rename = "ready", skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "whitelistedWordCountsByLevel", skip_serializing_if = "Option::is_none")]
    pub whitelisted_word_counts_by_level: Option<::std::collections::HashMap<String, i32>>,
}

impl SanitizerSanitizerStatus {
    pub fn new() -> SanitizerSanitizerStatus {
        SanitizerSanitizerStatus {
            breaking_chars_count: None,
            filtered_word_counts_by_level: None,
            locale: None,
            projected_chars_count: None,
            ready: None,
            region: None,
            whitelisted_word_counts_by_level: None,
        }
    }
}


