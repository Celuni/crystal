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
pub struct ChampSelectLcdsTradeContractDto {
    #[serde(rename = "requesterChampionId", skip_serializing_if = "Option::is_none")]
    pub requester_champion_id: Option<i32>,
    #[serde(rename = "requesterInternalSummonerName", skip_serializing_if = "Option::is_none")]
    pub requester_internal_summoner_name: Option<String>,
    #[serde(rename = "responderChampionId", skip_serializing_if = "Option::is_none")]
    pub responder_champion_id: Option<i32>,
    #[serde(rename = "responderInternalSummonerName", skip_serializing_if = "Option::is_none")]
    pub responder_internal_summoner_name: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl ChampSelectLcdsTradeContractDto {
    pub fn new() -> ChampSelectLcdsTradeContractDto {
        ChampSelectLcdsTradeContractDto {
            requester_champion_id: None,
            requester_internal_summoner_name: None,
            responder_champion_id: None,
            responder_internal_summoner_name: None,
            state: None,
        }
    }
}


