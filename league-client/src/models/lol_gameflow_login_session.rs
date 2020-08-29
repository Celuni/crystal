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
pub struct LolGameflowLoginSession {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "connected", skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::LolGameflowLoginSessionStates>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolGameflowLoginSession {
    pub fn new() -> LolGameflowLoginSession {
        LolGameflowLoginSession {
            account_id: None,
            connected: None,
            state: None,
            summoner_id: None,
        }
    }
}


