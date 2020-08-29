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
pub struct PlayerMissionRewardDto {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "isObjectiveBasedReward", skip_serializing_if = "Option::is_none")]
    pub is_objective_based_reward: Option<bool>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(rename = "media", skip_serializing_if = "Option::is_none")]
    pub media: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(rename = "rewardFulfilled", skip_serializing_if = "Option::is_none")]
    pub reward_fulfilled: Option<bool>,
    #[serde(rename = "rewardGroup", skip_serializing_if = "Option::is_none")]
    pub reward_group: Option<String>,
    #[serde(rename = "rewardGroupSelected", skip_serializing_if = "Option::is_none")]
    pub reward_group_selected: Option<bool>,
    #[serde(rename = "rewardType", skip_serializing_if = "Option::is_none")]
    pub reward_type: Option<String>,
    #[serde(rename = "sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
    #[serde(rename = "smallIconUrl", skip_serializing_if = "Option::is_none")]
    pub small_icon_url: Option<String>,
    #[serde(rename = "uniqueName", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
}

impl PlayerMissionRewardDto {
    pub fn new() -> PlayerMissionRewardDto {
        PlayerMissionRewardDto {
            description: None,
            icon_url: None,
            is_objective_based_reward: None,
            item_id: None,
            media: None,
            quantity: None,
            reward_fulfilled: None,
            reward_group: None,
            reward_group_selected: None,
            reward_type: None,
            sequence: None,
            small_icon_url: None,
            unique_name: None,
        }
    }
}


