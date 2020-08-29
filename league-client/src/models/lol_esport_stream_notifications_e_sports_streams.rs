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
pub struct LolEsportStreamNotificationsESportsStreams {
    #[serde(rename = "teamAAcronym", skip_serializing_if = "Option::is_none")]
    pub team_a_acronym: Option<String>,
    #[serde(rename = "teamAGuid", skip_serializing_if = "Option::is_none")]
    pub team_a_guid: Option<String>,
    #[serde(rename = "teamAId", skip_serializing_if = "Option::is_none")]
    pub team_a_id: Option<i64>,
    #[serde(rename = "teamALogoUrl", skip_serializing_if = "Option::is_none")]
    pub team_a_logo_url: Option<String>,
    #[serde(rename = "teamAName", skip_serializing_if = "Option::is_none")]
    pub team_a_name: Option<String>,
    #[serde(rename = "teamBAcronym", skip_serializing_if = "Option::is_none")]
    pub team_b_acronym: Option<String>,
    #[serde(rename = "teamBGuid", skip_serializing_if = "Option::is_none")]
    pub team_b_guid: Option<String>,
    #[serde(rename = "teamBId", skip_serializing_if = "Option::is_none")]
    pub team_b_id: Option<i64>,
    #[serde(rename = "teamBLogoUrl", skip_serializing_if = "Option::is_none")]
    pub team_b_logo_url: Option<String>,
    #[serde(rename = "teamBName", skip_serializing_if = "Option::is_none")]
    pub team_b_name: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tournamentDescription", skip_serializing_if = "Option::is_none")]
    pub tournament_description: Option<String>,
}

impl LolEsportStreamNotificationsESportsStreams {
    pub fn new() -> LolEsportStreamNotificationsESportsStreams {
        LolEsportStreamNotificationsESportsStreams {
            team_a_acronym: None,
            team_a_guid: None,
            team_a_id: None,
            team_a_logo_url: None,
            team_a_name: None,
            team_b_acronym: None,
            team_b_guid: None,
            team_b_id: None,
            team_b_logo_url: None,
            team_b_name: None,
            title: None,
            tournament_description: None,
        }
    }
}


