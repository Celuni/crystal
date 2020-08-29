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
pub struct LolStoreCapOfferPayloadEntry {
    #[serde(rename = "fulfillmentTypeId", skip_serializing_if = "Option::is_none")]
    pub fulfillment_type_id: Option<String>,
    #[serde(rename = "inventoryTypeUUID", skip_serializing_if = "Option::is_none")]
    pub inventory_type_uuid: Option<String>,
    #[serde(rename = "itemInstanceId", skip_serializing_if = "Option::is_none")]
    pub item_instance_id: Option<String>,
    #[serde(rename = "itemPriceMap", skip_serializing_if = "Option::is_none")]
    pub item_price_map: Option<::std::collections::HashMap<String, i32>>,
}

impl LolStoreCapOfferPayloadEntry {
    pub fn new() -> LolStoreCapOfferPayloadEntry {
        LolStoreCapOfferPayloadEntry {
            fulfillment_type_id: None,
            inventory_type_uuid: None,
            item_instance_id: None,
            item_price_map: None,
        }
    }
}


