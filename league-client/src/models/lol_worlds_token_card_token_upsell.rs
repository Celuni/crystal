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
pub struct LolWorldsTokenCardTokenUpsell {
    #[serde(rename = "backgroundUrl", skip_serializing_if = "Option::is_none")]
    pub background_url: Option<String>,
    #[serde(rename = "buttonText", skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(rename = "currencyUrl", skip_serializing_if = "Option::is_none")]
    pub currency_url: Option<String>,
    #[serde(rename = "currentlyLocked", skip_serializing_if = "Option::is_none")]
    pub currently_locked: Option<crate::models::LolWorldsTokenCardTokenUpsellLockedType>,
    #[serde(rename = "dependentInventoryId", skip_serializing_if = "Option::is_none")]
    pub dependent_inventory_id: Option<i32>,
    #[serde(rename = "dependentInventoryType", skip_serializing_if = "Option::is_none")]
    pub dependent_inventory_type: Option<String>,
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "internalName", skip_serializing_if = "Option::is_none")]
    pub internal_name: Option<String>,
    #[serde(rename = "lockedCount", skip_serializing_if = "Option::is_none")]
    pub locked_count: Option<i32>,
    #[serde(rename = "premiumCurrencyName", skip_serializing_if = "Option::is_none")]
    pub premium_currency_name: Option<String>,
    #[serde(rename = "purchaseUrl", skip_serializing_if = "Option::is_none")]
    pub purchase_url: Option<String>,
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "tooltipBackgroundUrl", skip_serializing_if = "Option::is_none")]
    pub tooltip_background_url: Option<String>,
    #[serde(rename = "tooltipDescription", skip_serializing_if = "Option::is_none")]
    pub tooltip_description: Option<String>,
    #[serde(rename = "tooltipTitle", skip_serializing_if = "Option::is_none")]
    pub tooltip_title: Option<String>,
}

impl LolWorldsTokenCardTokenUpsell {
    pub fn new() -> LolWorldsTokenCardTokenUpsell {
        LolWorldsTokenCardTokenUpsell {
            background_url: None,
            button_text: None,
            currency_url: None,
            currently_locked: None,
            dependent_inventory_id: None,
            dependent_inventory_type: None,
            end_date: None,
            id: None,
            internal_name: None,
            locked_count: None,
            premium_currency_name: None,
            purchase_url: None,
            start_date: None,
            title: None,
            tooltip_background_url: None,
            tooltip_description: None,
            tooltip_title: None,
        }
    }
}


