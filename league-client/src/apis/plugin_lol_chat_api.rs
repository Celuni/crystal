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

pub struct PluginLolChatApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PluginLolChatApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PluginLolChatApiClient<C> {
        PluginLolChatApiClient {
            configuration,
        }
    }
}

pub trait PluginLolChatApi {
    fn delete_lol_chat_v1_blocked_players_by_id(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn delete_lol_chat_v1_conversations_active(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn delete_lol_chat_v1_conversations_by_id(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn delete_lol_chat_v1_conversations_by_id_messages(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn delete_lol_chat_v1_errors_by_id(&self, id: i64) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn delete_lol_chat_v1_friend_groups_by_id(&self, id: i32) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn delete_lol_chat_v1_friend_requests_by_id(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn delete_lol_chat_v1_friends_by_id(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn delete_lol_chat_v1_session(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn delete_lol_chat_v1_settings_by_key(&self, key: &str, do_async: Option<bool>) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_blocked_players(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatBlockedPlayerResource>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_blocked_players_by_id(&self, id: &str) -> Box<dyn Future<Item = crate::models::LolChatBlockedPlayerResource, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_config(&self, ) -> Box<dyn Future<Item = crate::models::LolChatChatServiceDynamicClientConfig, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_conversations(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatConversationResource>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_conversations_active(&self, ) -> Box<dyn Future<Item = crate::models::LolChatActiveConversationResource, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_conversations_by_id(&self, id: &str) -> Box<dyn Future<Item = crate::models::LolChatConversationResource, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_conversations_by_id_messages(&self, id: &str) -> Box<dyn Future<Item = Vec<crate::models::LolChatConversationMessageResource>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_conversations_by_id_participants(&self, id: &str) -> Box<dyn Future<Item = Vec<crate::models::LolChatUserResource>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_conversations_by_id_participants_by_pid(&self, id: &str, pid: &str) -> Box<dyn Future<Item = crate::models::LolChatUserResource, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_conversations_notify(&self, ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_errors(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatErrorResource>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_friend_counts(&self, ) -> Box<dyn Future<Item = crate::models::LolChatFriendCountsResource, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_friend_exists_by_summoner_id(&self, summoner_id: i64) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_friend_groups(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatGroupResource>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_friend_groups_by_id(&self, id: i32) -> Box<dyn Future<Item = crate::models::LolChatGroupResource, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_friend_groups_by_id_friends(&self, id: i32) -> Box<dyn Future<Item = Vec<crate::models::LolChatFriendResource>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_friend_requests(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatFriendRequestResource>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_friends(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatFriendResource>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_friends_by_id(&self, id: &str) -> Box<dyn Future<Item = crate::models::LolChatFriendResource, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_me(&self, ) -> Box<dyn Future<Item = crate::models::LolChatUserResource, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_resources(&self, ) -> Box<dyn Future<Item = crate::models::LolChatProductMetadataMap, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_session(&self, ) -> Box<dyn Future<Item = crate::models::LolChatSessionResource, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_settings(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn get_lol_chat_v1_settings_by_key(&self, key: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_chat_v1_blocked_players(&self, blocked: crate::models::LolChatBlockedPlayerResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_chat_v1_conversations(&self, conversation: crate::models::LolChatConversationResource) -> Box<dyn Future<Item = crate::models::LolChatConversationResource, Error = Error<serde_json::Value>>>;
    fn post_lol_chat_v1_conversations_by_id_closed(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_chat_v1_conversations_by_id_messages(&self, id: &str, body: crate::models::LolChatConversationMessageResource) -> Box<dyn Future<Item = crate::models::LolChatConversationMessageResource, Error = Error<serde_json::Value>>>;
    fn post_lol_chat_v1_conversations_by_id_participants(&self, id: &str, invitee: crate::models::LolChatUserResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_chat_v1_friend_groups(&self, group: crate::models::LolChatGroupResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_chat_v1_friend_requests(&self, request: crate::models::LolChatFriendRequestResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn post_lol_chat_v1_session_plain(&self, auth: crate::models::LolChatAuthResourcePlain) -> Box<dyn Future<Item = crate::models::LolChatSessionResource, Error = Error<serde_json::Value>>>;
    fn post_lol_chat_v1_session_rso(&self, auth: crate::models::LolChatAuthResourceRsoAccessToken) -> Box<dyn Future<Item = crate::models::LolChatSessionResource, Error = Error<serde_json::Value>>>;
    fn put_lol_chat_v1_conversations_active(&self, active_conversation: crate::models::LolChatActiveConversationResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn put_lol_chat_v1_conversations_by_id(&self, id: &str, updated_conversation: crate::models::LolChatConversationResource) -> Box<dyn Future<Item = crate::models::LolChatConversationResource, Error = Error<serde_json::Value>>>;
    fn put_lol_chat_v1_conversations_by_id_closed(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn put_lol_chat_v1_friend_groups_by_id(&self, id: i32, group: crate::models::LolChatGroupResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn put_lol_chat_v1_friend_groups_order(&self, order: crate::models::LolChatFriendGroupOrder) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn put_lol_chat_v1_friend_requests_by_id(&self, id: &str, request: crate::models::LolChatFriendRequestResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn put_lol_chat_v1_friends_by_id(&self, id: &str, contact: crate::models::LolChatFriendResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn put_lol_chat_v1_me(&self, me: crate::models::LolChatUserResource) -> Box<dyn Future<Item = crate::models::LolChatUserResource, Error = Error<serde_json::Value>>>;
    fn put_lol_chat_v1_settings(&self, data: serde_json::Value, do_async: Option<bool>) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
    fn put_lol_chat_v1_settings_by_key(&self, key: &str, value: serde_json::Value, do_async: Option<bool>) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>PluginLolChatApi for PluginLolChatApiClient<C> {
    fn delete_lol_chat_v1_blocked_players_by_id(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-chat/v1/blocked-players/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_chat_v1_conversations_active(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-chat/v1/conversations/active".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_chat_v1_conversations_by_id(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-chat/v1/conversations/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_chat_v1_conversations_by_id_messages(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-chat/v1/conversations/{id}/messages".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_chat_v1_errors_by_id(&self, id: i64) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-chat/v1/errors/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_chat_v1_friend_groups_by_id(&self, id: i32) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-chat/v1/friend-groups/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_chat_v1_friend_requests_by_id(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-chat/v1/friend-requests/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_chat_v1_friends_by_id(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-chat/v1/friends/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_chat_v1_session(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-chat/v1/session".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn delete_lol_chat_v1_settings_by_key(&self, key: &str, do_async: Option<bool>) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/lol-chat/v1/settings/{key}".to_string())
        ;
        if let Some(ref s) = do_async {
            req = req.with_query_param("doAsync".to_string(), s.to_string());
        }
        req = req.with_path_param("key".to_string(), key.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_blocked_players(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatBlockedPlayerResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/blocked-players".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_blocked_players_by_id(&self, id: &str) -> Box<dyn Future<Item = crate::models::LolChatBlockedPlayerResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/blocked-players/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_config(&self, ) -> Box<dyn Future<Item = crate::models::LolChatChatServiceDynamicClientConfig, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/config".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_conversations(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatConversationResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/conversations".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_conversations_active(&self, ) -> Box<dyn Future<Item = crate::models::LolChatActiveConversationResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/conversations/active".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_conversations_by_id(&self, id: &str) -> Box<dyn Future<Item = crate::models::LolChatConversationResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/conversations/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_conversations_by_id_messages(&self, id: &str) -> Box<dyn Future<Item = Vec<crate::models::LolChatConversationMessageResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/conversations/{id}/messages".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_conversations_by_id_participants(&self, id: &str) -> Box<dyn Future<Item = Vec<crate::models::LolChatUserResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/conversations/{id}/participants".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_conversations_by_id_participants_by_pid(&self, id: &str, pid: &str) -> Box<dyn Future<Item = crate::models::LolChatUserResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/conversations/{id}/participants/{pid}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_path_param("pid".to_string(), pid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_conversations_notify(&self, ) -> Box<dyn Future<Item = String, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/conversations/notify".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_errors(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatErrorResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/errors".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_friend_counts(&self, ) -> Box<dyn Future<Item = crate::models::LolChatFriendCountsResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/friend-counts".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_friend_exists_by_summoner_id(&self, summoner_id: i64) -> Box<dyn Future<Item = bool, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/friend-exists/{summonerId}".to_string())
        ;
        req = req.with_path_param("summonerId".to_string(), summoner_id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_friend_groups(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatGroupResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/friend-groups".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_friend_groups_by_id(&self, id: i32) -> Box<dyn Future<Item = crate::models::LolChatGroupResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/friend-groups/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_friend_groups_by_id_friends(&self, id: i32) -> Box<dyn Future<Item = Vec<crate::models::LolChatFriendResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/friend-groups/{id}/friends".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_friend_requests(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatFriendRequestResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/friend-requests".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_friends(&self, ) -> Box<dyn Future<Item = Vec<crate::models::LolChatFriendResource>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/friends".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_friends_by_id(&self, id: &str) -> Box<dyn Future<Item = crate::models::LolChatFriendResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/friends/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_me(&self, ) -> Box<dyn Future<Item = crate::models::LolChatUserResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/me".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_resources(&self, ) -> Box<dyn Future<Item = crate::models::LolChatProductMetadataMap, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/resources".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_session(&self, ) -> Box<dyn Future<Item = crate::models::LolChatSessionResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/session".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_settings(&self, ) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/settings".to_string())
        ;

        req.execute(self.configuration.borrow())
    }

    fn get_lol_chat_v1_settings_by_key(&self, key: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/lol-chat/v1/settings/{key}".to_string())
        ;
        req = req.with_path_param("key".to_string(), key.to_string());

        req.execute(self.configuration.borrow())
    }

    fn post_lol_chat_v1_blocked_players(&self, blocked: crate::models::LolChatBlockedPlayerResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-chat/v1/blocked-players".to_string())
        ;
        req = req.with_body_param(blocked);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_chat_v1_conversations(&self, conversation: crate::models::LolChatConversationResource) -> Box<dyn Future<Item = crate::models::LolChatConversationResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-chat/v1/conversations".to_string())
        ;
        req = req.with_body_param(conversation);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_chat_v1_conversations_by_id_closed(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-chat/v1/conversations/{id}/closed".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn post_lol_chat_v1_conversations_by_id_messages(&self, id: &str, body: crate::models::LolChatConversationMessageResource) -> Box<dyn Future<Item = crate::models::LolChatConversationMessageResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-chat/v1/conversations/{id}/messages".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(body);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_chat_v1_conversations_by_id_participants(&self, id: &str, invitee: crate::models::LolChatUserResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-chat/v1/conversations/{id}/participants".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(invitee);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_chat_v1_friend_groups(&self, group: crate::models::LolChatGroupResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-chat/v1/friend-groups".to_string())
        ;
        req = req.with_body_param(group);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_chat_v1_friend_requests(&self, request: crate::models::LolChatFriendRequestResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-chat/v1/friend-requests".to_string())
        ;
        req = req.with_body_param(request);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_chat_v1_session_plain(&self, auth: crate::models::LolChatAuthResourcePlain) -> Box<dyn Future<Item = crate::models::LolChatSessionResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-chat/v1/session/plain".to_string())
        ;
        req = req.with_body_param(auth);

        req.execute(self.configuration.borrow())
    }

    fn post_lol_chat_v1_session_rso(&self, auth: crate::models::LolChatAuthResourceRsoAccessToken) -> Box<dyn Future<Item = crate::models::LolChatSessionResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/lol-chat/v1/session/rso".to_string())
        ;
        req = req.with_body_param(auth);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_chat_v1_conversations_active(&self, active_conversation: crate::models::LolChatActiveConversationResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-chat/v1/conversations/active".to_string())
        ;
        req = req.with_body_param(active_conversation);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_chat_v1_conversations_by_id(&self, id: &str, updated_conversation: crate::models::LolChatConversationResource) -> Box<dyn Future<Item = crate::models::LolChatConversationResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-chat/v1/conversations/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(updated_conversation);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_chat_v1_conversations_by_id_closed(&self, id: &str) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-chat/v1/conversations/{id}/closed".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());

        req.execute(self.configuration.borrow())
    }

    fn put_lol_chat_v1_friend_groups_by_id(&self, id: i32, group: crate::models::LolChatGroupResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-chat/v1/friend-groups/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(group);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_chat_v1_friend_groups_order(&self, order: crate::models::LolChatFriendGroupOrder) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-chat/v1/friend-groups/order".to_string())
        ;
        req = req.with_body_param(order);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_chat_v1_friend_requests_by_id(&self, id: &str, request: crate::models::LolChatFriendRequestResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-chat/v1/friend-requests/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(request);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_chat_v1_friends_by_id(&self, id: &str, contact: crate::models::LolChatFriendResource) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-chat/v1/friends/{id}".to_string())
        ;
        req = req.with_path_param("id".to_string(), id.to_string());
        req = req.with_body_param(contact);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_chat_v1_me(&self, me: crate::models::LolChatUserResource) -> Box<dyn Future<Item = crate::models::LolChatUserResource, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-chat/v1/me".to_string())
        ;
        req = req.with_body_param(me);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_chat_v1_settings(&self, data: serde_json::Value, do_async: Option<bool>) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-chat/v1/settings".to_string())
        ;
        if let Some(ref s) = do_async {
            req = req.with_query_param("doAsync".to_string(), s.to_string());
        }
        req = req.with_body_param(data);

        req.execute(self.configuration.borrow())
    }

    fn put_lol_chat_v1_settings_by_key(&self, key: &str, value: serde_json::Value, do_async: Option<bool>) -> Box<dyn Future<Item = ::std::collections::HashMap<String, serde_json::Value>, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Put, "/lol-chat/v1/settings/{key}".to_string())
        ;
        if let Some(ref s) = do_async {
            req = req.with_query_param("doAsync".to_string(), s.to_string());
        }
        req = req.with_path_param("key".to_string(), key.to_string());
        req = req.with_body_param(value);

        req.execute(self.configuration.borrow())
    }

}
