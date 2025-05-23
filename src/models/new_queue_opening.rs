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
pub struct NewQueueOpening {
    #[serde(rename = "weekday")]
    pub weekday: models::WeekdayHoliday,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
}

impl NewQueueOpening {
    pub fn new(weekday: models::WeekdayHoliday, start_time: String, end_time: String) -> NewQueueOpening {
        NewQueueOpening {
            weekday,
            start_time,
            end_time,
        }
    }
}

