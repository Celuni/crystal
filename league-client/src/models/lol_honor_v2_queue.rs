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
pub struct LolHonorV2Queue {
    #[serde(rename = "gameMode", skip_serializing_if = "Option::is_none")]
    pub game_mode: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "mapId", skip_serializing_if = "Option::is_none")]
    pub map_id: Option<i32>,
    #[serde(rename = "removalFromGameAllowed", skip_serializing_if = "Option::is_none")]
    pub removal_from_game_allowed: Option<bool>,
    #[serde(rename = "removalFromGameDelayMinutes", skip_serializing_if = "Option::is_none")]
    pub removal_from_game_delay_minutes: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl LolHonorV2Queue {
    pub fn new() -> LolHonorV2Queue {
        LolHonorV2Queue {
            game_mode: None,
            id: None,
            map_id: None,
            removal_from_game_allowed: None,
            removal_from_game_delay_minutes: None,
            _type: None,
        }
    }
}


