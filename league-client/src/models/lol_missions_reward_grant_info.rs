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
pub struct LolMissionsRewardGrantInfo {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "parentGrantId", skip_serializing_if = "Option::is_none")]
    pub parent_grant_id: Option<String>,
    #[serde(rename = "recipientId", skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    #[serde(rename = "recipientTypeId", skip_serializing_if = "Option::is_none")]
    pub recipient_type_id: Option<String>,
    #[serde(rename = "rewardGroupId", skip_serializing_if = "Option::is_none")]
    pub reward_group_id: Option<String>,
    #[serde(rename = "rewardStatuses", skip_serializing_if = "Option::is_none")]
    pub reward_statuses: Option<::std::collections::HashMap<String, crate::models::LolMissionsGrantStatus>>,
    #[serde(rename = "selectedIds", skip_serializing_if = "Option::is_none")]
    pub selected_ids: Option<Vec<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::LolMissionsGrantStatus>,
    #[serde(rename = "viewed", skip_serializing_if = "Option::is_none")]
    pub viewed: Option<bool>,
}

impl LolMissionsRewardGrantInfo {
    pub fn new() -> LolMissionsRewardGrantInfo {
        LolMissionsRewardGrantInfo {
            id: None,
            parent_grant_id: None,
            recipient_id: None,
            recipient_type_id: None,
            reward_group_id: None,
            reward_statuses: None,
            selected_ids: None,
            status: None,
            viewed: None,
        }
    }
}

