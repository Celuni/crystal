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
pub struct MatchmakingLcdsQueueInfo {
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[serde(rename = "waitTime", skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<i64>,
}

impl MatchmakingLcdsQueueInfo {
    pub fn new() -> MatchmakingLcdsQueueInfo {
        MatchmakingLcdsQueueInfo {
            queue_id: None,
            wait_time: None,
        }
    }
}


