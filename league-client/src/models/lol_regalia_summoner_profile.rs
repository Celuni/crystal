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
pub struct LolRegaliaSummonerProfile {
    #[serde(rename = "regalia", skip_serializing_if = "Option::is_none")]
    pub regalia: Option<String>,
}

impl LolRegaliaSummonerProfile {
    pub fn new() -> LolRegaliaSummonerProfile {
        LolRegaliaSummonerProfile {
            regalia: None,
        }
    }
}


