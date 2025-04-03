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
pub struct ListWrapperNumberConfiguration {
    #[serde(rename = "data")]
    pub data: Vec<models::ListWrapperNumberConfigurationDataInner>,
}

impl ListWrapperNumberConfiguration {
    pub fn new(data: Vec<models::ListWrapperNumberConfigurationDataInner>) -> ListWrapperNumberConfiguration {
        ListWrapperNumberConfiguration {
            data,
        }
    }
}

