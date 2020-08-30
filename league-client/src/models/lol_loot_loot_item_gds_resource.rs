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
pub struct LolLootLootItemGdsResource {
    #[serde(rename = "autoRedeem", skip_serializing_if = "Option::is_none")]
    pub auto_redeem: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "lifetimeMax", skip_serializing_if = "Option::is_none")]
    pub lifetime_max: Option<i32>,
    #[serde(rename = "mappedStoreId", skip_serializing_if = "Option::is_none")]
    pub mapped_store_id: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "rarity", skip_serializing_if = "Option::is_none")]
    pub rarity: Option<crate::models::LolLootLootRarity>,
    #[serde(rename = "recipeMenuActive", skip_serializing_if = "Option::is_none")]
    pub recipe_menu_active: Option<bool>,
    #[serde(rename = "recipeMenuSubtitle", skip_serializing_if = "Option::is_none")]
    pub recipe_menu_subtitle: Option<String>,
    #[serde(rename = "recipeMenuTitle", skip_serializing_if = "Option::is_none")]
    pub recipe_menu_title: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::LolLootLootType>,
}

impl LolLootLootItemGdsResource {
    pub fn new() -> LolLootLootItemGdsResource {
        LolLootLootItemGdsResource {
            auto_redeem: None,
            description: None,
            end_date: None,
            id: None,
            image: None,
            lifetime_max: None,
            mapped_store_id: None,
            name: None,
            rarity: None,
            recipe_menu_active: None,
            recipe_menu_subtitle: None,
            recipe_menu_title: None,
            start_date: None,
            _type: None,
        }
    }
}

