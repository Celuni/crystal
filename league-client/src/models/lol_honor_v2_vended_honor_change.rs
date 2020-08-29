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
pub struct LolHonorV2VendedHonorChange {
    #[serde(rename = "actionType", skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "currentState", skip_serializing_if = "Option::is_none")]
    pub current_state: Option<crate::models::LolHonorV2VendedHonorState>,
    #[serde(rename = "dynamicHonorMessage", skip_serializing_if = "Option::is_none")]
    pub dynamic_honor_message: Option<crate::models::LolHonorV2DynamicHonorMessage>,
    #[serde(rename = "previousState", skip_serializing_if = "Option::is_none")]
    pub previous_state: Option<crate::models::LolHonorV2VendedHonorState>,
    #[serde(rename = "reward", skip_serializing_if = "Option::is_none")]
    pub reward: Option<crate::models::LolHonorV2Reward>,
}

impl LolHonorV2VendedHonorChange {
    pub fn new() -> LolHonorV2VendedHonorChange {
        LolHonorV2VendedHonorChange {
            action_type: None,
            current_state: None,
            dynamic_honor_message: None,
            previous_state: None,
            reward: None,
        }
    }
}


