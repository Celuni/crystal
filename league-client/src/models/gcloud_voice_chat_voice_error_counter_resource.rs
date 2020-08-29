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
pub struct GcloudVoiceChatVoiceErrorCounterResource {
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::models::GcloudVoiceChatVoiceErrorResource>,
    #[serde(rename = "timeOfLastError", skip_serializing_if = "Option::is_none")]
    pub time_of_last_error: Option<i64>,
}

impl GcloudVoiceChatVoiceErrorCounterResource {
    pub fn new() -> GcloudVoiceChatVoiceErrorCounterResource {
        GcloudVoiceChatVoiceErrorCounterResource {
            count: None,
            error: None,
            time_of_last_error: None,
        }
    }
}


