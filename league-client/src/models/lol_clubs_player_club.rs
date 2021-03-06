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
pub struct LolClubsPlayerClub {
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isClubTagEligible", skip_serializing_if = "Option::is_none")]
    pub is_club_tag_eligible: Option<bool>,
    #[serde(rename = "jid", skip_serializing_if = "Option::is_none")]
    pub jid: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<i64>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<crate::models::LolClubsClubMemberLists>,
    #[serde(rename = "motd", skip_serializing_if = "Option::is_none")]
    pub motd: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<crate::models::LolClubsClubMember>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<crate::models::LolClubsClubPermissions>,
    #[serde(rename = "primary", skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::LolClubsClubRole>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl LolClubsPlayerClub {
    pub fn new() -> LolClubsPlayerClub {
        LolClubsPlayerClub {
            created: None,
            description: None,
            is_club_tag_eligible: None,
            jid: None,
            key: None,
            last_modified: None,
            members: None,
            motd: None,
            name: None,
            owner: None,
            permissions: None,
            primary: None,
            role: None,
            tag: None,
        }
    }
}


