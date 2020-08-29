use crate::helpers::JoinIterator;

/*
 *
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use futures::Future;
use hyper;
use serde_json;

use super::request as __internal_request;
use super::{configuration, Error};

pub struct PluginLolLoginApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolLoginApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolLoginApiClient<C> {
        PluginLolLoginApiClient { configuration }
    }
}

pub trait PluginLolLoginApi {
    fn delete_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name(
        &self,
        service_name: &str,
        method_name: &str,
        plugin_id: i32,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_lol_login_v1_session(
        &self,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_lol_login_v1_shutdown_locks_by_lock_name(
        &self,
        lock_name: &str,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_lol_login_v1_account_state(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolLoginAccountStateResource,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_login_v1_login_connection_state(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolLoginLoginConnectionState,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_login_v1_login_data_packet(
        &self,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_login_v1_login_in_game_creds(
        &self,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_login_v1_login_platform_credentials(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolLoginPlatformGeneratedCredentials,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_login_v1_login_queue_state(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LolLoginLoginQueue, Error = Error<serde_json::Value>>>;
    fn get_lol_login_v1_session(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LolLoginLoginSession, Error = Error<serde_json::Value>>>;
    fn get_lol_login_v1_wallet(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolLoginLoginSessionWallet,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn get_lol_login_v2_league_session_init_token(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolLoginLeagueSessionTokenEnvelope,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn post_lol_login_v1_account_state(
        &self,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_login_v1_change_summoner_name(
        &self,
        name: &str,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn post_lol_login_v1_delete_rso_on_close(
        &self,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn post_lol_login_v1_league_session_status(
        &self,
        league_session: &str,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_login_v1_new_player_flow_completed(
        &self,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn post_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name(
        &self,
        service_name: &str,
        method_name: &str,
        plugin_id: i32,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_login_v1_service_proxy_uuid_requests(
        &self,
        service_name: &str,
        method_name: &str,
        plugin_id: i32,
        timeout_millis: i64,
        payload: &str,
    ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>>;
    fn post_lol_login_v1_session(
        &self,
        username_and_password: crate::models::LolLoginUsernameAndPassword,
    ) -> Box<dyn Future<Item = crate::models::LolLoginLoginSession, Error = Error<serde_json::Value>>>;
    fn post_lol_login_v1_session_invoke(
        &self,
        destination: &str,
        method: &str,
        UNKNOWN_BASE_TYPE: crate::models::UNKNOWN_BASE_TYPE,
    ) -> Box<dyn Future<Item = crate::models::LolLoginLcdsResponse, Error = Error<serde_json::Value>>>;
    fn post_lol_login_v1_summoner_created(
        &self,
        summoner_id: crate::models::LolLoginSummonerCreatedResource,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    >;
    fn put_lol_login_v1_shutdown_locks_by_lock_name(
        &self,
        lock_name: &str,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect> PluginLolLoginApi for PluginLolLoginApiClient<C> {
    fn delete_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name(
        &self,
        service_name: &str,
        method_name: &str,
        plugin_id: i32,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/lol-login/v1/service-proxy-async-requests/{serviceName}/{methodName}".to_string(),
        );
        req = req.with_query_param("pluginId".to_string(), plugin_id.to_string());
        req = req.with_path_param("serviceName".to_string(), service_name.to_string());
        req = req.with_path_param("methodName".to_string(), method_name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_login_v1_session(
        &self,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/lol-login/v1/session".to_string(),
        );
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_login_v1_shutdown_locks_by_lock_name(
        &self,
        lock_name: &str,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Delete,
            "/lol-login/v1/shutdown-locks/{lockName}".to_string(),
        );
        req = req.with_path_param("lockName".to_string(), lock_name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn get_lol_login_v1_account_state(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolLoginAccountStateResource,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-login/v1/account-state".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_login_v1_login_connection_state(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolLoginLoginConnectionState,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-login/v1/login-connection-state".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_login_v1_login_data_packet(
        &self,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-login/v1/login-data-packet".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_login_v1_login_in_game_creds(
        &self,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-login/v1/login-in-game-creds".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_login_v1_login_platform_credentials(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolLoginPlatformGeneratedCredentials,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-login/v1/login-platform-credentials".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_login_v1_login_queue_state(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LolLoginLoginQueue, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-login/v1/login-queue-state".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_login_v1_session(
        &self,
    ) -> Box<dyn Future<Item = crate::models::LolLoginLoginSession, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-login/v1/session".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_login_v1_wallet(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolLoginLoginSessionWallet,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-login/v1/wallet".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn get_lol_login_v2_league_session_init_token(
        &self,
    ) -> Box<
        dyn Future<
            Item = crate::models::LolLoginLeagueSessionTokenEnvelope,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Get,
            "/lol-login/v2/league-session-init-token".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn post_lol_login_v1_account_state(
        &self,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-login/v1/account-state".to_string(),
        );
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_login_v1_change_summoner_name(
        &self,
        name: &str,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-login/v1/change-summoner-name".to_string(),
        );
        req = req.with_body_param(name);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_login_v1_delete_rso_on_close(
        &self,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-login/v1/delete-rso-on-close".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn post_lol_login_v1_league_session_status(
        &self,
        league_session: &str,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-login/v1/leagueSessionStatus".to_string(),
        );
        req = req.with_body_param(league_session);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_login_v1_new_player_flow_completed(
        &self,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-login/v1/new-player-flow-completed".to_string(),
        );

        req.execute(self.configuration.borrow())
    }

    fn post_lol_login_v1_service_proxy_async_requests_by_service_name_by_method_name(
        &self,
        service_name: &str,
        method_name: &str,
        plugin_id: i32,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-login/v1/service-proxy-async-requests/{serviceName}/{methodName}".to_string(),
        );
        req = req.with_query_param("pluginId".to_string(), plugin_id.to_string());
        req = req.with_path_param("serviceName".to_string(), service_name.to_string());
        req = req.with_path_param("methodName".to_string(), method_name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_login_v1_service_proxy_uuid_requests(
        &self,
        service_name: &str,
        method_name: &str,
        plugin_id: i32,
        timeout_millis: i64,
        payload: &str,
    ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-login/v1/service-proxy-uuid-requests".to_string(),
        );
        req = req.with_query_param("serviceName".to_string(), service_name.to_string());
        req = req.with_query_param("methodName".to_string(), method_name.to_string());
        req = req.with_query_param("pluginId".to_string(), plugin_id.to_string());
        req = req.with_query_param("timeoutMillis".to_string(), timeout_millis.to_string());
        req = req.with_query_param("payload".to_string(), payload.to_string());

        req.execute(self.configuration.borrow())
    }

    fn post_lol_login_v1_session(
        &self,
        username_and_password: crate::models::LolLoginUsernameAndPassword,
    ) -> Box<dyn Future<Item = crate::models::LolLoginLoginSession, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-login/v1/session".to_string(),
        );
        req = req.with_body_param(username_and_password);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_login_v1_session_invoke(
        &self,
        destination: &str,
        method: &str,
        UNKNOWN_BASE_TYPE: crate::models::UNKNOWN_BASE_TYPE,
    ) -> Box<dyn Future<Item = crate::models::LolLoginLcdsResponse, Error = Error<serde_json::Value>>>
    {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-login/v1/session/invoke".to_string(),
        );
        req = req.with_query_param("destination".to_string(), destination.to_string());
        req = req.with_query_param("method".to_string(), method.to_string());
        req = req.with_body_param(UNKNOWN_BASE_TYPE);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_login_v1_summoner_created(
        &self,
        summoner_id: crate::models::LolLoginSummonerCreatedResource,
    ) -> Box<
        dyn Future<
            Item = ::std::collections::HashMap<String, serde_json::Value>,
            Error = Error<serde_json::Value>,
        >,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::Post,
            "/lol-login/v1/summoner-created".to_string(),
        );
        req = req.with_body_param(summoner_id);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_login_v1_shutdown_locks_by_lock_name(
        &self,
        lock_name: &str,
    ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::Put,
            "/lol-login/v1/shutdown-locks/{lockName}".to_string(),
        );
        req = req.with_path_param("lockName".to_string(), lock_name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }
}
