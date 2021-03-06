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
pub struct LolRankedSplitReward {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "pointsRequired", skip_serializing_if = "Option::is_none")]
    pub points_required: Option<i32>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(rename = "regaliaLevel", skip_serializing_if = "Option::is_none")]
    pub regalia_level: Option<i32>,
    #[serde(rename = "rewardType", skip_serializing_if = "Option::is_none")]
    pub reward_type: Option<String>,
    #[serde(rename = "splitId", skip_serializing_if = "Option::is_none")]
    pub split_id: Option<i32>,
}

impl LolRankedSplitReward {
    pub fn new() -> LolRankedSplitReward {
        LolRankedSplitReward {
            description: None,
            id: None,
            points_required: None,
            quantity: None,
            regalia_level: None,
            reward_type: None,
            split_id: None,
        }
    }
}


