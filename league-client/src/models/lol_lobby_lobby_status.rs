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
pub struct LolLobbyLobbyStatus {
    #[serde(rename = "allowedPlayAgain", skip_serializing_if = "Option::is_none")]
    pub allowed_play_again: Option<bool>,
    #[serde(rename = "customSpectatorPolicy", skip_serializing_if = "Option::is_none")]
    pub custom_spectator_policy: Option<crate::models::LolLobbyQueueCustomGameSpectatorPolicy>,
    #[serde(rename = "invitedSummonerIds", skip_serializing_if = "Option::is_none")]
    pub invited_summoner_ids: Option<Vec<i64>>,
    #[serde(rename = "isCustom", skip_serializing_if = "Option::is_none")]
    pub is_custom: Option<bool>,
    #[serde(rename = "isLeader", skip_serializing_if = "Option::is_none")]
    pub is_leader: Option<bool>,
    #[serde(rename = "isPracticeTool", skip_serializing_if = "Option::is_none")]
    pub is_practice_tool: Option<bool>,
    #[serde(rename = "isSpectator", skip_serializing_if = "Option::is_none")]
    pub is_spectator: Option<bool>,
    #[serde(rename = "lobbyId", skip_serializing_if = "Option::is_none")]
    pub lobby_id: Option<String>,
    #[serde(rename = "memberSummonerIds", skip_serializing_if = "Option::is_none")]
    pub member_summoner_ids: Option<Vec<i64>>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
}

impl LolLobbyLobbyStatus {
    pub fn new() -> LolLobbyLobbyStatus {
        LolLobbyLobbyStatus {
            allowed_play_again: None,
            custom_spectator_policy: None,
            invited_summoner_ids: None,
            is_custom: None,
            is_leader: None,
            is_practice_tool: None,
            is_spectator: None,
            lobby_id: None,
            member_summoner_ids: None,
            queue_id: None,
        }
    }
}


