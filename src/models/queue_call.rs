/*
 * Phone and Queue Backend
 *
 * API for managing phone services  This is the API of the service at P7M that manages phone services.  **Attention:** this API will probably still change a lot in the future, it's not at all stable yet
 *
 * The version of the OpenAPI document: 0.6.0
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueCall {
    #[serde(rename = "state")]
    pub state: models::CallState,
    #[serde(rename = "caller_id_num", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub caller_id_num: Option<Option<String>>,
    #[serde(rename = "caller_id_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub caller_id_name: Option<Option<String>>,
    #[serde(rename = "call_time")]
    pub call_time: String,
    #[serde(rename = "wait_time")]
    pub wait_time: String,
    #[serde(rename = "talk_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub talk_time: Option<Option<String>>,
    #[serde(rename = "priority", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Option<i64>>,
    #[serde(rename = "unique_id")]
    pub unique_id: String,
    #[serde(rename = "queue_pos", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub queue_pos: Option<Option<i64>>,
    #[serde(rename = "from_caller_jitter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_caller_jitter: Option<Option<String>>,
    #[serde(rename = "from_caller_cumulative_lost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_caller_cumulative_lost: Option<Option<String>>,
    #[serde(rename = "from_caller_fraction_lost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_caller_fraction_lost: Option<Option<String>>,
    #[serde(rename = "to_caller_jitter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to_caller_jitter: Option<Option<String>>,
    #[serde(rename = "to_caller_cumulative_lost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to_caller_cumulative_lost: Option<Option<String>>,
    #[serde(rename = "to_caller_fraction_lost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to_caller_fraction_lost: Option<Option<String>>,
    #[serde(rename = "from_agent_jitter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_agent_jitter: Option<Option<String>>,
    #[serde(rename = "from_agent_cumulative_lost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_agent_cumulative_lost: Option<Option<String>>,
    #[serde(rename = "from_agent_fraction_lost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from_agent_fraction_lost: Option<Option<String>>,
    #[serde(rename = "to_agent_jitter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to_agent_jitter: Option<Option<String>>,
    #[serde(rename = "to_agent_cumulative_lost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to_agent_cumulative_lost: Option<Option<String>>,
    #[serde(rename = "to_agent_fraction_lost", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to_agent_fraction_lost: Option<Option<String>>,
}

impl QueueCall {
    pub fn new(state: models::CallState, call_time: String, wait_time: String, unique_id: String) -> QueueCall {
        QueueCall {
            state,
            caller_id_num: None,
            caller_id_name: None,
            call_time,
            wait_time,
            talk_time: None,
            priority: None,
            unique_id,
            queue_pos: None,
            from_caller_jitter: None,
            from_caller_cumulative_lost: None,
            from_caller_fraction_lost: None,
            to_caller_jitter: None,
            to_caller_cumulative_lost: None,
            to_caller_fraction_lost: None,
            from_agent_jitter: None,
            from_agent_cumulative_lost: None,
            from_agent_fraction_lost: None,
            to_agent_jitter: None,
            to_agent_cumulative_lost: None,
            to_agent_fraction_lost: None,
        }
    }
}

