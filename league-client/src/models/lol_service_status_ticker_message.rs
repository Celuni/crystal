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
pub struct LolServiceStatusTickerMessage {
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "heading", skip_serializing_if = "Option::is_none")]
    pub heading: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl LolServiceStatusTickerMessage {
    pub fn new() -> LolServiceStatusTickerMessage {
        LolServiceStatusTickerMessage {
            created_at: None,
            heading: None,
            message: None,
            severity: None,
            updated_at: None,
        }
    }
}


