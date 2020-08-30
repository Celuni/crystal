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
pub struct MatchmakingLcdsMatchMakerParams {
    #[serde(rename = "botDifficulty", skip_serializing_if = "Option::is_none")]
    pub bot_difficulty: Option<String>,
    #[serde(rename = "invitationId", skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "languages", skip_serializing_if = "Option::is_none")]
    pub languages: Option<String>,
    #[serde(rename = "lastMaestroMessage", skip_serializing_if = "Option::is_none")]
    pub last_maestro_message: Option<String>,
    #[serde(rename = "queueIds", skip_serializing_if = "Option::is_none")]
    pub queue_ids: Option<Vec<i32>>,
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<Vec<i64>>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i64>,
}

impl MatchmakingLcdsMatchMakerParams {
    pub fn new() -> MatchmakingLcdsMatchMakerParams {
        MatchmakingLcdsMatchMakerParams {
            bot_difficulty: None,
            invitation_id: None,
            languages: None,
            last_maestro_message: None,
            queue_ids: None,
            team: None,
            team_id: None,
        }
    }
}

