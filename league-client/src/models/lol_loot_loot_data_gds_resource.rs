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
pub struct LolLootLootDataGdsResource {
    #[serde(rename = "LootBundles", skip_serializing_if = "Option::is_none")]
    pub loot_bundles: Option<Vec<crate::models::LolLootLootBundleGdsResource>>,
    #[serde(rename = "LootItems", skip_serializing_if = "Option::is_none")]
    pub loot_items: Option<Vec<crate::models::LolLootLootItemGdsResource>>,
    #[serde(rename = "LootRecipes", skip_serializing_if = "Option::is_none")]
    pub loot_recipes: Option<Vec<crate::models::LolLootLootRecipeGdsResource>>,
    #[serde(rename = "LootTables", skip_serializing_if = "Option::is_none")]
    pub loot_tables: Option<Vec<crate::models::LolLootLootTableGdsResource>>,
}

impl LolLootLootDataGdsResource {
    pub fn new() -> LolLootLootDataGdsResource {
        LolLootLootDataGdsResource {
            loot_bundles: None,
            loot_items: None,
            loot_recipes: None,
            loot_tables: None,
        }
    }
}


