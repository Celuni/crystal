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
pub struct LolNpeRewardsAllRewards {
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<crate::models::LolNpeRewardsRewardSeries>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<crate::models::LolNpeRewardsRewardSeries>,
}

impl LolNpeRewardsAllRewards {
    pub fn new() -> LolNpeRewardsAllRewards {
        LolNpeRewardsAllRewards {
            level: None,
            login: None,
        }
    }
}


