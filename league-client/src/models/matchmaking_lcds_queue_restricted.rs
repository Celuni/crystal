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
pub struct MatchmakingLcdsQueueRestricted {
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    pub queue_id: Option<i32>,
    #[serde(rename = "reasonFailed", skip_serializing_if = "Option::is_none")]
    pub reason_failed: Option<String>,
    #[serde(rename = "summoner", skip_serializing_if = "Option::is_none")]
    pub summoner: Option<crate::models::MatchmakingLcdsSummoner>,
}

impl MatchmakingLcdsQueueRestricted {
    pub fn new() -> MatchmakingLcdsQueueRestricted {
        MatchmakingLcdsQueueRestricted {
            message: None,
            queue_id: None,
            reason_failed: None,
            summoner: None,
        }
    }
}


