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
pub struct LolStatstonesCollectionsChampion {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "squarePortraitPath", skip_serializing_if = "Option::is_none")]
    pub square_portrait_path: Option<String>,
}

impl LolStatstonesCollectionsChampion {
    pub fn new() -> LolStatstonesCollectionsChampion {
        LolStatstonesCollectionsChampion {
            id: None,
            name: None,
            square_portrait_path: None,
        }
    }
}


