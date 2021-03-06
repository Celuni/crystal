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
pub struct LolClashInviteSubRequest {
    #[serde(rename = "replacedSummonerId", skip_serializing_if = "Option::is_none")]
    pub replaced_summoner_id: Option<i64>,
    #[serde(rename = "substituteSummonerId", skip_serializing_if = "Option::is_none")]
    pub substitute_summoner_id: Option<i64>,
}

impl LolClashInviteSubRequest {
    pub fn new() -> LolClashInviteSubRequest {
        LolClashInviteSubRequest {
            replaced_summoner_id: None,
            substitute_summoner_id: None,
        }
    }
}


