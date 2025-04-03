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
pub struct NumberConfiguration {
    #[serde(rename = "numberConfigurationId")]
    pub number_configuration_id: uuid::Uuid,
    #[serde(rename = "tenantId")]
    pub tenant_id: uuid::Uuid,
    #[serde(rename = "number")]
    pub number: String,
    #[serde(rename = "lastChange")]
    pub last_change: String,
}

impl NumberConfiguration {
    pub fn new(number_configuration_id: uuid::Uuid, tenant_id: uuid::Uuid, number: String, last_change: String) -> NumberConfiguration {
        NumberConfiguration {
            number_configuration_id,
            tenant_id,
            number,
            last_change,
        }
    }
}

