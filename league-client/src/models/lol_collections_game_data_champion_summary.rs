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
pub struct LolCollectionsGameDataChampionSummary {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

impl LolCollectionsGameDataChampionSummary {
    pub fn new() -> LolCollectionsGameDataChampionSummary {
        LolCollectionsGameDataChampionSummary {
            id: None,
        }
    }
}


