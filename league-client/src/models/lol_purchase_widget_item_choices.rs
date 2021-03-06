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
pub struct LolPurchaseWidgetItemChoices {
    #[serde(rename = "choices", skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<crate::models::LolPurchaseWidgetItemChoiceDetails>>,
    #[serde(rename = "validationErrors", skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<crate::models::LolPurchaseWidgetValidationErrorEntry>>,
}

impl LolPurchaseWidgetItemChoices {
    pub fn new() -> LolPurchaseWidgetItemChoices {
        LolPurchaseWidgetItemChoices {
            choices: None,
            validation_errors: None,
        }
    }
}


