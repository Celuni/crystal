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
pub struct LolPatchPatchSieveQueryResponse {
    #[serde(rename = "releases", skip_serializing_if = "Option::is_none")]
    pub releases: Option<Vec<crate::models::LolPatchPatchSieveRelease>>,
}

impl LolPatchPatchSieveQueryResponse {
    pub fn new() -> LolPatchPatchSieveQueryResponse {
        LolPatchPatchSieveQueryResponse {
            releases: None,
        }
    }
}


