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
pub struct LolLobbyReceivedInvitationDto {
    #[serde(rename = "canAcceptInvitation", skip_serializing_if = "Option::is_none")]
    pub can_accept_invitation: Option<bool>,
    #[serde(rename = "fromSummonerId", skip_serializing_if = "Option::is_none")]
    pub from_summoner_id: Option<i64>,
    #[serde(rename = "fromSummonerName", skip_serializing_if = "Option::is_none")]
    pub from_summoner_name: Option<String>,
    #[serde(rename = "gameConfig", skip_serializing_if = "Option::is_none")]
    pub game_config: Option<crate::models::LolLobbyReceivedInvitationGameConfigDto>,
    #[serde(rename = "invitationId", skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Vec<crate::models::LolLobbyEligibilityRestriction>>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::LolLobbyLobbyInvitationState>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl LolLobbyReceivedInvitationDto {
    pub fn new() -> LolLobbyReceivedInvitationDto {
        LolLobbyReceivedInvitationDto {
            can_accept_invitation: None,
            from_summoner_id: None,
            from_summoner_name: None,
            game_config: None,
            invitation_id: None,
            restrictions: None,
            state: None,
            timestamp: None,
        }
    }
}


