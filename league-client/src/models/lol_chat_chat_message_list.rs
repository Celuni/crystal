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
pub struct LolChatChatMessageList {
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::models::LolChatChatMessage>>,
}

impl LolChatChatMessageList {
    pub fn new() -> LolChatChatMessageList {
        LolChatChatMessageList {
            messages: None,
        }
    }
}


