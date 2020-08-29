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
pub struct LolAccountVerificationInvalidateResponse {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "smsTokenExpireDurationInSec", skip_serializing_if = "Option::is_none")]
    pub sms_token_expire_duration_in_sec: Option<i32>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl LolAccountVerificationInvalidateResponse {
    pub fn new() -> LolAccountVerificationInvalidateResponse {
        LolAccountVerificationInvalidateResponse {
            message: None,
            sms_token_expire_duration_in_sec: None,
            status: None,
            success: None,
        }
    }
}


