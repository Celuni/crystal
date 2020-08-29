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
pub struct LolLoadoutsUpdateLoadoutDto {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "loadout", skip_serializing_if = "Option::is_none")]
    pub loadout: Option<::std::collections::HashMap<String, crate::models::LolLoadoutsItemKey>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl LolLoadoutsUpdateLoadoutDto {
    pub fn new() -> LolLoadoutsUpdateLoadoutDto {
        LolLoadoutsUpdateLoadoutDto {
            id: None,
            loadout: None,
            name: None,
        }
    }
}


