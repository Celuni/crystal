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
pub struct LolLobbyTeamBuilderBackwardsTransitionInfoV1 {
    #[serde(rename = "backwardsTransitionReason", skip_serializing_if = "Option::is_none")]
    pub backwards_transition_reason: Option<String>,
    #[serde(rename = "initiatorSummonerIds", skip_serializing_if = "Option::is_none")]
    pub initiator_summoner_ids: Option<Vec<i64>>,
}

impl LolLobbyTeamBuilderBackwardsTransitionInfoV1 {
    pub fn new() -> LolLobbyTeamBuilderBackwardsTransitionInfoV1 {
        LolLobbyTeamBuilderBackwardsTransitionInfoV1 {
            backwards_transition_reason: None,
            initiator_summoner_ids: None,
        }
    }
}


