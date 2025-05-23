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

/// Queue : Main definition of a waiting queue
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Queue {
    /// ID of this queue
    #[serde(rename = "queueId")]
    pub queue_id: uuid::Uuid,
    /// The tenant this queue belongs to
    #[serde(rename = "tenantId")]
    pub tenant_id: uuid::Uuid,
    /// Legacy ID of this queue
    #[serde(rename = "queueToken")]
    pub queue_token: String,
    /// Name descripting this queue
    #[serde(rename = "name")]
    pub name: String,
    /// The time zone used for calculations regarding this queue (e.g. for opening hours)
    #[serde(rename = "timezone")]
    pub timezone: String,
    /// The numbers a caller can use to reach to this queue
    #[serde(rename = "numbers")]
    pub numbers: Vec<models::QueueNumber>,
    /// The phones that get called when there is a caller in this queue
    #[serde(rename = "destinations")]
    pub destinations: Vec<models::QueueDestination>,
    /// Legacy data on the priorities (see 'numbers' now)
    #[serde(rename = "prio")]
    pub prio: Vec<models::QueuePrio>,
    /// When this queue has been updated the last time
    #[serde(rename = "lastChange")]
    pub last_change: String,
    /// The holiday calender being used by this queue
    #[serde(rename = "holidayRegionId")]
    pub holiday_region_id: uuid::Uuid,
}

impl Queue {
    /// Main definition of a waiting queue
    pub fn new(queue_id: uuid::Uuid, tenant_id: uuid::Uuid, queue_token: String, name: String, timezone: String, numbers: Vec<models::QueueNumber>, destinations: Vec<models::QueueDestination>, prio: Vec<models::QueuePrio>, last_change: String, holiday_region_id: uuid::Uuid) -> Queue {
        Queue {
            queue_id,
            tenant_id,
            queue_token,
            name,
            timezone,
            numbers,
            destinations,
            prio,
            last_change,
            holiday_region_id,
        }
    }
}

