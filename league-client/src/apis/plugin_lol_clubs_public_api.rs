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

pub struct PluginLolClubsPublicApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolClubsPublicApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolClubsPublicApiClient<C> {
        PluginLolClubsPublicApiClient {
            configuration,
        }
    }
}

pub trait PluginLolClubsPublicApi {
    fn get_lol_clubs_public_v1_clubs_public(&self, summoner_names: &str) -> Box<dyn Future<Item = Vec<crate::models::LolClubsPublicClubsPublicData>, Error = Error<serde_json::Value>>>;
    fn get_lol_clubs_public_v1_clubs_public_by_summoner_id(&self, summoner_id: i64) -> Box<dyn Future<Item = crate::models::LolClubsPublicClubsPublicData, Error = Error<serde_json::Value>>>;
    fn get_lol_clubs_public_v1_clubs_public_by_summoner_id_tag(&self, summoner_id: i64) -> Box<dyn Future<Item = crate::models::LolClubsPublicClubTag, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolClubsPublicApi for PluginLolClubsPublicApiClient<C> {
    fn get_lol_clubs_public_v1_clubs_public(&self, summoner_names: &str) -> Box<dyn Future<Item = Vec<crate::models::LolClubsPublicClubsPublicData>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-clubs-public/v1/clubs/public".to_string())
        ;
        req = req.with_query_param("summonerNames".to_string(), summoner_names.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_clubs_public_v1_clubs_public_by_summoner_id(&self, summoner_id: i64) -> Box<dyn Future<Item = crate::models::LolClubsPublicClubsPublicData, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-clubs-public/v1/clubs/public/{summonerId}".to_string())
        ;
        req = req.with_path_param("summonerId".to_string(), summoner_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_clubs_public_v1_clubs_public_by_summoner_id_tag(&self, summoner_id: i64) -> Box<dyn Future<Item = crate::models::LolClubsPublicClubTag, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-clubs-public/v1/clubs/public/{summonerId}/tag".to_string())
        ;
        req = req.with_path_param("summonerId".to_string(), summoner_id.to_string());

        req.execute(self.configuration.borrow())
    }

}
