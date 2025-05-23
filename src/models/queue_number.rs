/*
 * Phone and Queue Backend
 *
 * API for managing phone services  This is the API of the service at P7M that manages phone services.  **Attention:** this API will probably still change a lot in the future, it's not at all stable yet
 *
 * The version of the OpenAPI document: 0.7.0
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// QueueNumber : A phone number under which a queue can be reached
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueNumber {
    /// The ID representing this queue number
    #[serde(rename = "queueNumberId")]
    pub queue_number_id: uuid::Uuid,
    /// The tenant this number belongs to
    #[serde(rename = "tenantId")]
    pub tenant_id: uuid::Uuid,
    /// The phone number in E.164 format including a '+' prefix
    #[serde(rename = "number")]
    pub number: String,
    /// The queue this number belongs to
    #[serde(rename = "queueId")]
    pub queue_id: uuid::Uuid,
    /// The priority calls on this number get assigned by default
    #[serde(rename = "prio")]
    pub prio: i32,
    /// Set this as the name of the caller when ringing the destinations (empty to not force a caller name)
    #[serde(rename = "forcedCallerName")]
    pub forced_caller_name: String,
    /// When this number has been changed the last time
    #[serde(rename = "lastChange")]
    pub last_change: String,
}

impl QueueNumber {
    /// A phone number under which a queue can be reached
    pub fn new(queue_number_id: uuid::Uuid, tenant_id: uuid::Uuid, number: String, queue_id: uuid::Uuid, prio: i32, forced_caller_name: String, last_change: String) -> QueueNumber {
        QueueNumber {
            queue_number_id,
            tenant_id,
            number,
            queue_id,
            prio,
            forced_caller_name,
            last_change,
        }
    }
}

