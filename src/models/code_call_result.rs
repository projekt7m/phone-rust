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
pub struct CodeCallResult {
    #[serde(rename = "accepted")]
    pub accepted: bool,
    #[serde(rename = "dialing")]
    pub dialing: String,
    #[serde(rename = "reason")]
    pub reason: String,
}

impl CodeCallResult {
    pub fn new(accepted: bool, dialing: String, reason: String) -> CodeCallResult {
        CodeCallResult {
            accepted,
            dialing,
            reason,
        }
    }
}

