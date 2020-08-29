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
pub struct LolLootGameDataSummonerEmote {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "inventoryIcon", skip_serializing_if = "Option::is_none")]
    pub inventory_icon: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LolLootGameDataSummonerEmote {
    pub fn new() -> LolLootGameDataSummonerEmote {
        LolLootGameDataSummonerEmote {
            description: None,
            id: None,
            inventory_icon: None,
            name: None,
        }
    }
}


