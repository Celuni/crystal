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
pub struct LcdsSimpleMessageResponse {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "msgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
}

impl LcdsSimpleMessageResponse {
    pub fn new() -> LcdsSimpleMessageResponse {
        LcdsSimpleMessageResponse {
            account_id: None,
            command: None,
            msg_id: None,
        }
    }
}


