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
pub struct LolShutdownShutdownNotification {
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(rename = "countdown", skip_serializing_if = "Option::is_none")]
    pub countdown: Option<f32>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<crate::models::LolShutdownShutdownReason>,
}

impl LolShutdownShutdownNotification {
    pub fn new() -> LolShutdownShutdownNotification {
        LolShutdownShutdownNotification {
            additional_info: None,
            countdown: None,
            reason: None,
        }
    }
}


