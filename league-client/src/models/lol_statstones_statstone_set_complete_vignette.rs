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
pub struct LolStatstonesStatstoneSetCompleteVignette {
    #[serde(rename = "statstones", skip_serializing_if = "Option::is_none")]
    pub statstones: Option<Vec<crate::models::LolStatstonesStatstoneCompletion>>,
}

impl LolStatstonesStatstoneSetCompleteVignette {
    pub fn new() -> LolStatstonesStatstoneSetCompleteVignette {
        LolStatstonesStatstoneSetCompleteVignette {
            statstones: None,
        }
    }
}


