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
pub struct LolChampSelectChampSelectSummoner {
    #[serde(rename = "actingBackgroundAnimationState", skip_serializing_if = "Option::is_none")]
    pub acting_background_animation_state: Option<String>,
    #[serde(rename = "activeActionType", skip_serializing_if = "Option::is_none")]
    pub active_action_type: Option<String>,
    #[serde(rename = "areSummonerActionsComplete", skip_serializing_if = "Option::is_none")]
    pub are_summoner_actions_complete: Option<bool>,
    #[serde(rename = "assignedPosition", skip_serializing_if = "Option::is_none")]
    pub assigned_position: Option<String>,
    #[serde(rename = "banIntentSquarePortratPath", skip_serializing_if = "Option::is_none")]
    pub ban_intent_square_portrat_path: Option<String>,
    #[serde(rename = "cellId", skip_serializing_if = "Option::is_none")]
    pub cell_id: Option<i64>,
    #[serde(rename = "championIconStyle", skip_serializing_if = "Option::is_none")]
    pub champion_icon_style: Option<String>,
    #[serde(rename = "championName", skip_serializing_if = "Option::is_none")]
    pub champion_name: Option<String>,
    #[serde(rename = "currentChampionVotePercentInteger", skip_serializing_if = "Option::is_none")]
    pub current_champion_vote_percent_integer: Option<i32>,
    #[serde(rename = "entitledFeatureType", skip_serializing_if = "Option::is_none")]
    pub entitled_feature_type: Option<String>,
    #[serde(rename = "isActingNow", skip_serializing_if = "Option::is_none")]
    pub is_acting_now: Option<bool>,
    #[serde(rename = "isDonePicking", skip_serializing_if = "Option::is_none")]
    pub is_done_picking: Option<bool>,
    #[serde(rename = "isOnPlayersTeam", skip_serializing_if = "Option::is_none")]
    pub is_on_players_team: Option<bool>,
    #[serde(rename = "isPickIntenting", skip_serializing_if = "Option::is_none")]
    pub is_pick_intenting: Option<bool>,
    #[serde(rename = "isPlaceholder", skip_serializing_if = "Option::is_none")]
    pub is_placeholder: Option<bool>,
    #[serde(rename = "isSelf", skip_serializing_if = "Option::is_none")]
    pub is_self: Option<bool>,
    #[serde(rename = "pickSnipedClass", skip_serializing_if = "Option::is_none")]
    pub pick_sniped_class: Option<String>,
    #[serde(rename = "shouldShowActingBar", skip_serializing_if = "Option::is_none")]
    pub should_show_acting_bar: Option<bool>,
    #[serde(rename = "shouldShowBanIntentIcon", skip_serializing_if = "Option::is_none")]
    pub should_show_ban_intent_icon: Option<bool>,
    #[serde(rename = "shouldShowExpanded", skip_serializing_if = "Option::is_none")]
    pub should_show_expanded: Option<bool>,
    #[serde(rename = "shouldShowRingAnimations", skip_serializing_if = "Option::is_none")]
    pub should_show_ring_animations: Option<bool>,
    #[serde(rename = "shouldShowSelectedSkin", skip_serializing_if = "Option::is_none")]
    pub should_show_selected_skin: Option<bool>,
    #[serde(rename = "shouldShowSpells", skip_serializing_if = "Option::is_none")]
    pub should_show_spells: Option<bool>,
    #[serde(rename = "showMuted", skip_serializing_if = "Option::is_none")]
    pub show_muted: Option<bool>,
    #[serde(rename = "showTrades", skip_serializing_if = "Option::is_none")]
    pub show_trades: Option<bool>,
    #[serde(rename = "skinId", skip_serializing_if = "Option::is_none")]
    pub skin_id: Option<i32>,
    #[serde(rename = "skinSplashPath", skip_serializing_if = "Option::is_none")]
    pub skin_splash_path: Option<String>,
    #[serde(rename = "slotId", skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<i32>,
    #[serde(rename = "spell1IconPath", skip_serializing_if = "Option::is_none")]
    pub spell1_icon_path: Option<String>,
    #[serde(rename = "spell2IconPath", skip_serializing_if = "Option::is_none")]
    pub spell2_icon_path: Option<String>,
    #[serde(rename = "statusMessageKey", skip_serializing_if = "Option::is_none")]
    pub status_message_key: Option<String>,
    #[serde(rename = "summonerId", skip_serializing_if = "Option::is_none")]
    pub summoner_id: Option<i64>,
    #[serde(rename = "tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<i64>,
}

impl LolChampSelectChampSelectSummoner {
    pub fn new() -> LolChampSelectChampSelectSummoner {
        LolChampSelectChampSelectSummoner {
            acting_background_animation_state: None,
            active_action_type: None,
            are_summoner_actions_complete: None,
            assigned_position: None,
            ban_intent_square_portrat_path: None,
            cell_id: None,
            champion_icon_style: None,
            champion_name: None,
            current_champion_vote_percent_integer: None,
            entitled_feature_type: None,
            is_acting_now: None,
            is_done_picking: None,
            is_on_players_team: None,
            is_pick_intenting: None,
            is_placeholder: None,
            is_self: None,
            pick_sniped_class: None,
            should_show_acting_bar: None,
            should_show_ban_intent_icon: None,
            should_show_expanded: None,
            should_show_ring_animations: None,
            should_show_selected_skin: None,
            should_show_spells: None,
            show_muted: None,
            show_trades: None,
            skin_id: None,
            skin_splash_path: None,
            slot_id: None,
            spell1_icon_path: None,
            spell2_icon_path: None,
            status_message_key: None,
            summoner_id: None,
            trade_id: None,
        }
    }
}


