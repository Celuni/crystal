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
pub struct LolPatchSupportedGameReleases {
    #[serde(rename = "supported_game_releases", skip_serializing_if = "Option::is_none")]
    pub supported_game_releases: Option<Vec<crate::models::LolPatchSupportedGameRelease>>,
}

impl LolPatchSupportedGameReleases {
    pub fn new() -> LolPatchSupportedGameReleases {
        LolPatchSupportedGameReleases {
            supported_game_releases: None,
        }
    }
}


