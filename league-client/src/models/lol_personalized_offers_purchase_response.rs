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
pub struct LolPersonalizedOffersPurchaseResponse {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::LolPersonalizedOffersPurchaseItem>>,
    #[serde(rename = "wallet", skip_serializing_if = "Option::is_none")]
    pub wallet: Option<crate::models::LolPersonalizedOffersWallet>,
}

impl LolPersonalizedOffersPurchaseResponse {
    pub fn new() -> LolPersonalizedOffersPurchaseResponse {
        LolPersonalizedOffersPurchaseResponse {
            items: None,
            wallet: None,
        }
    }
}


