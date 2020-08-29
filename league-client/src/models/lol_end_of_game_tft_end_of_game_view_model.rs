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
pub struct LolEndOfGameTftEndOfGameViewModel {
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "gameLength", skip_serializing_if = "Option::is_none")]
    pub game_length: Option<i32>,
    #[serde(rename = "isRanked", skip_serializing_if = "Option::is_none")]
    pub is_ranked: Option<bool>,
    #[serde(rename = "localPlayer", skip_serializing_if = "Option::is_none")]
    pub local_player: Option<crate::models::LolEndOfGameTftEndOfGamePlayerViewModel>,
    #[serde(rename = "players", skip_serializing_if = "Option::is_none")]
    pub players: Option<Vec<crate::models::LolEndOfGameTftEndOfGamePlayerViewModel>>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
}

impl LolEndOfGameTftEndOfGameViewModel {
    pub fn new() -> LolEndOfGameTftEndOfGameViewModel {
        LolEndOfGameTftEndOfGameViewModel {
            game_id: None,
            game_length: None,
            is_ranked: None,
            local_player: None,
            players: None,
            queue_id: None,
        }
    }
}


