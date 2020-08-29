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
pub struct LolNpeRewardsChallengesSettings {
    #[serde(rename = "allMissionsCompleted", skip_serializing_if = "Option::is_none")]
    pub all_missions_completed: Option<bool>,
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
}

impl LolNpeRewardsChallengesSettings {
    pub fn new() -> LolNpeRewardsChallengesSettings {
        LolNpeRewardsChallengesSettings {
            all_missions_completed: None,
            total_count: None,
        }
    }
}


