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
pub struct RosterDto {
    #[serde(rename = "banned", skip_serializing_if = "Option::is_none")]
    pub banned: Option<bool>,
    #[serde(rename = "captainId", skip_serializing_if = "Option::is_none")]
    pub captain_id: Option<i64>,
    #[serde(rename = "dynamicState", skip_serializing_if = "Option::is_none")]
    pub dynamic_state: Option<crate::models::RosterDynamicStateDto>,
    #[serde(rename = "eliminated", skip_serializing_if = "Option::is_none")]
    pub eliminated: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "invitationId", skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<i32>,
    #[serde(rename = "logoColor", skip_serializing_if = "Option::is_none")]
    pub logo_color: Option<i32>,
    #[serde(rename = "losses", skip_serializing_if = "Option::is_none")]
    pub losses: Option<i32>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::RosterMemberDto>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "phases", skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<crate::models::PhaseRosterDto>>,
    #[serde(rename = "points", skip_serializing_if = "Option::is_none")]
    pub points: Option<i32>,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<i32>,
    #[serde(rename = "tournamentId", skip_serializing_if = "Option::is_none")]
    pub tournament_id: Option<i64>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "wins", skip_serializing_if = "Option::is_none")]
    pub wins: Option<i32>,
}

impl RosterDto {
    pub fn new() -> RosterDto {
        RosterDto {
            banned: None,
            captain_id: None,
            dynamic_state: None,
            eliminated: None,
            id: None,
            invitation_id: None,
            logo: None,
            logo_color: None,
            losses: None,
            members: None,
            name: None,
            phases: None,
            points: None,
            short_name: None,
            tier: None,
            tournament_id: None,
            version: None,
            wins: None,
        }
    }
}


