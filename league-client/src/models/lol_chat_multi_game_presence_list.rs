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
pub struct LolChatMultiGamePresenceList {
    #[serde(rename = "presences", skip_serializing_if = "Option::is_none")]
    pub presences: Option<Vec<crate::models::LolChatMultiGamePresence>>,
}

impl LolChatMultiGamePresenceList {
    pub fn new() -> LolChatMultiGamePresenceList {
        LolChatMultiGamePresenceList {
            presences: None,
        }
    }
}


