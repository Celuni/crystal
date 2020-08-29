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
pub struct LolSummonerSummonerRerollPoints {
    #[serde(rename = "currentPoints", skip_serializing_if = "Option::is_none")]
    pub current_points: Option<i32>,
    #[serde(rename = "maxRolls", skip_serializing_if = "Option::is_none")]
    pub max_rolls: Option<i32>,
    #[serde(rename = "numberOfRolls", skip_serializing_if = "Option::is_none")]
    pub number_of_rolls: Option<i32>,
    #[serde(rename = "pointsCostToRoll", skip_serializing_if = "Option::is_none")]
    pub points_cost_to_roll: Option<i32>,
    #[serde(rename = "pointsToReroll", skip_serializing_if = "Option::is_none")]
    pub points_to_reroll: Option<i32>,
}

impl LolSummonerSummonerRerollPoints {
    pub fn new() -> LolSummonerSummonerRerollPoints {
        LolSummonerSummonerRerollPoints {
            current_points: None,
            max_rolls: None,
            number_of_rolls: None,
            points_cost_to_roll: None,
            points_to_reroll: None,
        }
    }
}


