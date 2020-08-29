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
pub struct LolStatstonesCatalogItemDetails {
    #[serde(rename = "inventoryType", skip_serializing_if = "Option::is_none")]
    pub inventory_type: Option<String>,
    #[serde(rename = "itemId", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<i32>,
    #[serde(rename = "itemInstanceId", skip_serializing_if = "Option::is_none")]
    pub item_instance_id: Option<String>,
    #[serde(rename = "prices", skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<crate::models::LolStatstonesCatalogBundlePrice>>,
    #[serde(rename = "releaseDate", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(rename = "subInventoryType", skip_serializing_if = "Option::is_none")]
    pub sub_inventory_type: Option<String>,
}

impl LolStatstonesCatalogItemDetails {
    pub fn new() -> LolStatstonesCatalogItemDetails {
        LolStatstonesCatalogItemDetails {
            inventory_type: None,
            item_id: None,
            item_instance_id: None,
            prices: None,
            release_date: None,
            sub_inventory_type: None,
        }
    }
}


