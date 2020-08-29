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

pub struct PluginLolRegaliaApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolRegaliaApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolRegaliaApiClient<C> {
        PluginLolRegaliaApiClient {
            configuration,
        }
    }
}

pub trait PluginLolRegaliaApi {
    fn get_lol_regalia_v2_config(&self, ) -> Box<dyn Future<Item = crate::models::LolRegaliaRegaliaFrontendConfig, Error = Error<serde_json::Value>>>;
    fn get_lol_regalia_v2_current_summoner_regalia(&self, ) -> Box<dyn Future<Item = crate::models::LolRegaliaRegaliaWithPreferences, Error = Error<serde_json::Value>>>;
    fn get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_positions_by_position_regalia(&self, summoner_id: i64, queue: &str, position: &str) -> Box<dyn Future<Item = crate::models::LolRegaliaRegalia, Error = Error<serde_json::Value>>>;
    fn get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_regalia(&self, summoner_id: i64, queue: &str) -> Box<dyn Future<Item = crate::models::LolRegaliaRegalia, Error = Error<serde_json::Value>>>;
    fn get_lol_regalia_v2_summoners_by_summoner_id_regalia(&self, summoner_id: i64, hovercard: bool) -> Box<dyn Future<Item = crate::models::LolRegaliaRegalia, Error = Error<serde_json::Value>>>;
    fn get_lol_regalia_v2_summoners_by_summoner_id_regalia_async(&self, summoner_id: i64) -> Box<dyn Future<Item = crate::models::LolRegaliaRegaliaAsync, Error = Error<serde_json::Value>>>;
    fn put_lol_regalia_v2_current_summoner_regalia(&self, regalia: crate::models::LolRegaliaRegaliaPreferences) -> Box<dyn Future<Item = crate::models::LolRegaliaRegaliaWithPreferences, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolRegaliaApi for PluginLolRegaliaApiClient<C> {
    fn get_lol_regalia_v2_config(&self, ) -> Box<dyn Future<Item = crate::models::LolRegaliaRegaliaFrontendConfig, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-regalia/v2/config".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_regalia_v2_current_summoner_regalia(&self, ) -> Box<dyn Future<Item = crate::models::LolRegaliaRegaliaWithPreferences, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-regalia/v2/current-summoner/regalia".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_positions_by_position_regalia(&self, summoner_id: i64, queue: &str, position: &str) -> Box<dyn Future<Item = crate::models::LolRegaliaRegalia, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-regalia/v2/summoners/{summonerId}/queues/{queue}/positions/{position}/regalia".to_string())
        ;
        req = req.with_path_param("summonerId".to_string(), summoner_id.to_string());
        req = req.with_path_param("queue".to_string(), queue.to_string());
        req = req.with_path_param("position".to_string(), position.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_regalia_v2_summoners_by_summoner_id_queues_by_queue_regalia(&self, summoner_id: i64, queue: &str) -> Box<dyn Future<Item = crate::models::LolRegaliaRegalia, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-regalia/v2/summoners/{summonerId}/queues/{queue}/regalia".to_string())
        ;
        req = req.with_path_param("summonerId".to_string(), summoner_id.to_string());
        req = req.with_path_param("queue".to_string(), queue.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_regalia_v2_summoners_by_summoner_id_regalia(&self, summoner_id: i64, hovercard: bool) -> Box<dyn Future<Item = crate::models::LolRegaliaRegalia, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-regalia/v2/summoners/{summonerId}/regalia".to_string())
        ;
        req = req.with_query_param("hovercard".to_string(), hovercard.to_string());
        req = req.with_path_param("summonerId".to_string(), summoner_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_regalia_v2_summoners_by_summoner_id_regalia_async(&self, summoner_id: i64) -> Box<dyn Future<Item = crate::models::LolRegaliaRegaliaAsync, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-regalia/v2/summoners/{summonerId}/regalia/async".to_string())
        ;
        req = req.with_path_param("summonerId".to_string(), summoner_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn put_lol_regalia_v2_current_summoner_regalia(&self, regalia: crate::models::LolRegaliaRegaliaPreferences) -> Box<dyn Future<Item = crate::models::LolRegaliaRegaliaWithPreferences, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-regalia/v2/current-summoner/regalia".to_string())
        ;
        req = req.with_body_param(regalia);

        req.execute(self.configuration.borrow())
    }

}
