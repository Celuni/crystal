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
pub struct LolHighlightsHighlightsSettingsData {
    #[serde(rename = "highlights-folder-path", skip_serializing_if = "Option::is_none")]
    pub highlights_folder_path: Option<String>,
}

impl LolHighlightsHighlightsSettingsData {
    pub fn new() -> LolHighlightsHighlightsSettingsData {
        LolHighlightsHighlightsSettingsData {
            highlights_folder_path: None,
        }
    }
}


