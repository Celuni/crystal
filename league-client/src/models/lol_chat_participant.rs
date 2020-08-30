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
pub struct LolChatParticipant {
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "game_name", skip_serializing_if = "Option::is_none")]
    pub game_name: Option<String>,
    #[serde(rename = "game_tag", skip_serializing_if = "Option::is_none")]
    pub game_tag: Option<String>,
    #[serde(rename = "muted", skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl LolChatParticipant {
    pub fn new() -> LolChatParticipant {
        LolChatParticipant {
            cid: None,
            game_name: None,
            game_tag: None,
            muted: None,
            name: None,
            pid: None,
            puuid: None,
            region: None,
        }
    }
}

