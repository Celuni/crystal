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

pub struct PluginLolLoadoutsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolLoadoutsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolLoadoutsApiClient<C> {
        PluginLolLoadoutsApiClient {
            configuration,
        }
    }
}

pub trait PluginLolLoadoutsApi {
    fn delete_lol_loadouts_v4_loadouts_by_id(&self, id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_lol_loadouts_v1_loadouts_ready(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_loadouts_v4_loadouts_by_loadout_id(&self, loadout_id: &str) -> Box<dyn Future<Item = crate::models::LolLoadoutsScopedLoadout, Error = Error<serde_json::Value>>>;
    fn get_lol_loadouts_v4_loadouts_scope_account(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolLoadoutsScopedLoadout>, Error = Error<serde_json::Value>>>;
    fn get_lol_loadouts_v4_loadouts_scope_by_scope_by_scope_item_id(&self, scope: &str, scope_item_id: i32) -> Box<dyn Future<Item = Vec<crate::models::LolLoadoutsScopedLoadout>, Error = Error<serde_json::Value>>>;
    fn patch_lol_loadouts_v4_loadouts_by_id(&self, id: &str, loadout: crate::models::LolLoadoutsUpdateLoadoutDto) -> Box<dyn Future<Item = crate::models::LolLoadoutsScopedLoadout, Error = Error<serde_json::Value>>>;
    fn post_lol_loadouts_v4_loadouts(&self, loadout: crate::models::LolLoadoutsCreateLoadoutDto) -> Box<dyn Future<Item = crate::models::LolLoadoutsScopedLoadout, Error = Error<serde_json::Value>>>;
    fn put_lol_loadouts_v4_loadouts_by_id(&self, id: &str, loadout: crate::models::LolLoadoutsUpdateLoadoutDto) -> Box<dyn Future<Item = crate::models::LolLoadoutsScopedLoadout, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolLoadoutsApi for PluginLolLoadoutsApiClient<C> {
    fn delete_lol_loadouts_v4_loadouts_by_id(&self, id: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-loadouts/v4/loadouts/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn get_lol_loadouts_v1_loadouts_ready(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-loadouts/v1/loadouts-ready".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_loadouts_v4_loadouts_by_loadout_id(&self, loadout_id: &str) -> Box<dyn Future<Item = crate::models::LolLoadoutsScopedLoadout, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-loadouts/v4/loadouts/{loadoutId}".to_string())
        ;
        req = req.with_path_param("loadoutId".to_string(), loadout_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_loadouts_v4_loadouts_scope_account(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolLoadoutsScopedLoadout>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-loadouts/v4/loadouts/scope/account".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_loadouts_v4_loadouts_scope_by_scope_by_scope_item_id(&self, scope: &str, scope_item_id: i32) -> Box<dyn Future<Item = Vec<crate::models::LolLoadoutsScopedLoadout>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-loadouts/v4/loadouts/scope/{scope}/{scopeItemId}".to_string())
        ;
        req = req.with_path_param("scope".to_string(), scope.to_string());
        req = req.with_path_param("scopeItemId".to_string(), scope_item_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn patch_lol_loadouts_v4_loadouts_by_id(&self, id: &str, loadout: crate::models::LolLoadoutsUpdateLoadoutDto) -> Box<dyn Future<Item = crate::models::LolLoadoutsScopedLoadout, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/lol-loadouts/v4/loadouts/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(loadout);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_loadouts_v4_loadouts(&self, loadout: crate::models::LolLoadoutsCreateLoadoutDto) -> Box<dyn Future<Item = crate::models::LolLoadoutsScopedLoadout, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-loadouts/v4/loadouts".to_string())
        ;
        req = req.with_body_param(loadout);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_loadouts_v4_loadouts_by_id(&self, id: &str, loadout: crate::models::LolLoadoutsUpdateLoadoutDto) -> Box<dyn Future<Item = crate::models::LolLoadoutsScopedLoadout, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-loadouts/v4/loadouts/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(loadout);

        req.execute(self.configuration.borrow())
    }

}