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
pub struct PhaseInMember {
    #[serde(rename = "bet", skip_serializing_if = "Option::is_none")]
    pub bet: Option<i32>,
    #[serde(rename = "pendingPay", skip_serializing_if = "Option::is_none")]
    pub pending_pay: Option<i32>,
    #[serde(rename = "playerId", skip_serializing_if = "Option::is_none")]
    pub player_id: Option<i64>,
    #[serde(rename = "selfBet", skip_serializing_if = "Option::is_none")]
    pub self_bet: Option<i32>,
}

impl PhaseInMember {
    pub fn new() -> PhaseInMember {
        PhaseInMember {
            bet: None,
            pending_pay: None,
            player_id: None,
            self_bet: None,
        }
    }
}


