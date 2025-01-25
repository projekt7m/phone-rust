/*
 * Phone and Queue Backend
 *
 * API for managing phone services  This is the API of the service at P7M that manages phone services.  **Attention:** this API will probably still change a lot in the future, it's not at all stable yet
 *
 * The version of the OpenAPI document: 0.5.1
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeCallRequest {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "phone")]
    pub phone: String,
}

impl CodeCallRequest {
    pub fn new(code: String, phone: String) -> CodeCallRequest {
        CodeCallRequest {
            code,
            phone,
        }
    }
}

