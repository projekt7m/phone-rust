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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum WeekdayHoliday {
    #[serde(rename = "MONDAY")]
    Monday,
    #[serde(rename = "TUESDAY")]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    Thursday,
    #[serde(rename = "FRIDAY")]
    Friday,
    #[serde(rename = "SATURDAY")]
    Saturday,
    #[serde(rename = "SUNDAY")]
    Sunday,
    #[serde(rename = "HOLIDAY")]
    Holiday,

}

impl std::fmt::Display for WeekdayHoliday {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Monday => write!(f, "MONDAY"),
            Self::Tuesday => write!(f, "TUESDAY"),
            Self::Wednesday => write!(f, "WEDNESDAY"),
            Self::Thursday => write!(f, "THURSDAY"),
            Self::Friday => write!(f, "FRIDAY"),
            Self::Saturday => write!(f, "SATURDAY"),
            Self::Sunday => write!(f, "SUNDAY"),
            Self::Holiday => write!(f, "HOLIDAY"),
        }
    }
}

impl Default for WeekdayHoliday {
    fn default() -> WeekdayHoliday {
        Self::Monday
    }
}

