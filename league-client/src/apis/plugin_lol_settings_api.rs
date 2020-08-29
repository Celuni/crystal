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

pub struct PluginLolSettingsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolSettingsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolSettingsApiClient<C> {
        PluginLolSettingsApiClient {
            configuration,
        }
    }
}

pub trait PluginLolSettingsApi {
    fn get_lol_settings_v1_account_by_category(&self, category: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_lol_settings_v1_account_didreset(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_settings_v1_local_by_category(&self, category: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_lol_settings_v2_account_by_pp_type_by_category(&self, pp_type: &str, category: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_lol_settings_v2_didreset_by_pp_type(&self, pp_type: &str) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_settings_v2_local_by_category(&self, category: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_lol_settings_v2_ready(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn patch_lol_settings_v1_account_by_category(&self, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn patch_lol_settings_v1_local_by_category(&self, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn patch_lol_settings_v2_account_by_pp_type_by_category(&self, pp_type: &str, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn patch_lol_settings_v2_local_by_category(&self, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_settings_v1_account_save(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn put_lol_settings_v1_account_by_category(&self, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn put_lol_settings_v2_account_by_pp_type_by_category(&self, pp_type: &str, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolSettingsApi for PluginLolSettingsApiClient<C> {
    fn get_lol_settings_v1_account_by_category(&self, category: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-settings/v1/account/{category}".to_string())
        ;
        req = req.with_path_param("category".to_string(), category.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_settings_v1_account_didreset(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-settings/v1/account/didreset".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_settings_v1_local_by_category(&self, category: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-settings/v1/local/{category}".to_string())
        ;
        req = req.with_path_param("category".to_string(), category.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_settings_v2_account_by_pp_type_by_category(&self, pp_type: &str, category: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-settings/v2/account/{ppType}/{category}".to_string())
        ;
        req = req.with_path_param("ppType".to_string(), pp_type.to_string());
        req = req.with_path_param("category".to_string(), category.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_settings_v2_didreset_by_pp_type(&self, pp_type: &str) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-settings/v2/didreset/{ppType}".to_string())
        ;
        req = req.with_path_param("ppType".to_string(), pp_type.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_settings_v2_local_by_category(&self, category: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-settings/v2/local/{category}".to_string())
        ;
        req = req.with_path_param("category".to_string(), category.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_settings_v2_ready(&self, ) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-settings/v2/ready".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn patch_lol_settings_v1_account_by_category(&self, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/lol-settings/v1/account/{category}".to_string())
        ;
        req = req.with_path_param("category".to_string(), category.to_string());
        req = req.with_body_param(settings_resource);

        req.execute(self.configuration.borrow())
    }

    fn patch_lol_settings_v1_local_by_category(&self, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/lol-settings/v1/local/{category}".to_string())
        ;
        req = req.with_path_param("category".to_string(), category.to_string());
        req = req.with_body_param(settings_resource);

        req.execute(self.configuration.borrow())
    }

    fn patch_lol_settings_v2_account_by_pp_type_by_category(&self, pp_type: &str, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/lol-settings/v2/account/{ppType}/{category}".to_string())
        ;
        req = req.with_path_param("ppType".to_string(), pp_type.to_string());
        req = req.with_path_param("category".to_string(), category.to_string());
        req = req.with_body_param(settings_resource);

        req.execute(self.configuration.borrow())
    }

    fn patch_lol_settings_v2_local_by_category(&self, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Patch, "/lol-settings/v2/local/{category}".to_string())
        ;
        req = req.with_path_param("category".to_string(), category.to_string());
        req = req.with_body_param(settings_resource);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_settings_v1_account_save(&self, ) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-settings/v1/account/save".to_string())
        ;
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn put_lol_settings_v1_account_by_category(&self, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-settings/v1/account/{category}".to_string())
        ;
        req = req.with_path_param("category".to_string(), category.to_string());
        req = req.with_body_param(settings_resource);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_settings_v2_account_by_pp_type_by_category(&self, pp_type: &str, category: &str, settings_resource: crate::models::LolSettingsSettingCategory) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-settings/v2/account/{ppType}/{category}".to_string())
        ;
        req = req.with_path_param("ppType".to_string(), pp_type.to_string());
        req = req.with_path_param("category".to_string(), category.to_string());
        req = req.with_body_param(settings_resource);

        req.execute(self.configuration.borrow())
    }

}
