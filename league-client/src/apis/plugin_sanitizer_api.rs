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

pub struct PluginSanitizerApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginSanitizerApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginSanitizerApiClient<C> {
        PluginSanitizerApiClient {
            configuration,
        }
    }
}

pub trait PluginSanitizerApi {
    fn get_sanitizer_v1_status(&self, ) -> Box<dyn Future<Item = crate::models::SanitizerSanitizerStatus, Error = Error<serde_json::Value>>>;
    fn post_sanitizer_v1_contains_sanitized(&self, request: crate::models::SanitizerContainsSanitizedRequest) -> Box<dyn Future<Item = crate::models::SanitizerContainsSanitizedResponse, Error = Error<serde_json::Value>>>;
    fn post_sanitizer_v1_sanitize(&self, request: crate::models::SanitizerSanitizeRequest) -> Box<dyn Future<Item = crate::models::SanitizerSanitizeResponse, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginSanitizerApi for PluginSanitizerApiClient<C> {
    fn get_sanitizer_v1_status(&self, ) -> Box<dyn Future<Item = crate::models::SanitizerSanitizerStatus, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/sanitizer/v1/status".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn post_sanitizer_v1_contains_sanitized(&self, request: crate::models::SanitizerContainsSanitizedRequest) -> Box<dyn Future<Item = crate::models::SanitizerContainsSanitizedResponse, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/sanitizer/v1/containsSanitized".to_string())
        ;
        req = req.with_body_param(request);

        req.execute(self.configuration.borrow())
    }

    fn post_sanitizer_v1_sanitize(&self, request: crate::models::SanitizerSanitizeRequest) -> Box<dyn Future<Item = crate::models::SanitizerSanitizeResponse, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/sanitizer/v1/sanitize".to_string())
        ;
        req = req.with_body_param(request);

        req.execute(self.configuration.borrow())
    }

}