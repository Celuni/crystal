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
pub struct LolChatSpectateGameInfoResource {
    #[serde(rename = "allowObserveMode", skip_serializing_if = "Option::is_none")]
    pub allow_observe_mode: Option<String>,
    #[serde(rename = "dropInSpectateGameId", skip_serializing_if = "Option::is_none")]
    pub drop_in_spectate_game_id: Option<String>,
    #[serde(rename = "gameQueueType", skip_serializing_if = "Option::is_none")]
    pub game_queue_type: Option<String>,
    #[serde(rename = "puuid", skip_serializing_if = "Option::is_none")]
    pub puuid: Option<String>,
}

impl LolChatSpectateGameInfoResource {
    pub fn new() -> LolChatSpectateGameInfoResource {
        LolChatSpectateGameInfoResource {
            allow_observe_mode: None,
            drop_in_spectate_game_id: None,
            game_queue_type: None,
            puuid: None,
        }
    }
}


