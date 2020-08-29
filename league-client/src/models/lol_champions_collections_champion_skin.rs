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
pub struct LolChampionsCollectionsChampionSkin {
    #[serde(rename = "championId", skip_serializing_if = "Option::is_none")]
    pub champion_id: Option<i32>,
    #[serde(rename = "chromaPath", skip_serializing_if = "Option::is_none")]
    pub chroma_path: Option<String>,
    #[serde(rename = "chromas", skip_serializing_if = "Option::is_none")]
    pub chromas: Option<Vec<crate::models::LolChampionsCollectionsChampionChroma>>,
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "emblems", skip_serializing_if = "Option::is_none")]
    pub emblems: Option<Vec<crate::models::LolChampionsCollectionsChampionSkinEmblem>>,
    #[serde(rename = "featuresText", skip_serializing_if = "Option::is_none")]
    pub features_text: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "isBase", skip_serializing_if = "Option::is_none")]
    pub is_base: Option<bool>,
    #[serde(rename = "lastSelected", skip_serializing_if = "Option::is_none")]
    pub last_selected: Option<bool>,
    #[serde(rename = "loadScreenPath", skip_serializing_if = "Option::is_none")]
    pub load_screen_path: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownership", skip_serializing_if = "Option::is_none")]
    pub ownership: Option<crate::models::LolChampionsCollectionsOwnership>,
    #[serde(rename = "rarityGemPath", skip_serializing_if = "Option::is_none")]
    pub rarity_gem_path: Option<String>,
    #[serde(rename = "skinType", skip_serializing_if = "Option::is_none")]
    pub skin_type: Option<String>,
    #[serde(rename = "splashPath", skip_serializing_if = "Option::is_none")]
    pub splash_path: Option<String>,
    #[serde(rename = "splashVideoPath", skip_serializing_if = "Option::is_none")]
    pub splash_video_path: Option<String>,
    #[serde(rename = "stillObtainable", skip_serializing_if = "Option::is_none")]
    pub still_obtainable: Option<bool>,
    #[serde(rename = "tilePath", skip_serializing_if = "Option::is_none")]
    pub tile_path: Option<String>,
    #[serde(rename = "uncenteredSplashPath", skip_serializing_if = "Option::is_none")]
    pub uncentered_splash_path: Option<String>,
}

impl LolChampionsCollectionsChampionSkin {
    pub fn new() -> LolChampionsCollectionsChampionSkin {
        LolChampionsCollectionsChampionSkin {
            champion_id: None,
            chroma_path: None,
            chromas: None,
            disabled: None,
            emblems: None,
            features_text: None,
            id: None,
            is_base: None,
            last_selected: None,
            load_screen_path: None,
            name: None,
            ownership: None,
            rarity_gem_path: None,
            skin_type: None,
            splash_path: None,
            splash_video_path: None,
            still_obtainable: None,
            tile_path: None,
            uncentered_splash_path: None,
        }
    }
}


