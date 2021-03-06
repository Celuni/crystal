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
pub struct LolChatBlockedPlayerResource {
    #[serde(rename = "gameName", skip_serializing_if = "Option::is_none")]
    pub game_name: Option<String>,
    #[serde(rename = "gameTag", skip_serializing_if = "Option::is_none")]
    pub game_tag: Option<String>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolChatBlockedPlayerResource {
    pub fn new() -> LolChatBlockedPlayerResource {
        LolChatBlockedPlayerResource {
            game_name: None,
            game_tag: None,
            icon: None,
            id: None,
            name: None,
            pid: None,
            puuid: None,
            summoner_id: None,
        }
    }
}


