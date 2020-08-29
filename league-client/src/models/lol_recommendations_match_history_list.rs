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
pub struct LolRecommendationsMatchHistoryList {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "games", skip_serializing_if = "Option::is_none")]
    pub games: Option<crate::models::LolRecommendationsMatchHistoryGameList>,
    #[serde(rename = "platformId", skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
}

impl LolRecommendationsMatchHistoryList {
    pub fn new() -> LolRecommendationsMatchHistoryList {
        LolRecommendationsMatchHistoryList {
            account_id: None,
            games: None,
            platform_id: None,
        }
    }
}


