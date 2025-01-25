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
pub struct AlarmCall {
    #[serde(rename = "alarmCallId")]
    pub alarm_call_id: uuid::Uuid,
    #[serde(rename = "tenantId")]
    pub tenant_id: uuid::Uuid,
    #[serde(rename = "alarmId")]
    pub alarm_id: uuid::Uuid,
    #[serde(rename = "callee")]
    pub callee: String,
    #[serde(rename = "callState")]
    pub call_state: models::AlarmCallState,
    #[serde(rename = "callText")]
    pub call_text: String,
    #[serde(rename = "callTextId")]
    pub call_text_id: uuid::Uuid,
    #[serde(rename = "callbackText")]
    pub callback_text: String,
    #[serde(rename = "callbackTextId")]
    pub callback_text_id: uuid::Uuid,
    #[serde(rename = "callAcceptedDigits")]
    pub call_accepted_digits: String,
    #[serde(rename = "callbackAcceptedDigits")]
    pub callback_accepted_digits: String,
    #[serde(rename = "pressedDigit")]
    pub pressed_digit: String,
    #[serde(rename = "lastChange")]
    pub last_change: String,
}

impl AlarmCall {
    pub fn new(alarm_call_id: uuid::Uuid, tenant_id: uuid::Uuid, alarm_id: uuid::Uuid, callee: String, call_state: models::AlarmCallState, call_text: String, call_text_id: uuid::Uuid, callback_text: String, callback_text_id: uuid::Uuid, call_accepted_digits: String, callback_accepted_digits: String, pressed_digit: String, last_change: String) -> AlarmCall {
        AlarmCall {
            alarm_call_id,
            tenant_id,
            alarm_id,
            callee,
            call_state,
            call_text,
            call_text_id,
            callback_text,
            callback_text_id,
            call_accepted_digits,
            callback_accepted_digits,
            pressed_digit,
            last_change,
        }
    }
}

