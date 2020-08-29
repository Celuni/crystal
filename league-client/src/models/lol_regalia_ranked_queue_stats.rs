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
pub struct LolRegaliaRankedQueueStats {
    #[serde(rename = "division", skip_serializing_if = "Option::is_none")]
    pub division: Option<crate::models::LolRegaliaLeagueDivision>,
    #[serde(rename = "isProvisional", skip_serializing_if = "Option::is_none")]
    pub is_provisional: Option<bool>,
    #[serde(rename = "queueType", skip_serializing_if = "Option::is_none")]
    pub queue_type: Option<crate::models::LolRegaliaLeagueQueueType>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<crate::models::LolRegaliaLeagueTier>,
}

impl LolRegaliaRankedQueueStats {
    pub fn new() -> LolRegaliaRankedQueueStats {
        LolRegaliaRankedQueueStats {
            division: None,
            is_provisional: None,
            queue_type: None,
            tier: None,
        }
    }
}


