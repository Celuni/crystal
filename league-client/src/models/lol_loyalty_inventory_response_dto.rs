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
pub struct LolLoyaltyInventoryResponseDto {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<crate::models::LolLoyaltyInventoryDto>,
}

impl LolLoyaltyInventoryResponseDto {
    pub fn new() -> LolLoyaltyInventoryResponseDto {
        LolLoyaltyInventoryResponseDto {
            data: None,
        }
    }
}


