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
pub struct LolMapsMaps {
    #[serde(rename = "assets", skip_serializing_if = "Option::is_none")]
    pub assets: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "categorizedContentBundles", skip_serializing_if = "Option::is_none")]
    pub categorized_content_bundles: Option<serde_json::Value>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "gameMode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<String>,
    #[serde(rename = "gameModeDescription", skip_serializing_if = "Option::is_none")]
    pub game_mode_description: Option<String>,
    #[serde(rename = "gameModeName", skip_serializing_if = "Option::is_none")]
    pub game_mode_name: Option<String>,
    #[serde(rename = "gameModeShortName", skip_serializing_if = "Option::is_none")]
    pub game_mode_short_name: Option<String>,
    #[serde(rename = "gameMutator", skip_serializing_if = "Option::is_none")]
    pub game_mutator: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "isRGM", skip_serializing_if = "Option::is_none")]
    pub is_rgm: Option<bool>,
    #[serde(rename = "mapStringId", skip_serializing_if = "Option::is_none")]
    pub map_string_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "platformId", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "platformName", skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "tutorialCards", skip_serializing_if = "Option::is_none")]
    pub tutorial_cards: Option<Vec<crate::models::LolMapsTutorialCard>>,
}

impl LolMapsMaps {
    pub fn new() -> LolMapsMaps {
        LolMapsMaps {
            assets: None,
            categorized_content_bundles: None,
            description: None,
            game_mode: None,
            game_mode_description: None,
            game_mode_name: None,
            game_mode_short_name: None,
            game_mutator: None,
            id: None,
            is_default: None,
            is_rgm: None,
            map_string_id: None,
            name: None,
            platform_id: None,
            platform_name: None,
            properties: None,
            tutorial_cards: None,
        }
    }
}


