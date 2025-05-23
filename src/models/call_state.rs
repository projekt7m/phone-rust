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

/// CallState : State of a call(er) in a queue
/// State of a call(er) in a queue
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallState {
    #[serde(rename = "INTRO")]
    Intro,
    #[serde(rename = "WAITING")]
    Waiting,
    #[serde(rename = "CONNECTED")]
    Connected,

}

impl std::fmt::Display for CallState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Intro => write!(f, "INTRO"),
            Self::Waiting => write!(f, "WAITING"),
            Self::Connected => write!(f, "CONNECTED"),
        }
    }
}

impl Default for CallState {
    fn default() -> CallState {
        Self::Intro
    }
}

