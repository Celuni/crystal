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
pub struct LootLcdsClientConfigurationDto {
    #[serde(rename = "currenciesUsingCapWallets", skip_serializing_if = "Option::is_none")]
    pub currencies_using_cap_wallets: Option<Vec<String>>,
    #[serde(rename = "lootItemsUsingBreakoutRecipeMenu", skip_serializing_if = "Option::is_none")]
    pub loot_items_using_breakout_recipe_menu: Option<Vec<String>>,
    #[serde(rename = "lootMaterialsToAlwaysRender", skip_serializing_if = "Option::is_none")]
    pub loot_materials_to_always_render: Option<Vec<String>>,
}

impl LootLcdsClientConfigurationDto {
    pub fn new() -> LootLcdsClientConfigurationDto {
        LootLcdsClientConfigurationDto {
            currencies_using_cap_wallets: None,
            loot_items_using_breakout_recipe_menu: None,
            loot_materials_to_always_render: None,
        }
    }
}


