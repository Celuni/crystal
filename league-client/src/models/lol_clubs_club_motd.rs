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
pub struct LolClubsClubMotd {
    #[serde(rename = "motd", skip_serializing_if = "Option::is_none")]
    pub motd: Option<String>,
}

impl LolClubsClubMotd {
    pub fn new() -> LolClubsClubMotd {
        LolClubsClubMotd {
            motd: None,
        }
    }
}


