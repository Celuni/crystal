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
pub struct GcloudVoiceChatParticipantResource {
    #[serde(rename = "energy", skip_serializing_if = "Option::is_none")]
    pub energy: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isMuted", skip_serializing_if = "Option::is_none")]
    pub is_muted: Option<bool>,
    #[serde(rename = "isSpeaking", skip_serializing_if = "Option::is_none")]
    pub is_speaking: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "speakingTimestamp", skip_serializing_if = "Option::is_none")]
    pub speaking_timestamp: Option<i32>,
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<i32>,
}

impl GcloudVoiceChatParticipantResource {
    pub fn new() -> GcloudVoiceChatParticipantResource {
        GcloudVoiceChatParticipantResource {
            energy: None,
            id: None,
            is_muted: None,
            is_speaking: None,
            name: None,
            speaking_timestamp: None,
            uri: None,
            volume: None,
        }
    }
}


