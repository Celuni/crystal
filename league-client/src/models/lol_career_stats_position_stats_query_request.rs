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
pub struct LolCareerStatsPositionStatsQueryRequest {
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<crate::models::LolCareerStatsSummonersRiftPosition>,
    #[serde(rename = "queueType", skip_serializing_if = "Option::is_none")]
    pub queue_type: Option<crate::models::LolCareerStatsCareerStatsQueueType>,
    #[serde(rename = "rankTier", skip_serializing_if = "Option::is_none")]
    pub rank_tier: Option<crate::models::LolCareerStatsRankedTier>,
    #[serde(rename = "season", skip_serializing_if = "Option::is_none")]
    pub season: Option<i32>,
}

impl LolCareerStatsPositionStatsQueryRequest {
    pub fn new() -> LolCareerStatsPositionStatsQueryRequest {
        LolCareerStatsPositionStatsQueryRequest {
            position: None,
            queue_type: None,
            rank_tier: None,
            season: None,
        }
    }
}


