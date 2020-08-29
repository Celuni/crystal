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
pub struct LolHonorV2MutualHonorPlayer {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "skinIndex", skip_serializing_if = "Option::is_none")]
    pub skin_index: Option<i32>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
}

impl LolHonorV2MutualHonorPlayer {
    pub fn new() -> LolHonorV2MutualHonorPlayer {
        LolHonorV2MutualHonorPlayer {
            champion_id: None,
            skin_index: None,
            summoner_id: None,
        }
    }
}


