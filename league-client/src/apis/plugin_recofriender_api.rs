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

pub struct PluginRecofrienderApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginRecofrienderApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginRecofrienderApiClient<C> {
        PluginRecofrienderApiClient {
            configuration,
        }
    }
}

pub trait PluginRecofrienderApi {
    fn delete_recofriender_v1_registrations_by_network(&self, network: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn delete_recofriender_v2_contacts(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn delete_recofriender_v2_dismissed(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_recofriender_v1_config(&self, ) -> Box<dyn Future<Item = crate::models::RecofrienderConfig, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v1_config_by_network(&self, network: &str) -> Box<dyn Future<Item = crate::models::RecofrienderNetworkConfig, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v1_contacts(&self, account_id: Option<i64>, source: Option<&str>, friend_state: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::RecofrienderContactResource>, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v1_debug(&self, ) -> Box<dyn Future<Item = crate::models::RecofrienderDebugConfig, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v1_faq_url(&self, ) -> Box<dyn Future<Item = crate::models::RecofrienderUrlResource, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v1_registrations(&self, cb: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::RecofrienderLinkResource>, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v1_registrations_by_network(&self, network: &str) -> Box<dyn Future<Item = crate::models::RecofrienderLinkResource, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v2_contacts(&self, ) -> Box<dyn Future<Item = Vec<crate::models::RecofrienderContactResource>, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v2_contacts_by_account_id(&self, account_id: i64) -> Box<dyn Future<Item = crate::models::RecofrienderContactResource, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v2_contacts_page(&self, start: i64, limit: i64) -> Box<dyn Future<Item = crate::models::RecofrienderContactPaginationResource, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v2_dismissed(&self, ) -> Box<dyn Future<Item = Vec<crate::models::RecofrienderContactResource>, Error = Error<serde_json::Value>>>;
    fn get_recofriender_v2_dismissed_page(&self, start: i64, limit: i64) -> Box<dyn Future<Item = crate::models::RecofrienderContactPaginationResource, Error = Error<serde_json::Value>>>;
    fn post_recofriender_v1_contacts_by_account_id_available(&self, account_id: i64, retain_in_cache: Option<bool>) -> Box<dyn Future<Item = crate::models::RecofrienderContactStateResource, Error = Error<serde_json::Value>>>;
    fn post_recofriender_v1_contacts_by_account_id_dismissed(&self, account_id: i64, retain_in_cache: Option<bool>) -> Box<dyn Future<Item = crate::models::RecofrienderContactStateResource, Error = Error<serde_json::Value>>>;
    fn post_recofriender_v1_contacts_by_account_id_invited(&self, account_id: i64, retain_in_cache: Option<bool>) -> Box<dyn Future<Item = crate::models::RecofrienderContactStateResource, Error = Error<serde_json::Value>>>;
    fn post_recofriender_v1_registrations_by_network(&self, network: &str) -> Box<dyn Future<Item = crate::models::RecofrienderUrlResource, Error = Error<serde_json::Value>>>;
    fn put_recofriender_v1_debug(&self, debug_configuration: crate::models::RecofrienderDebugConfig) -> Box<dyn Future<Item = crate::models::RecofrienderDebugConfig, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginRecofrienderApi for PluginRecofrienderApiClient<C> {
    fn delete_recofriender_v1_registrations_by_network(&self, network: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/recofriender/v1/registrations/{network}".to_string())
        ;
        req = req.with_path_param("network".to_string(), network.to_string());

        req.execute(self.configuration.borrow())
    }

    fn delete_recofriender_v2_contacts(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/recofriender/v2/contacts".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn delete_recofriender_v2_dismissed(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/recofriender/v2/dismissed".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v1_config(&self, ) -> Box<dyn Future<Item = crate::models::RecofrienderConfig, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v1/config".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v1_config_by_network(&self, network: &str) -> Box<dyn Future<Item = crate::models::RecofrienderNetworkConfig, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v1/config/{network}".to_string())
        ;
        req = req.with_path_param("network".to_string(), network.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v1_contacts(&self, account_id: Option<i64>, source: Option<&str>, friend_state: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::RecofrienderContactResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v1/contacts".to_string())
        ;
        if let Some(ref s) = account_id {
            req = req.with_query_param("accountId".to_string(), s.to_string());
        }
        if let Some(ref s) = source {
            req = req.with_query_param("source".to_string(), s.to_string());
        }
        if let Some(ref s) = friend_state {
            req = req.with_query_param("friendState".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v1_debug(&self, ) -> Box<dyn Future<Item = crate::models::RecofrienderDebugConfig, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v1/debug".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v1_faq_url(&self, ) -> Box<dyn Future<Item = crate::models::RecofrienderUrlResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v1/faq-url".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v1_registrations(&self, cb: Option<&str>) -> Box<dyn Future<Item = Vec<crate::models::RecofrienderLinkResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v1/registrations".to_string())
        ;
        if let Some(ref s) = cb {
            req = req.with_query_param("cb".to_string(), s.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v1_registrations_by_network(&self, network: &str) -> Box<dyn Future<Item = crate::models::RecofrienderLinkResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v1/registrations/{network}".to_string())
        ;
        req = req.with_path_param("network".to_string(), network.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v2_contacts(&self, ) -> Box<dyn Future<Item = Vec<crate::models::RecofrienderContactResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v2/contacts".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v2_contacts_by_account_id(&self, account_id: i64) -> Box<dyn Future<Item = crate::models::RecofrienderContactResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v2/contacts/{accountId}".to_string())
        ;
        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v2_contacts_page(&self, start: i64, limit: i64) -> Box<dyn Future<Item = crate::models::RecofrienderContactPaginationResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v2/contacts/page".to_string())
        ;
        req = req.with_query_param("start".to_string(), start.to_string());
        req = req.with_query_param("limit".to_string(), limit.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v2_dismissed(&self, ) -> Box<dyn Future<Item = Vec<crate::models::RecofrienderContactResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v2/dismissed".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_recofriender_v2_dismissed_page(&self, start: i64, limit: i64) -> Box<dyn Future<Item = crate::models::RecofrienderContactPaginationResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/recofriender/v2/dismissed/page".to_string())
        ;
        req = req.with_query_param("start".to_string(), start.to_string());
        req = req.with_query_param("limit".to_string(), limit.to_string());

        req.execute(self.configuration.borrow())
    }

    fn post_recofriender_v1_contacts_by_account_id_available(&self, account_id: i64, retain_in_cache: Option<bool>) -> Box<dyn Future<Item = crate::models::RecofrienderContactStateResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/recofriender/v1/contacts/{accountId}/available".to_string())
        ;
        if let Some(ref s) = retain_in_cache {
            req = req.with_query_param("retainInCache".to_string(), s.to_string());
        }
        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn post_recofriender_v1_contacts_by_account_id_dismissed(&self, account_id: i64, retain_in_cache: Option<bool>) -> Box<dyn Future<Item = crate::models::RecofrienderContactStateResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/recofriender/v1/contacts/{accountId}/dismissed".to_string())
        ;
        if let Some(ref s) = retain_in_cache {
            req = req.with_query_param("retainInCache".to_string(), s.to_string());
        }
        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn post_recofriender_v1_contacts_by_account_id_invited(&self, account_id: i64, retain_in_cache: Option<bool>) -> Box<dyn Future<Item = crate::models::RecofrienderContactStateResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/recofriender/v1/contacts/{accountId}/invited".to_string())
        ;
        if let Some(ref s) = retain_in_cache {
            req = req.with_query_param("retainInCache".to_string(), s.to_string());
        }
        req = req.with_path_param("accountId".to_string(), account_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn post_recofriender_v1_registrations_by_network(&self, network: &str) -> Box<dyn Future<Item = crate::models::RecofrienderUrlResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/recofriender/v1/registrations/{network}".to_string())
        ;
        req = req.with_path_param("network".to_string(), network.to_string());

        req.execute(self.configuration.borrow())
    }

    fn put_recofriender_v1_debug(&self, debug_configuration: crate::models::RecofrienderDebugConfig) -> Box<dyn Future<Item = crate::models::RecofrienderDebugConfig, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/recofriender/v1/debug".to_string())
        ;
        req = req.with_body_param(debug_configuration);

        req.execute(self.configuration.borrow())
    }

}