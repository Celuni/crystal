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
pub struct LootLcdsRecipeSlotClientDto {
    #[serde(rename = "quantityExpression", skip_serializing_if = "Option::is_none")]
    pub quantity_expression: Option<String>,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "slotNumber", skip_serializing_if = "Option::is_none")]
    pub slot_number: Option<i32>,
}

impl LootLcdsRecipeSlotClientDto {
    pub fn new() -> LootLcdsRecipeSlotClientDto {
        LootLcdsRecipeSlotClientDto {
            quantity_expression: None,
            query: None,
            slot_number: None,
        }
    }
}


