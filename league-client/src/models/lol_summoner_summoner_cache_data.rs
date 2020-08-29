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
pub struct LolSummonerSummonerCacheData {
    #[serde(rename = "summoner", skip_serializing_if = "Option::is_none")]
    pub summoner: Option<crate::models::LolSummonerSummoner>,
    #[serde(rename = "summonerIcon", skip_serializing_if = "Option::is_none")]
    pub summoner_icon: Option<i32>,
    #[serde(rename = "summonerName", skip_serializing_if = "Option::is_none")]
    pub summoner_name: Option<String>,
}

impl LolSummonerSummonerCacheData {
    pub fn new() -> LolSummonerSummonerCacheData {
        LolSummonerSummonerCacheData {
            summoner: None,
            summoner_icon: None,
            summoner_name: None,
        }
    }
}


