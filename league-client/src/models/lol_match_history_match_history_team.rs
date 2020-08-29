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
pub struct LolMatchHistoryMatchHistoryTeam {
    #[serde(rename = "bans", skip_serializing_if = "Option::is_none")]
    pub bans: Option<Vec<crate::models::LolMatchHistoryMatchHistoryTeamBan>>,
    #[serde(rename = "baronKills", skip_serializing_if = "Option::is_none")]
    pub baron_kills: Option<i32>,
    #[serde(rename = "dominionVictoryScore", skip_serializing_if = "Option::is_none")]
    pub dominion_victory_score: Option<i32>,
    #[serde(rename = "dragonKills", skip_serializing_if = "Option::is_none")]
    pub dragon_kills: Option<i32>,
    #[serde(rename = "firstBaron", skip_serializing_if = "Option::is_none")]
    pub first_baron: Option<bool>,
    #[serde(rename = "firstBlood", skip_serializing_if = "Option::is_none")]
    pub first_blood: Option<bool>,
    #[serde(rename = "firstDargon", skip_serializing_if = "Option::is_none")]
    pub first_dargon: Option<bool>,
    #[serde(rename = "firstInhibitor", skip_serializing_if = "Option::is_none")]
    pub first_inhibitor: Option<bool>,
    #[serde(rename = "firstTower", skip_serializing_if = "Option::is_none")]
    pub first_tower: Option<bool>,
    #[serde(rename = "inhibitorKills", skip_serializing_if = "Option::is_none")]
    pub inhibitor_kills: Option<i32>,
    #[serde(rename = "riftHeraldKills", skip_serializing_if = "Option::is_none")]
    pub rift_herald_kills: Option<i32>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i32>,
    #[serde(rename = "towerKills", skip_serializing_if = "Option::is_none")]
    pub tower_kills: Option<i32>,
    #[serde(rename = "vilemawKills", skip_serializing_if = "Option::is_none")]
    pub vilemaw_kills: Option<i32>,
    #[serde(rename = "win", skip_serializing_if = "Option::is_none")]
    pub win: Option<String>,
}

impl LolMatchHistoryMatchHistoryTeam {
    pub fn new() -> LolMatchHistoryMatchHistoryTeam {
        LolMatchHistoryMatchHistoryTeam {
            bans: None,
            baron_kills: None,
            dominion_victory_score: None,
            dragon_kills: None,
            first_baron: None,
            first_blood: None,
            first_dargon: None,
            first_inhibitor: None,
            first_tower: None,
            inhibitor_kills: None,
            rift_herald_kills: None,
            team_id: None,
            tower_kills: None,
            vilemaw_kills: None,
            win: None,
        }
    }
}


