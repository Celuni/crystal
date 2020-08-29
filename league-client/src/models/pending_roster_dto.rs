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
pub struct PendingRosterDto {
    #[serde(rename = "captainId", skip_serializing_if = "Option::is_none")]
    pub captain_id: Option<i64>,
    #[serde(rename = "highTierVariance", skip_serializing_if = "Option::is_none")]
    pub high_tier_variance: Option<bool>,
    #[serde(rename = "invitationId", skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "inviteFaileds", skip_serializing_if = "Option::is_none")]
    pub invite_faileds: Option<Vec<crate::models::FailedInvite>>,
    #[serde(rename = "invitees", skip_serializing_if = "Option::is_none")]
    pub invitees: Option<Vec<crate::models::PendingRosterInviteeDto>>,
    #[serde(rename = "logo", skip_serializing_if = "Option::is_none")]
    pub logo: Option<i32>,
    #[serde(rename = "logoColor", skip_serializing_if = "Option::is_none")]
    pub logo_color: Option<i32>,
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::PendingRosterMemberDto>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "rewardLogos", skip_serializing_if = "Option::is_none")]
    pub reward_logos: Option<Vec<crate::models::RewardLogo>>,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "ticketOffers", skip_serializing_if = "Option::is_none")]
    pub ticket_offers: Option<Vec<crate::models::TicketOfferDto>>,
    #[serde(rename = "tier", skip_serializing_if = "Option::is_none")]
    pub tier: Option<i32>,
    #[serde(rename = "tournamentId", skip_serializing_if = "Option::is_none")]
    pub tournament_id: Option<i64>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl PendingRosterDto {
    pub fn new() -> PendingRosterDto {
        PendingRosterDto {
            captain_id: None,
            high_tier_variance: None,
            invitation_id: None,
            invite_faileds: None,
            invitees: None,
            logo: None,
            logo_color: None,
            members: None,
            name: None,
            reward_logos: None,
            short_name: None,
            ticket_offers: None,
            tier: None,
            tournament_id: None,
            version: None,
        }
    }
}


