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
pub struct LolChatMultiGamePresenceUpdate {
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<String>,
    #[serde(rename = "privateJwt", skip_serializing_if = "Option::is_none")]
    pub private_jwt: Option<String>,
    #[serde(rename = "shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<crate::models::LolChatMultiGamePresenceSharedPayload>,
    #[serde(rename = "sharedJwt", skip_serializing_if = "Option::is_none")]
    pub shared_jwt: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::LolChatAccountState>,
}

impl LolChatMultiGamePresenceUpdate {
    pub fn new() -> LolChatMultiGamePresenceUpdate {
        LolChatMultiGamePresenceUpdate {
            msg: None,
            private: None,
            private_jwt: None,
            shared: None,
            shared_jwt: None,
            state: None,
        }
    }
}


