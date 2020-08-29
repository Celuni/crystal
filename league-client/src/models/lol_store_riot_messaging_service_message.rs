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
pub struct LolStoreRiotMessagingServiceMessage {
    #[serde(rename = "payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl LolStoreRiotMessagingServiceMessage {
    pub fn new() -> LolStoreRiotMessagingServiceMessage {
        LolStoreRiotMessagingServiceMessage {
            payload: None,
            resource: None,
            service: None,
            timestamp: None,
            version: None,
        }
    }
}


