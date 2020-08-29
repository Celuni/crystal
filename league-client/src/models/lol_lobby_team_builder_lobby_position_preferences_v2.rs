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
pub struct LolLobbyTeamBuilderLobbyPositionPreferencesV2 {
    #[serde(rename = "excludedPreference", skip_serializing_if = "Option::is_none")]
    pub excluded_preference: Option<String>,
    #[serde(rename = "firstPreference", skip_serializing_if = "Option::is_none")]
    pub first_preference: Option<String>,
    #[serde(rename = "secondPreference", skip_serializing_if = "Option::is_none")]
    pub second_preference: Option<String>,
}

impl LolLobbyTeamBuilderLobbyPositionPreferencesV2 {
    pub fn new() -> LolLobbyTeamBuilderLobbyPositionPreferencesV2 {
        LolLobbyTeamBuilderLobbyPositionPreferencesV2 {
            excluded_preference: None,
            first_preference: None,
            second_preference: None,
        }
    }
}


