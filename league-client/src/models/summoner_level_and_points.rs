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
pub struct SummonerLevelAndPoints {
    #[serde(rename = "expPoints", skip_serializing_if = "Option::is_none")]
    pub exp_points: Option<i64>,
    #[serde(rename = "expToNextLevel", skip_serializing_if = "Option::is_none")]
    pub exp_to_next_level: Option<i64>,
    #[serde(rename = "sumId", skip_serializing_if = "Option::is_none")]
    pub sum_id: Option<i64>,
    #[serde(rename = "summonerLevel", skip_serializing_if = "Option::is_none")]
    pub summoner_level: Option<i32>,
}

impl SummonerLevelAndPoints {
    pub fn new() -> SummonerLevelAndPoints {
        SummonerLevelAndPoints {
            exp_points: None,
            exp_to_next_level: None,
            sum_id: None,
            summoner_level: None,
        }
    }
}


