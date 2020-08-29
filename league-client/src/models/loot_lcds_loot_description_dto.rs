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
pub struct LootLcdsLootDescriptionDto {
    #[serde(rename = "childLootTableNames", skip_serializing_if = "Option::is_none")]
    pub child_loot_table_names: Option<Vec<String>>,
    #[serde(rename = "localizationLongDescriptionMap", skip_serializing_if = "Option::is_none")]
    pub localization_long_description_map: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "localizationMap", skip_serializing_if = "Option::is_none")]
    pub localization_map: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "lootName", skip_serializing_if = "Option::is_none")]
    pub loot_name: Option<String>,
}

impl LootLcdsLootDescriptionDto {
    pub fn new() -> LootLcdsLootDescriptionDto {
        LootLcdsLootDescriptionDto {
            child_loot_table_names: None,
            localization_long_description_map: None,
            localization_map: None,
            loot_name: None,
        }
    }
}


