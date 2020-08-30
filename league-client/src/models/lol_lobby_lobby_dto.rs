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
pub struct LolLobbyLobbyDto {
    #[serde(rename = "canStartActivity", skip_serializing_if = "Option::is_none")]
    pub can_start_activity: Option<bool>,
    #[serde(rename = "chatRoomId", skip_serializing_if = "Option::is_none")]
    pub chat_room_id: Option<String>,
    #[serde(rename = "chatRoomKey", skip_serializing_if = "Option::is_none")]
    pub chat_room_key: Option<String>,
    #[serde(rename = "gameConfig", skip_serializing_if = "Option::is_none")]
    pub game_config: Option<crate::models::LolLobbyLobbyGameConfigDto>,
    #[serde(rename = "invitations", skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<crate::models::LolLobbyLobbyInvitationDto>>,
    #[serde(rename = "localMember", skip_serializing_if = "Option::is_none")]
    pub local_member: Option<crate::models::LolLobbyLobbyParticipantDto>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::LolLobbyLobbyParticipantDto>>,
    #[serde(rename = "partyId", skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    #[serde(rename = "partyType", skip_serializing_if = "Option::is_none")]
    pub party_type: Option<String>,
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Vec<crate::models::LolLobbyEligibilityRestriction>>,
}

impl LolLobbyLobbyDto {
    pub fn new() -> LolLobbyLobbyDto {
        LolLobbyLobbyDto {
            can_start_activity: None,
            chat_room_id: None,
            chat_room_key: None,
            game_config: None,
            invitations: None,
            local_member: None,
            members: None,
            party_id: None,
            party_type: None,
            restrictions: None,
        }
    }
}

