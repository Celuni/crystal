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
pub struct LolLobbyPartyMemberMetadataDto {
    #[serde(rename = "championSelection", skip_serializing_if = "Option::is_none")]
    pub champion_selection: Option<i32>,
    #[serde(rename = "positionPref", skip_serializing_if = "Option::is_none")]
    pub position_pref: Option<Vec<String>>,
    #[serde(rename = "skinSelection", skip_serializing_if = "Option::is_none")]
    pub skin_selection: Option<i32>,
}

impl LolLobbyPartyMemberMetadataDto {
    pub fn new() -> LolLobbyPartyMemberMetadataDto {
        LolLobbyPartyMemberMetadataDto {
            champion_selection: None,
            position_pref: None,
            skin_selection: None,
        }
    }
}


