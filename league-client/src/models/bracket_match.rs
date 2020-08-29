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
pub struct BracketMatch {
    #[serde(rename = "failRosterStatus", skip_serializing_if = "Option::is_none")]
    pub fail_roster_status: Option<i32>,
    #[serde(rename = "forfeitRosterId", skip_serializing_if = "Option::is_none")]
    pub forfeit_roster_id: Option<i64>,
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "gameStartTime", skip_serializing_if = "Option::is_none")]
    pub game_start_time: Option<i64>,
    #[serde(rename = "highestPossiblePosition", skip_serializing_if = "Option::is_none")]
    pub highest_possible_position: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "loserBracket", skip_serializing_if = "Option::is_none")]
    pub loser_bracket: Option<bool>,
    #[serde(rename = "lowestPossiblePosition", skip_serializing_if = "Option::is_none")]
    pub lowest_possible_position: Option<i32>,
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "resultHistory", skip_serializing_if = "Option::is_none")]
    pub result_history: Option<String>,
    #[serde(rename = "rosterId1", skip_serializing_if = "Option::is_none")]
    pub roster_id1: Option<i64>,
    #[serde(rename = "rosterId2", skip_serializing_if = "Option::is_none")]
    pub roster_id2: Option<i64>,
    #[serde(rename = "round", skip_serializing_if = "Option::is_none")]
    pub round: Option<i32>,
    #[serde(rename = "roundStartTime", skip_serializing_if = "Option::is_none")]
    pub round_start_time: Option<i64>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::ClientBracketMatchStatus>,
    #[serde(rename = "winnerId", skip_serializing_if = "Option::is_none")]
    pub winner_id: Option<i64>,
}

impl BracketMatch {
    pub fn new() -> BracketMatch {
        BracketMatch {
            fail_roster_status: None,
            forfeit_roster_id: None,
            game_id: None,
            game_start_time: None,
            highest_possible_position: None,
            id: None,
            loser_bracket: None,
            lowest_possible_position: None,
            order: None,
            result_history: None,
            roster_id1: None,
            roster_id2: None,
            round: None,
            round_start_time: None,
            status: None,
            winner_id: None,
        }
    }
}


