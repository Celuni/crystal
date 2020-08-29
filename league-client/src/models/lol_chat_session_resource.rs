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
pub struct LolChatSessionResource {
    #[serde(rename = "sessionExpire", skip_serializing_if = "Option::is_none")]
    pub session_expire: Option<i32>,
    #[serde(rename = "sessionState", skip_serializing_if = "Option::is_none")]
    pub session_state: Option<crate::models::LolChatSessionState>,
}

impl LolChatSessionResource {
    pub fn new() -> LolChatSessionResource {
        LolChatSessionResource {
            session_expire: None,
            session_state: None,
        }
    }
}


