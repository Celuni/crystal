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
pub struct RankedScoutingTopChampionDto {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "gameCount", skip_serializing_if = "Option::is_none")]
    pub game_count: Option<i32>,
    #[serde(rename = "kda", skip_serializing_if = "Option::is_none")]
    pub kda: Option<f32>,
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    #[serde(rename = "winCount", skip_serializing_if = "Option::is_none")]
    pub win_count: Option<i32>,
}

impl RankedScoutingTopChampionDto {
    pub fn new() -> RankedScoutingTopChampionDto {
        RankedScoutingTopChampionDto {
            champion_id: None,
            game_count: None,
            kda: None,
            rank: None,
            win_count: None,
        }
    }
}


