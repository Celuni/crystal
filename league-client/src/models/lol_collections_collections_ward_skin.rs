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
pub struct LolCollectionsCollectionsWardSkin {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownership", skip_serializing_if = "Option::is_none")]
    pub ownership: Option<crate::models::LolCollectionsCollectionsOwnership>,
    #[serde(rename = "wardImagePath", skip_serializing_if = "Option::is_none")]
    pub ward_image_path: Option<String>,
    #[serde(rename = "wardShadowImagePath", skip_serializing_if = "Option::is_none")]
    pub ward_shadow_image_path: Option<String>,
}

impl LolCollectionsCollectionsWardSkin {
    pub fn new() -> LolCollectionsCollectionsWardSkin {
        LolCollectionsCollectionsWardSkin {
            description: None,
            id: None,
            name: None,
            ownership: None,
            ward_image_path: None,
            ward_shadow_image_path: None,
        }
    }
}


