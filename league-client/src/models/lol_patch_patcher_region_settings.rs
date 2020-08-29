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
pub struct LolPatchPatcherRegionSettings {
    #[serde(rename = "client_patcher", skip_serializing_if = "Option::is_none")]
    pub client_patcher: Option<String>,
    #[serde(rename = "game_patcher", skip_serializing_if = "Option::is_none")]
    pub game_patcher: Option<String>,
    #[serde(rename = "patchline", skip_serializing_if = "Option::is_none")]
    pub patchline: Option<String>,
}

impl LolPatchPatcherRegionSettings {
    pub fn new() -> LolPatchPatcherRegionSettings {
        LolPatchPatcherRegionSettings {
            client_patcher: None,
            game_patcher: None,
            patchline: None,
        }
    }
}


