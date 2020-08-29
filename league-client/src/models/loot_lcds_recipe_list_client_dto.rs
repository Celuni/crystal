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
pub struct LootLcdsRecipeListClientDto {
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<i64>,
    #[serde(rename = "recipes", skip_serializing_if = "Option::is_none")]
    pub recipes: Option<Vec<crate::models::LootLcdsRecipeClientDto>>,
}

impl LootLcdsRecipeListClientDto {
    pub fn new() -> LootLcdsRecipeListClientDto {
        LootLcdsRecipeListClientDto {
            last_update: None,
            recipes: None,
        }
    }
}


