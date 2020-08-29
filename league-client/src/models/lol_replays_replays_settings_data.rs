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
pub struct LolReplaysReplaysSettingsData {
    #[serde(rename = "highlights-folder-path", skip_serializing_if = "Option::is_none")]
    pub highlights_folder_path: Option<String>,
    #[serde(rename = "replays-folder-path", skip_serializing_if = "Option::is_none")]
    pub replays_folder_path: Option<String>,
}

impl LolReplaysReplaysSettingsData {
    pub fn new() -> LolReplaysReplaysSettingsData {
        LolReplaysReplaysSettingsData {
            highlights_folder_path: None,
            replays_folder_path: None,
        }
    }
}


