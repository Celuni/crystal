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
pub struct LolFeaturedModesMaps {
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "gameModeName", skip_serializing_if = "Option::is_none")]
    pub game_mode_name: Option<String>,
    #[serde(rename = "isRGM", skip_serializing_if = "Option::is_none")]
    pub is_rgm: Option<bool>,
}

impl LolFeaturedModesMaps {
    pub fn new() -> LolFeaturedModesMaps {
        LolFeaturedModesMaps {
            assets: None,
            game_mode_name: None,
            is_rgm: None,
        }
    }
}


