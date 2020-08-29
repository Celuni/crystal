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
pub struct LolChatConversationResource {
    #[serde(rename = "gameName", skip_serializing_if = "Option::is_none")]
    pub game_name: Option<String>,
    #[serde(rename = "gameTag", skip_serializing_if = "Option::is_none")]
    pub game_tag: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inviterId", skip_serializing_if = "Option::is_none")]
    pub inviter_id: Option<String>,
    #[serde(rename = "isMuted", skip_serializing_if = "Option::is_none")]
    pub is_muted: Option<bool>,
    #[serde(rename = "lastMessage", skip_serializing_if = "Option::is_none")]
    pub last_message: Option<crate::models::LolChatConversationMessageResource>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[serde(rename = "targetRegion", skip_serializing_if = "Option::is_none")]
    pub target_region: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "unreadMessageCount", skip_serializing_if = "Option::is_none")]
    pub unread_message_count: Option<i64>,
}

impl LolChatConversationResource {
    pub fn new() -> LolChatConversationResource {
        LolChatConversationResource {
            game_name: None,
            game_tag: None,
            id: None,
            inviter_id: None,
            is_muted: None,
            last_message: None,
            name: None,
            password: None,
            pid: None,
            target_region: None,
            _type: None,
            unread_message_count: None,
        }
    }
}


