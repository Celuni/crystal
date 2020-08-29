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
pub struct LolLobbyQueueGameTypeConfig {
    #[serde(rename = "advancedLearningQuests", skip_serializing_if = "Option::is_none")]
    pub advanced_learning_quests: Option<bool>,
    #[serde(rename = "allowTrades", skip_serializing_if = "Option::is_none")]
    pub allow_trades: Option<bool>,
    #[serde(rename = "banMode", skip_serializing_if = "Option::is_none")]
    pub ban_mode: Option<String>,
    #[serde(rename = "banTimerDuration", skip_serializing_if = "Option::is_none")]
    pub ban_timer_duration: Option<i32>,
    #[serde(rename = "battleBoost", skip_serializing_if = "Option::is_none")]
    pub battle_boost: Option<bool>,
    #[serde(rename = "crossTeamChampionPool", skip_serializing_if = "Option::is_none")]
    pub cross_team_champion_pool: Option<bool>,
    #[serde(rename = "deathMatch", skip_serializing_if = "Option::is_none")]
    pub death_match: Option<bool>,
    #[serde(rename = "doNotRemove", skip_serializing_if = "Option::is_none")]
    pub do_not_remove: Option<bool>,
    #[serde(rename = "duplicatePick", skip_serializing_if = "Option::is_none")]
    pub duplicate_pick: Option<bool>,
    #[serde(rename = "exclusivePick", skip_serializing_if = "Option::is_none")]
    pub exclusive_pick: Option<bool>,
    #[serde(rename = "gameModeOverride", skip_serializing_if = "Option::is_none")]
    pub game_mode_override: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "learningQuests", skip_serializing_if = "Option::is_none")]
    pub learning_quests: Option<bool>,
    #[serde(rename = "mainPickTimerDuration", skip_serializing_if = "Option::is_none")]
    pub main_pick_timer_duration: Option<i32>,
    #[serde(rename = "maxAllowableBans", skip_serializing_if = "Option::is_none")]
    pub max_allowable_bans: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numPlayersPerTeamOverride", skip_serializing_if = "Option::is_none")]
    pub num_players_per_team_override: Option<i32>,
    #[serde(rename = "onboardCoopBeginner", skip_serializing_if = "Option::is_none")]
    pub onboard_coop_beginner: Option<bool>,
    #[serde(rename = "pickMode", skip_serializing_if = "Option::is_none")]
    pub pick_mode: Option<String>,
    #[serde(rename = "postPickTimerDuration", skip_serializing_if = "Option::is_none")]
    pub post_pick_timer_duration: Option<i32>,
    #[serde(rename = "reroll", skip_serializing_if = "Option::is_none")]
    pub reroll: Option<bool>,
    #[serde(rename = "teamChampionPool", skip_serializing_if = "Option::is_none")]
    pub team_champion_pool: Option<bool>,
}

impl LolLobbyQueueGameTypeConfig {
    pub fn new() -> LolLobbyQueueGameTypeConfig {
        LolLobbyQueueGameTypeConfig {
            advanced_learning_quests: None,
            allow_trades: None,
            ban_mode: None,
            ban_timer_duration: None,
            battle_boost: None,
            cross_team_champion_pool: None,
            death_match: None,
            do_not_remove: None,
            duplicate_pick: None,
            exclusive_pick: None,
            game_mode_override: None,
            id: None,
            learning_quests: None,
            main_pick_timer_duration: None,
            max_allowable_bans: None,
            name: None,
            num_players_per_team_override: None,
            onboard_coop_beginner: None,
            pick_mode: None,
            post_pick_timer_duration: None,
            reroll: None,
            team_champion_pool: None,
        }
    }
}


