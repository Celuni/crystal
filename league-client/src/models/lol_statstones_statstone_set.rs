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
pub struct LolStatstonesStatstoneSet {
    #[serde(rename = "inventoryType", skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "itemInstanceID", skip_serializing_if = "Option::is_none")]
    pub item_instance_id: Option<String>,
    #[serde(rename = "milestonesPassed", skip_serializing_if = "Option::is_none")]
    pub milestones_passed: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownedFromPacks", skip_serializing_if = "Option::is_none")]
    pub owned_from_packs: Option<Vec<crate::models::LolStatstonesGameDataStatstonePack>>,
    #[serde(rename = "prices", skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<crate::models::LolStatstonesPriceInfo>>,
    #[serde(rename = "statstones", skip_serializing_if = "Option::is_none")]
    pub statstones: Option<Vec<crate::models::LolStatstonesStatstone>>,
    #[serde(rename = "stonesOwned", skip_serializing_if = "Option::is_none")]
    pub stones_owned: Option<i32>,
    #[serde(rename = "subInventoryType", skip_serializing_if = "Option::is_none")]
    pub sub_inventory_type: Option<String>,
}

impl LolStatstonesStatstoneSet {
    pub fn new() -> LolStatstonesStatstoneSet {
        LolStatstonesStatstoneSet {
            inventory_type: None,
            item_id: None,
            item_instance_id: None,
            milestones_passed: None,
            name: None,
            owned_from_packs: None,
            prices: None,
            statstones: None,
            stones_owned: None,
            sub_inventory_type: None,
        }
    }
}


