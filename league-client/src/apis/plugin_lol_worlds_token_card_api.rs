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

pub struct PluginLolWorldsTokenCardApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolWorldsTokenCardApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolWorldsTokenCardApiClient<C> {
        PluginLolWorldsTokenCardApiClient {
            configuration,
        }
    }
}

pub trait PluginLolWorldsTokenCardApi {
    fn get_lol_token_upsell_v1_all(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolWorldsTokenCardTokenUpsell>, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolWorldsTokenCardApi for PluginLolWorldsTokenCardApiClient<C> {
    fn get_lol_token_upsell_v1_all(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolWorldsTokenCardTokenUpsell>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-token-upsell/v1/all".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

}
