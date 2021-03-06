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
pub struct GameflowLcdsGameDto {
    #[serde(rename = "gameMode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<String>,
    #[serde(rename = "gameQueueConfigId", skip_serializing_if = "Option::is_none")]
    pub game_queue_config_id: Option<i32>,
    #[serde(rename = "gameState", skip_serializing_if = "Option::is_none")]
    pub game_state: Option<String>,
    #[serde(rename = "gameType", skip_serializing_if = "Option::is_none")]
    pub game_type: Option<String>,
    #[serde(rename = "gameTypeConfigId", skip_serializing_if = "Option::is_none")]
    pub game_type_config_id: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "mapId", skip_serializing_if = "Option::is_none")]
    pub map_id: Option<i32>,
    #[serde(rename = "maxNumPlayers", skip_serializing_if = "Option::is_none")]
    pub max_num_players: Option<i32>,
    #[serde(rename = "playerChampionSelections", skip_serializing_if = "Option::is_none")]
    pub player_champion_selections: Option<Vec<serde_json::Value>>,
    #[serde(rename = "queueTypeName", skip_serializing_if = "Option::is_none")]
    pub queue_type_name: Option<String>,
    #[serde(rename = "spectatorDelay", skip_serializing_if = "Option::is_none")]
    pub spectator_delay: Option<i32>,
    #[serde(rename = "teamOne", skip_serializing_if = "Option::is_none")]
    pub team_one: Option<Vec<serde_json::Value>>,
    #[serde(rename = "teamTwo", skip_serializing_if = "Option::is_none")]
    pub team_two: Option<Vec<serde_json::Value>>,
}

impl GameflowLcdsGameDto {
    pub fn new() -> GameflowLcdsGameDto {
        GameflowLcdsGameDto {
            game_mode: None,
            game_queue_config_id: None,
            game_state: None,
            game_type: None,
            game_type_config_id: None,
            id: None,
            map_id: None,
            max_num_players: None,
            player_champion_selections: None,
            queue_type_name: None,
            spectator_delay: None,
            team_one: None,
            team_two: None,
        }
    }
}


