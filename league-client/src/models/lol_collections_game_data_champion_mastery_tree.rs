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
pub struct LolCollectionsGameDataChampionMasteryTree {
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::models::LolCollectionsGameDataChampionMasteryGroup>>,
}

impl LolCollectionsGameDataChampionMasteryTree {
    pub fn new() -> LolCollectionsGameDataChampionMasteryTree {
        LolCollectionsGameDataChampionMasteryTree {
            groups: None,
        }
    }
}


