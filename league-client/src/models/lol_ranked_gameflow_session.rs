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
pub struct LolRankedGameflowSession {
    #[serde(rename = "gameData", skip_serializing_if = "Option::is_none")]
    pub game_data: Option<crate::models::LolRankedGameflowGameData>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<crate::models::LolRankedGameflowPhase>,
}

impl LolRankedGameflowSession {
    pub fn new() -> LolRankedGameflowSession {
        LolRankedGameflowSession {
            game_data: None,
            phase: None,
        }
    }
}


