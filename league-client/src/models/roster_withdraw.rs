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
pub struct RosterWithdraw {
    #[serde(rename = "declineWithdrawMembers", skip_serializing_if = "Option::is_none")]
    pub decline_withdraw_members: Option<Vec<i64>>,
    #[serde(rename = "gameStartBufferMs", skip_serializing_if = "Option::is_none")]
    pub game_start_buffer_ms: Option<i64>,
    #[serde(rename = "initVoteMember", skip_serializing_if = "Option::is_none")]
    pub init_vote_member: Option<i64>,
    #[serde(rename = "initVoteTime", skip_serializing_if = "Option::is_none")]
    pub init_vote_time: Option<i64>,
    #[serde(rename = "lockoutTimeMs", skip_serializing_if = "Option::is_none")]
    pub lockout_time_ms: Option<i64>,
    #[serde(rename = "voteTimeoutMs", skip_serializing_if = "Option::is_none")]
    pub vote_timeout_ms: Option<i64>,
    #[serde(rename = "voteWithdrawMembers", skip_serializing_if = "Option::is_none")]
    pub vote_withdraw_members: Option<Vec<i64>>,
}

impl RosterWithdraw {
    pub fn new() -> RosterWithdraw {
        RosterWithdraw {
            decline_withdraw_members: None,
            game_start_buffer_ms: None,
            init_vote_member: None,
            init_vote_time: None,
            lockout_time_ms: None,
            vote_timeout_ms: None,
            vote_withdraw_members: None,
        }
    }
}


