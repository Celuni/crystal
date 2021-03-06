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
pub struct LootItemClientDto {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "displayCategories", skip_serializing_if = "Option::is_none")]
    pub display_categories: Option<String>,
    #[serde(rename = "expiryTime", skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<i64>,
    #[serde(rename = "lootName", skip_serializing_if = "Option::is_none")]
    pub loot_name: Option<String>,
    #[serde(rename = "rarity", skip_serializing_if = "Option::is_none")]
    pub rarity: Option<String>,
    #[serde(rename = "rentalGames", skip_serializing_if = "Option::is_none")]
    pub rental_games: Option<i32>,
    #[serde(rename = "rentalSeconds", skip_serializing_if = "Option::is_none")]
    pub rental_seconds: Option<i64>,
    #[serde(rename = "storeItemId", skip_serializing_if = "Option::is_none")]
    pub store_item_id: Option<i32>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "upgradeLootName", skip_serializing_if = "Option::is_none")]
    pub upgrade_loot_name: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

impl LootItemClientDto {
    pub fn new() -> LootItemClientDto {
        LootItemClientDto {
            asset: None,
            display_categories: None,
            expiry_time: None,
            loot_name: None,
            rarity: None,
            rental_games: None,
            rental_seconds: None,
            store_item_id: None,
            tags: None,
            _type: None,
            upgrade_loot_name: None,
            value: None,
        }
    }
}


