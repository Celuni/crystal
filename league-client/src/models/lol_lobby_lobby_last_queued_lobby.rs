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
pub struct LolLobbyLobbyLastQueuedLobby {
    #[serde(rename = "canPlayAgain", skip_serializing_if = "Option::is_none")]
    pub can_play_again: Option<bool>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::LolLobbyLobbyLastQueuedMember>>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[serde(rename = "wasOwner", skip_serializing_if = "Option::is_none")]
    pub was_owner: Option<bool>,
}

impl LolLobbyLobbyLastQueuedLobby {
    pub fn new() -> LolLobbyLobbyLastQueuedLobby {
        LolLobbyLobbyLastQueuedLobby {
            can_play_again: None,
            members: None,
            queue_id: None,
            was_owner: None,
        }
    }
}


