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
pub struct LolActiveBoostsEndOfGameStats {
    #[serde(rename = "timeUntilNextFirstWinBonus", skip_serializing_if = "Option::is_none")]
    pub time_until_next_first_win_bonus: Option<i64>,
}

impl LolActiveBoostsEndOfGameStats {
    pub fn new() -> LolActiveBoostsEndOfGameStats {
        LolActiveBoostsEndOfGameStats {
            time_until_next_first_win_bonus: None,
        }
    }
}


