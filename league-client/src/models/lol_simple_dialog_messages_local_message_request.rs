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
pub struct LolSimpleDialogMessagesLocalMessageRequest {
    #[serde(rename = "msgBody", skip_serializing_if = "Option::is_none")]
    pub msg_body: Option<Vec<String>>,
    #[serde(rename = "msgType", skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
}

impl LolSimpleDialogMessagesLocalMessageRequest {
    pub fn new() -> LolSimpleDialogMessagesLocalMessageRequest {
        LolSimpleDialogMessagesLocalMessageRequest {
            msg_body: None,
            msg_type: None,
        }
    }
}


