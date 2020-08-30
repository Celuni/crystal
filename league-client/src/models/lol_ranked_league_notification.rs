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
pub struct LolRankedLeagueNotification {
    #[serde(rename = "changeReason", skip_serializing_if = "Option::is_none")]
    pub change_reason: Option<String>,
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "leaguePoints", skip_serializing_if = "Option::is_none")]
    pub league_points: Option<i32>,
    #[serde(rename = "leaguePointsDelta", skip_serializing_if = "Option::is_none")]
    pub league_points_delta: Option<i32>,
    #[serde(rename = "miniseriesProgress", skip_serializing_if = "Option::is_none")]
    pub miniseries_progress: Option<String>,
    #[serde(rename = "notifyReason", skip_serializing_if = "Option::is_none")]
    pub notify_reason: Option<String>,
    #[serde(rename = "provisionalGamesRemaining", skip_serializing_if = "Option::is_none")]
    pub provisional_games_remaining: Option<i32>,
    #[serde(rename = "queueType", skip_serializing_if = "Option::is_none")]
    pub queue_type: Option<String>,
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    #[serde(rename = "splitPoints", skip_serializing_if = "Option::is_none")]
    pub split_points: Option<i32>,
    #[serde(rename = "splitPointsBreakdown", skip_serializing_if = "Option::is_none")]
    pub split_points_breakdown: Option<::std::collections::HashMap<String, i32>>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

impl LolRankedLeagueNotification {
    pub fn new() -> LolRankedLeagueNotification {
        LolRankedLeagueNotification {
            change_reason: None,
            game_id: None,
            league_points: None,
            league_points_delta: None,
            miniseries_progress: None,
            notify_reason: None,
            provisional_games_remaining: None,
            queue_type: None,
            rank: None,
            split_points: None,
            split_points_breakdown: None,
            tier: None,
        }
    }
}

