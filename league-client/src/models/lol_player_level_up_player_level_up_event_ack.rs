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
pub struct LolPlayerLevelUpPlayerLevelUpEventAck {
    #[serde(rename = "newSummonerLevel", skip_serializing_if = "Option::is_none")]
    pub new_summoner_level: Option<i32>,
    #[serde(rename = "seenThisEvent", skip_serializing_if = "Option::is_none")]
    pub seen_this_event: Option<bool>,
}

impl LolPlayerLevelUpPlayerLevelUpEventAck {
    pub fn new() -> LolPlayerLevelUpPlayerLevelUpEventAck {
        LolPlayerLevelUpPlayerLevelUpEventAck {
            new_summoner_level: None,
            seen_this_event: None,
        }
    }
}


