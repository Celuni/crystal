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
pub struct LolMatchHistoryMatchHistoryTeamBan {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "pickTurn", skip_serializing_if = "Option::is_none")]
    pub pick_turn: Option<i32>,
}

impl LolMatchHistoryMatchHistoryTeamBan {
    pub fn new() -> LolMatchHistoryMatchHistoryTeamBan {
        LolMatchHistoryMatchHistoryTeamBan {
            champion_id: None,
            pick_turn: None,
        }
    }
}


