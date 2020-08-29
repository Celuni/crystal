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
pub struct LolContentTargetingChampionGames {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "lane", skip_serializing_if = "Option::is_none")]
    pub lane: Option<String>,
    #[serde(rename = "queue", skip_serializing_if = "Option::is_none")]
    pub queue: Option<i32>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl LolContentTargetingChampionGames {
    pub fn new() -> LolContentTargetingChampionGames {
        LolContentTargetingChampionGames {
            champion_id: None,
            lane: None,
            queue: None,
            role: None,
            timestamp: None,
        }
    }
}


