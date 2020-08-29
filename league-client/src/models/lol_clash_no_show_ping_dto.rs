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
pub struct LolClashNoShowPingDto {
    #[serde(rename = "dodgeTime", skip_serializing_if = "Option::is_none")]
    pub dodge_time: Option<i64>,
    #[serde(rename = "matchId", skip_serializing_if = "Option::is_none")]
    pub match_id: Option<i64>,
    #[serde(rename = "tournamentId", skip_serializing_if = "Option::is_none")]
    pub tournament_id: Option<i64>,
}

impl LolClashNoShowPingDto {
    pub fn new() -> LolClashNoShowPingDto {
        LolClashNoShowPingDto {
            dodge_time: None,
            match_id: None,
            tournament_id: None,
        }
    }
}


