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
pub struct LolLobbyAutoFillQueueDto {
    #[serde(rename = "autoFillEligible", skip_serializing_if = "Option::is_none")]
    pub auto_fill_eligible: Option<bool>,
    #[serde(rename = "autoFillProtectedForPromos", skip_serializing_if = "Option::is_none")]
    pub auto_fill_protected_for_promos: Option<bool>,
    #[serde(rename = "autoFillProtectedForStreaking", skip_serializing_if = "Option::is_none")]
    pub auto_fill_protected_for_streaking: Option<bool>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
}

impl LolLobbyAutoFillQueueDto {
    pub fn new() -> LolLobbyAutoFillQueueDto {
        LolLobbyAutoFillQueueDto {
            auto_fill_eligible: None,
            auto_fill_protected_for_promos: None,
            auto_fill_protected_for_streaking: None,
            queue_id: None,
        }
    }
}

