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
pub struct LolEndOfGameGameClientEndOfGame {
    #[serde(rename = "gameClientEOG", skip_serializing_if = "Option::is_none")]
    pub game_client_eog: Option<crate::models::LolEndOfGameGameClientEndOfGameStats>,
}

impl LolEndOfGameGameClientEndOfGame {
    pub fn new() -> LolEndOfGameGameClientEndOfGame {
        LolEndOfGameGameClientEndOfGame {
            game_client_eog: None,
        }
    }
}


