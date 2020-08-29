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
pub struct LolChatMessage {
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "cid", skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(rename = "game_name", skip_serializing_if = "Option::is_none")]
    pub game_name: Option<String>,
    #[serde(rename = "game_tag", skip_serializing_if = "Option::is_none")]
    pub game_tag: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "mid", skip_serializing_if = "Option::is_none")]
    pub mid: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[serde(rename = "read", skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl LolChatMessage {
    pub fn new() -> LolChatMessage {
        LolChatMessage {
            body: None,
            cid: None,
            game_name: None,
            game_tag: None,
            id: None,
            mid: None,
            name: None,
            pid: None,
            read: None,
            time: None,
            _type: None,
        }
    }
}


