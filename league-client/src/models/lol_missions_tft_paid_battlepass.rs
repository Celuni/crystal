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
pub struct LolMissionsTftPaidBattlepass {
    #[serde(rename = "activeMilestone", skip_serializing_if = "Option::is_none")]
    pub active_milestone: Option<crate::models::LolMissionsTftPaidBattlepassMilestone>,
    #[serde(rename = "bonuses", skip_serializing_if = "Option::is_none")]
    pub bonuses: Option<Vec<crate::models::LolMissionsTftPaidBattlepassMilestone>>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::models::LolMissionsTftPaidBattlepassInfo>,
    #[serde(rename = "milestones", skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<crate::models::LolMissionsTftPaidBattlepassMilestone>>,
    #[serde(rename = "totalPointsEarned", skip_serializing_if = "Option::is_none")]
    pub total_points_earned: Option<i32>,
}

impl LolMissionsTftPaidBattlepass {
    pub fn new() -> LolMissionsTftPaidBattlepass {
        LolMissionsTftPaidBattlepass {
            active_milestone: None,
            bonuses: None,
            info: None,
            milestones: None,
            total_points_earned: None,
        }
    }
}


