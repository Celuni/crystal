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
pub struct LolLootCollectionsSummonerIcons {
    #[serde(rename = "icons", skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<i32>>,
}

impl LolLootCollectionsSummonerIcons {
    pub fn new() -> LolLootCollectionsSummonerIcons {
        LolLootCollectionsSummonerIcons {
            icons: None,
        }
    }
}


