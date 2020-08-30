/*
 * 
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct PluginLolGameflowApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolGameflowApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolGameflowApiClient<C> {
        PluginLolGameflowApiClient {
            configuration,
        }
    }
}

pub trait PluginLolGameflowApi {
    fn delete_lol_gameflow_v1_early_exit_notifications_eog(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_lol_gameflow_v1_early_exit_notifications_eog_by_key(&self, key: i32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_lol_gameflow_v1_early_exit_notifications_missions(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_lol_gameflow_v1_early_exit_notifications_missions_by_key(&self, key: i32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_active_patcher_lock(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_availability(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowGameflowAvailability, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_basic_tutorial(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_battle_training(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_early_exit_notifications_eog(&self, ) -> Box<dyn Future<Item = Vec<serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_early_exit_notifications_missions(&self, ) -> Box<dyn Future<Item = Vec<serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_extra_game_client_args(&self, ) -> Box<dyn Future<Item = Vec<String>, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_gameflow_metadata_player_status(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowPlayerStatus, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_gameflow_metadata_registration_status(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowRegistrationStatus, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_gameflow_phase(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowGameflowPhase, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_session(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowGameflowSession, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_spectate(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_spectate_delayed_launch(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_lol_gameflow_v1_watch(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowGameflowWatchPhase, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_ack_failed_to_launch(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_basic_tutorial_start(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_battle_training_start(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_battle_training_stop(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_client_received_message(&self, messsage: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_early_exit(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_extra_game_client_args(&self, extra_game_client_args: Vec<String>) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_gameflow_metadata_player_status(&self, player_status: crate::models::LolGameflowPlayerStatus) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_gameflow_metadata_registration_status(&self, registration_status: crate::models::LolGameflowRegistrationStatus) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_pre_end_game_transition(&self, enabled: bool) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_reconnect(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_session_dodge(&self, dodge_data: crate::models::LolGameflowGameflowGameDodge) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_session_event(&self, session: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_session_game_configuration(&self, queue: crate::models::LolGameflowQueue) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_session_request_enter_gameflow(&self, event_type: &str) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_session_request_lobby(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_session_request_tournament_checkin(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_session_tournament_ended(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_spectate_launch(&self, target_summoner_name: Option<&str>) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_spectate_quit(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_tick(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v1_watch_launch(&self, args: Vec<String>) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_gameflow_v2_spectate_launch(&self, args: crate::models::LolGameflowSpectateGameInfoResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolGameflowApi for PluginLolGameflowApiClient<C> {
    fn delete_lol_gameflow_v1_early_exit_notifications_eog(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-gameflow/v1/early-exit-notifications/eog".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_gameflow_v1_early_exit_notifications_eog_by_key(&self, key: i32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-gameflow/v1/early-exit-notifications/eog/{key}".to_string())
        ;
        req = req.with_path_param("key".to_string(), key.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_gameflow_v1_early_exit_notifications_missions(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-gameflow/v1/early-exit-notifications/missions".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_gameflow_v1_early_exit_notifications_missions_by_key(&self, key: i32) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-gameflow/v1/early-exit-notifications/missions/{key}".to_string())
        ;
        req = req.with_path_param("key".to_string(), key.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_active_patcher_lock(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/active-patcher-lock".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_availability(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowGameflowAvailability, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/availability".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_basic_tutorial(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/basic-tutorial".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_battle_training(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/battle-training".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_early_exit_notifications_eog(&self, ) -> Box<dyn Future<Item = Vec<serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/early-exit-notifications/eog".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_early_exit_notifications_missions(&self, ) -> Box<dyn Future<Item = Vec<serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/early-exit-notifications/missions".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_extra_game_client_args(&self, ) -> Box<dyn Future<Item = Vec<String>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/extra-game-client-args".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_gameflow_metadata_player_status(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowPlayerStatus, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/gameflow-metadata/player-status".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_gameflow_metadata_registration_status(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowRegistrationStatus, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/gameflow-metadata/registration-status".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_gameflow_phase(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowGameflowPhase, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/gameflow-phase".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_session(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowGameflowSession, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/session".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_spectate(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/spectate".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_spectate_delayed_launch(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/spectate/delayed-launch".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn get_lol_gameflow_v1_watch(&self, ) -> Box<dyn Future<Item = crate::models::LolGameflowGameflowWatchPhase, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-gameflow/v1/watch".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_ack_failed_to_launch(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/ack-failed-to-launch".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_basic_tutorial_start(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/basic-tutorial/start".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_battle_training_start(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/battle-training/start".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_battle_training_stop(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/battle-training/stop".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_client_received_message(&self, messsage: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/client-received-message".to_string())
        ;
        req = req.with_body_param(messsage);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_early_exit(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/early-exit".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_extra_game_client_args(&self, extra_game_client_args: Vec<String>) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/extra-game-client-args".to_string())
        ;
        req = req.with_body_param(extra_game_client_args);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_gameflow_metadata_player_status(&self, player_status: crate::models::LolGameflowPlayerStatus) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/gameflow-metadata/player-status".to_string())
        ;
        req = req.with_body_param(player_status);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_gameflow_metadata_registration_status(&self, registration_status: crate::models::LolGameflowRegistrationStatus) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/gameflow-metadata/registration-status".to_string())
        ;
        req = req.with_body_param(registration_status);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_pre_end_game_transition(&self, enabled: bool) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/pre-end-game-transition".to_string())
        ;
        req = req.with_query_param("enabled".to_string(), enabled.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_reconnect(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/reconnect".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_session_dodge(&self, dodge_data: crate::models::LolGameflowGameflowGameDodge) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/session/dodge".to_string())
        ;
        req = req.with_body_param(dodge_data);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_session_event(&self, session: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/session/event".to_string())
        ;
        req = req.with_body_param(session);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_session_game_configuration(&self, queue: crate::models::LolGameflowQueue) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/session/game-configuration".to_string())
        ;
        req = req.with_body_param(queue);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_session_request_enter_gameflow(&self, event_type: &str) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/session/request-enter-gameflow".to_string())
        ;
        req = req.with_body_param(event_type);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_session_request_lobby(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/session/request-lobby".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_session_request_tournament_checkin(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/session/request-tournament-checkin".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_session_tournament_ended(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/session/tournament-ended".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_spectate_launch(&self, target_summoner_name: Option<&str>) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/spectate/launch".to_string())
        ;
        req = req.with_body_param(target_summoner_name);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_spectate_quit(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/spectate/quit".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_tick(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/tick".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v1_watch_launch(&self, args: Vec<String>) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v1/watch/launch".to_string())
        ;
        req = req.with_body_param(args);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_gameflow_v2_spectate_launch(&self, args: crate::models::LolGameflowSpectateGameInfoResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-gameflow/v2/spectate/launch".to_string())
        ;
        req = req.with_body_param(args);

        req.execute(self.configuration.borrow())
    }

}