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
pub struct LolStatstonesStatstoneNotificationDto {
    #[serde(rename = "best", skip_serializing_if = "Option::is_none")]
    pub best: Option<i32>,
    #[serde(rename = "delta", skip_serializing_if = "Option::is_none")]
    pub delta: Option<i32>,
    #[serde(rename = "isNewBest", skip_serializing_if = "Option::is_none")]
    pub is_new_best: Option<bool>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "statstoneId", skip_serializing_if = "Option::is_none")]
    pub statstone_id: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

impl LolStatstonesStatstoneNotificationDto {
    pub fn new() -> LolStatstonesStatstoneNotificationDto {
        LolStatstonesStatstoneNotificationDto {
            best: None,
            delta: None,
            is_new_best: None,
            level: None,
            puuid: None,
            statstone_id: None,
            value: None,
        }
    }
}


