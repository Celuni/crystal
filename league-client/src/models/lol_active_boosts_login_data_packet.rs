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
pub struct LolActiveBoostsLoginDataPacket {
    #[serde(rename = "timeUntilFirstWinOfDay", skip_serializing_if = "Option::is_none")]
    pub time_until_first_win_of_day: Option<i64>,
}

impl LolActiveBoostsLoginDataPacket {
    pub fn new() -> LolActiveBoostsLoginDataPacket {
        LolActiveBoostsLoginDataPacket {
            time_until_first_win_of_day: None,
        }
    }
}


