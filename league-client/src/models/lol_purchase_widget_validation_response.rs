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
pub struct LolPurchaseWidgetValidationResponse {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::LolPurchaseWidgetValidationResponseItem>>,
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

impl LolPurchaseWidgetValidationResponse {
    pub fn new() -> LolPurchaseWidgetValidationResponse {
        LolPurchaseWidgetValidationResponse {
            items: None,
            valid: None,
        }
    }
}


