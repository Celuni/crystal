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
pub struct LolInventoryRmsStoreEntitlementPayload {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::LolInventoryRmsStoreEntitlementItem>>,
    #[serde(rename = "transactionId", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

impl LolInventoryRmsStoreEntitlementPayload {
    pub fn new() -> LolInventoryRmsStoreEntitlementPayload {
        LolInventoryRmsStoreEntitlementPayload {
            items: None,
            transaction_id: None,
        }
    }
}

