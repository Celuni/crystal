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
pub struct LolHonorV2ServiceProxyHonorPlayerServerRequest {
    #[serde(rename = "gameId", skip_serializing_if = "Option::is_none")]
    pub game_id: Option<i64>,
    #[serde(rename = "honorType", skip_serializing_if = "Option::is_none")]
    pub honor_type: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolHonorV2ServiceProxyHonorPlayerServerRequest {
    pub fn new() -> LolHonorV2ServiceProxyHonorPlayerServerRequest {
        LolHonorV2ServiceProxyHonorPlayerServerRequest {
            game_id: None,
            honor_type: None,
            summoner_id: None,
        }
    }
}


