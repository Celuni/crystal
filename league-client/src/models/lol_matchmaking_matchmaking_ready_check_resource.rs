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
pub struct LolMatchmakingMatchmakingReadyCheckResource {
    #[serde(rename = "declinerIds", skip_serializing_if = "Option::is_none")]
    pub decliner_ids: Option<Vec<i64>>,
    #[serde(rename = "dodgeWarning", skip_serializing_if = "Option::is_none")]
    pub dodge_warning: Option<crate::models::LolMatchmakingMatchmakingDodgeWarning>,
    #[serde(rename = "playerResponse", skip_serializing_if = "Option::is_none")]
    pub player_response: Option<crate::models::LolMatchmakingMatchmakingReadyCheckResponse>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::models::LolMatchmakingMatchmakingReadyCheckState>,
    #[serde(rename = "suppressUx", skip_serializing_if = "Option::is_none")]
    pub suppress_ux: Option<bool>,
    #[serde(rename = "timer", skip_serializing_if = "Option::is_none")]
    pub timer: Option<f32>,
}

impl LolMatchmakingMatchmakingReadyCheckResource {
    pub fn new() -> LolMatchmakingMatchmakingReadyCheckResource {
        LolMatchmakingMatchmakingReadyCheckResource {
            decliner_ids: None,
            dodge_warning: None,
            player_response: None,
            state: None,
            suppress_ux: None,
            timer: None,
        }
    }
}

