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
pub struct LolHonorV2ProfileInfo {
    #[serde(rename = "checkpoint", skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<i32>,
    #[serde(rename = "honorLevel", skip_serializing_if = "Option::is_none")]
    pub honor_level: Option<i32>,
    #[serde(rename = "rewardsLocked", skip_serializing_if = "Option::is_none")]
    pub rewards_locked: Option<bool>,
}

impl LolHonorV2ProfileInfo {
    pub fn new() -> LolHonorV2ProfileInfo {
        LolHonorV2ProfileInfo {
            checkpoint: None,
            honor_level: None,
            rewards_locked: None,
        }
    }
}


