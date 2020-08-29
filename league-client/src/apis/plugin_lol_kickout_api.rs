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

pub struct PluginLolKickoutApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolKickoutApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolKickoutApiClient<C> {
        PluginLolKickoutApiClient {
            configuration,
        }
    }
}

pub trait PluginLolKickoutApi {
    fn get_lol_kickout_v1_notification(&self, ) -> Box<dyn Future<Item = crate::models::LolKickoutKickoutMessage, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolKickoutApi for PluginLolKickoutApiClient<C> {
    fn get_lol_kickout_v1_notification(&self, ) -> Box<dyn Future<Item = crate::models::LolKickoutKickoutMessage, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-kickout/v1/notification".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}
