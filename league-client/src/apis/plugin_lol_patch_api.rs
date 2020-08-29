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

pub struct PluginLolPatchApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolPatchApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolPatchApiClient<C> {
        PluginLolPatchApiClient {
            configuration,
        }
    }
}

pub trait PluginLolPatchApi {
    fn delete_lol_patch_v1_notifications_by_id(&self, id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_lol_patch_v1_checking_enabled(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_patch_v1_environment(&self, ) -> Box<dyn Future<Item = crate::models::LolPatchChunkingPatcherEnvironment, Error = Error<serde_json::Value>>>;
    fn get_lol_patch_v1_game_version(&self, ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>>;
    fn get_lol_patch_v1_notifications(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolPatchNotification>, Error = Error<serde_json::Value>>>;
    fn get_lol_patch_v1_products_league_of_legends_install_location(&self, ) -> Box<dyn Future<Item = crate::models::LolPatchInstallPaths, Error = Error<serde_json::Value>>>;
    fn get_lol_patch_v1_products_league_of_legends_state(&self, ) -> Box<dyn Future<Item = crate::models::LolPatchProductState, Error = Error<serde_json::Value>>>;
    fn get_lol_patch_v1_products_league_of_legends_supported_game_releases(&self, ) -> Box<dyn Future<Item = crate::models::LolPatchSupportedGameReleases, Error = Error<serde_json::Value>>>;
    fn get_lol_patch_v1_status(&self, ) -> Box<dyn Future<Item = crate::models::LolPatchStatus, Error = Error<serde_json::Value>>>;
    fn post_lol_patch_v1_products_league_of_legends_detect_corruption_request(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_patch_v1_products_league_of_legends_partial_repair_request(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_patch_v1_products_league_of_legends_start_checking_request(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_patch_v1_products_league_of_legends_start_patching_request(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_patch_v1_products_league_of_legends_stop_checking_request(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn post_lol_patch_v1_products_league_of_legends_stop_patching_request(&self, restart: bool) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn put_lol_patch_v1_game_patch_url(&self, url: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn put_lol_patch_v1_self_update_restart(&self, force_restart_on_self_update: bool) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn put_lol_patch_v1_ux(&self, ux: crate::models::LolPatchUxResource) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolPatchApi for PluginLolPatchApiClient<C> {
    fn delete_lol_patch_v1_notifications_by_id(&self, id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-patch/v1/notifications/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn get_lol_patch_v1_checking_enabled(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-patch/v1/checking-enabled".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_patch_v1_environment(&self, ) -> Box<dyn Future<Item = crate::models::LolPatchChunkingPatcherEnvironment, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-patch/v1/environment".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_patch_v1_game_version(&self, ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-patch/v1/game-version".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_patch_v1_notifications(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolPatchNotification>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-patch/v1/notifications".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_patch_v1_products_league_of_legends_install_location(&self, ) -> Box<dyn Future<Item = crate::models::LolPatchInstallPaths, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-patch/v1/products/league_of_legends/install-location".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_patch_v1_products_league_of_legends_state(&self, ) -> Box<dyn Future<Item = crate::models::LolPatchProductState, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-patch/v1/products/league_of_legends/state".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_patch_v1_products_league_of_legends_supported_game_releases(&self, ) -> Box<dyn Future<Item = crate::models::LolPatchSupportedGameReleases, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-patch/v1/products/league_of_legends/supported-game-releases".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_patch_v1_status(&self, ) -> Box<dyn Future<Item = crate::models::LolPatchStatus, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-patch/v1/status".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_lol_patch_v1_products_league_of_legends_detect_corruption_request(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-patch/v1/products/league_of_legends/detect-corruption-request".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_patch_v1_products_league_of_legends_partial_repair_request(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-patch/v1/products/league_of_legends/partial-repair-request".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_patch_v1_products_league_of_legends_start_checking_request(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-patch/v1/products/league_of_legends/start-checking-request".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_patch_v1_products_league_of_legends_start_patching_request(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-patch/v1/products/league_of_legends/start-patching-request".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_patch_v1_products_league_of_legends_stop_checking_request(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-patch/v1/products/league_of_legends/stop-checking-request".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn post_lol_patch_v1_products_league_of_legends_stop_patching_request(&self, restart: bool) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-patch/v1/products/league_of_legends/stop-patching-request".to_string())
        ;
        req = req.with_query_param("restart".to_string(), restart.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn put_lol_patch_v1_game_patch_url(&self, url: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-patch/v1/game-patch-url".to_string())
        ;
        req = req.with_query_param("url".to_string(), url.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn put_lol_patch_v1_self_update_restart(&self, force_restart_on_self_update: bool) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-patch/v1/self-update-restart".to_string())
        ;
        req = req.with_query_param("forceRestartOnSelfUpdate".to_string(), force_restart_on_self_update.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn put_lol_patch_v1_ux(&self, ux: crate::models::LolPatchUxResource) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-patch/v1/ux".to_string())
        ;
        req = req.with_body_param(ux);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}
