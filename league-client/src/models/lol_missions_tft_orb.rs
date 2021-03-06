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
pub struct LolMissionsTftOrb {
    #[serde(rename = "missionId", skip_serializing_if = "Option::is_none")]
    pub mission_id: Option<String>,
    #[serde(rename = "rewardLevel", skip_serializing_if = "Option::is_none")]
    pub reward_level: Option<i32>,
    #[serde(rename = "rewards", skip_serializing_if = "Option::is_none")]
    pub rewards: Option<Vec<crate::models::PlayerMissionRewardDto>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "unlockTime", skip_serializing_if = "Option::is_none")]
    pub unlock_time: Option<i64>,
}

impl LolMissionsTftOrb {
    pub fn new() -> LolMissionsTftOrb {
        LolMissionsTftOrb {
            mission_id: None,
            reward_level: None,
            rewards: None,
            status: None,
            unlock_time: None,
        }
    }
}


