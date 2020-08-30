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
pub struct LolLobbyLobbyParticipantDto {
    #[serde(rename = "allowedChangeActivity", skip_serializing_if = "Option::is_none")]
    pub allowed_change_activity: Option<bool>,
    #[serde(rename = "allowedInviteOthers", skip_serializing_if = "Option::is_none")]
    pub allowed_invite_others: Option<bool>,
    #[serde(rename = "allowedKickOthers", skip_serializing_if = "Option::is_none")]
    pub allowed_kick_others: Option<bool>,
    #[serde(rename = "allowedStartActivity", skip_serializing_if = "Option::is_none")]
    pub allowed_start_activity: Option<bool>,
    #[serde(rename = "allowedToggleInvite", skip_serializing_if = "Option::is_none")]
    pub allowed_toggle_invite: Option<bool>,
    #[serde(rename = "autoFillEligible", skip_serializing_if = "Option::is_none")]
    pub auto_fill_eligible: Option<bool>,
    #[serde(rename = "autoFillProtectedForPromos", skip_serializing_if = "Option::is_none")]
    pub auto_fill_protected_for_promos: Option<bool>,
    #[serde(rename = "autoFillProtectedForSoloing", skip_serializing_if = "Option::is_none")]
    pub auto_fill_protected_for_soloing: Option<bool>,
    #[serde(rename = "autoFillProtectedForStreaking", skip_serializing_if = "Option::is_none")]
    pub auto_fill_protected_for_streaking: Option<bool>,
    #[serde(rename = "botChampionId", skip_serializing_if = "Option::is_none")]
    pub bot_champion_id: Option<i32>,
    #[serde(rename = "botDifficulty", skip_serializing_if = "Option::is_none")]
    pub bot_difficulty: Option<crate::models::LolLobbyLobbyBotDifficulty>,
    #[serde(rename = "botId", skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "firstPositionPreference", skip_serializing_if = "Option::is_none")]
    pub first_position_preference: Option<String>,
    #[serde(rename = "isBot", skip_serializing_if = "Option::is_none")]
    pub is_bot: Option<bool>,
    #[serde(rename = "isLeader", skip_serializing_if = "Option::is_none")]
    pub is_leader: Option<bool>,
    #[serde(rename = "isSpectator", skip_serializing_if = "Option::is_none")]
    pub is_spectator: Option<bool>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "ready", skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(rename = "secondPositionPreference", skip_serializing_if = "Option::is_none")]
    pub second_position_preference: Option<String>,
    #[serde(rename = "showGhostedBanner", skip_serializing_if = "Option::is_none")]
    pub show_ghosted_banner: Option<bool>,
    #[serde(rename = "summonerIconId", skip_serializing_if = "Option::is_none")]
    pub summoner_icon_id: Option<i32>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "summonerInternalName", skip_serializing_if = "Option::is_none")]
    pub summoner_internal_name: Option<String>,
    #[serde(rename = "summonerLevel", skip_serializing_if = "Option::is_none")]
    pub summoner_level: Option<i32>,
    #[serde(rename = "summonerName", skip_serializing_if = "Option::is_none")]
    pub summoner_name: Option<String>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i32>,
}

impl LolLobbyLobbyParticipantDto {
    pub fn new() -> LolLobbyLobbyParticipantDto {
        LolLobbyLobbyParticipantDto {
            allowed_change_activity: None,
            allowed_invite_others: None,
            allowed_kick_others: None,
            allowed_start_activity: None,
            allowed_toggle_invite: None,
            auto_fill_eligible: None,
            auto_fill_protected_for_promos: None,
            auto_fill_protected_for_soloing: None,
            auto_fill_protected_for_streaking: None,
            bot_champion_id: None,
            bot_difficulty: None,
            bot_id: None,
            first_position_preference: None,
            is_bot: None,
            is_leader: None,
            is_spectator: None,
            puuid: None,
            ready: None,
            second_position_preference: None,
            show_ghosted_banner: None,
            summoner_icon_id: None,
            summoner_id: None,
            summoner_internal_name: None,
            summoner_level: None,
            summoner_name: None,
            team_id: None,
        }
    }
}

