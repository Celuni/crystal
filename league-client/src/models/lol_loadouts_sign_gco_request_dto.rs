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
pub struct LolLoadoutsSignGcoRequestDto {
    #[serde(rename = "loadout", skip_serializing_if = "Option::is_none")]
    pub loadout: Option<::std::collections::HashMap<String, crate::models::LolLoadoutsItemKey>>,
    #[serde(rename = "serviceToJwtsMap", skip_serializing_if = "Option::is_none")]
    pub service_to_jwts_map: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl LolLoadoutsSignGcoRequestDto {
    pub fn new() -> LolLoadoutsSignGcoRequestDto {
        LolLoadoutsSignGcoRequestDto {
            loadout: None,
            service_to_jwts_map: None,
        }
    }
}


