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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueueOpening {
    #[serde(rename = "queueOpeningId")]
    pub queue_opening_id: uuid::Uuid,
    #[serde(rename = "tenantId")]
    pub tenant_id: uuid::Uuid,
    #[serde(rename = "queueId")]
    pub queue_id: uuid::Uuid,
    #[serde(rename = "weekday")]
    pub weekday: models::WeekdayHoliday,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
}

impl QueueOpening {
    pub fn new(queue_opening_id: uuid::Uuid, tenant_id: uuid::Uuid, queue_id: uuid::Uuid, weekday: models::WeekdayHoliday, start_time: String, end_time: String) -> QueueOpening {
        QueueOpening {
            queue_opening_id,
            tenant_id,
            queue_id,
            weekday,
            start_time,
            end_time,
        }
    }
}

