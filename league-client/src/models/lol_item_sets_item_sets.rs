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
pub struct LolItemSetsItemSets {
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    #[serde(rename = "itemSets", skip_serializing_if = "Option::is_none")]
    pub item_sets: Option<Vec<crate::models::LolItemSetsItemSet>>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl LolItemSetsItemSets {
    pub fn new() -> LolItemSetsItemSets {
        LolItemSetsItemSets {
            account_id: None,
            item_sets: None,
            timestamp: None,
        }
    }
}


