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
pub struct PhaseRosterDto {
    #[serde(rename = "bracketDTO", skip_serializing_if = "Option::is_none")]
    pub bracket_dto: Option<crate::models::Bracket>,
    #[serde(rename = "bracketId", skip_serializing_if = "Option::is_none")]
    pub bracket_id: Option<i64>,
    #[serde(rename = "checkinTime", skip_serializing_if = "Option::is_none")]
    pub checkin_time: Option<i64>,
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "phaseId", skip_serializing_if = "Option::is_none")]
    pub phase_id: Option<i64>,
    #[serde(rename = "phaseRosterSubs", skip_serializing_if = "Option::is_none")]
    pub phase_roster_subs: Option<Vec<crate::models::PhaseRosterSubDto>>,
}

impl PhaseRosterDto {
    pub fn new() -> PhaseRosterDto {
        PhaseRosterDto {
            bracket_dto: None,
            bracket_id: None,
            checkin_time: None,
            period: None,
            phase_id: None,
            phase_roster_subs: None,
        }
    }
}


