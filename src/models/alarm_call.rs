/*
 * Phone and Queue Backend
 *
 * # API for managing phone services  This is the API of the service at P7M that manages phone services.  For all current endpoints, the caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. If your interacting with this API using the Swagger interface, you need to set the JWT token by clicking on the **Authorize** button on the right side of the header. As the value don't forget that the Authorization header starts with the fixed value `Bearer` followed by a space followed by the actual JWT token value.  **Attention:** _this API will probably still change a lot in the future, it's not at all stable yet_  If anything is unclear or you found a bug in the documentation, please contact <tech@p7m.de>. 
 *
 * The version of the OpenAPI document: 0.3.5
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AlarmCall {
    #[serde(rename = "alarmCallId")]
    pub alarm_call_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "alarmId")]
    pub alarm_id: String,
    #[serde(rename = "callee")]
    pub callee: String,
    #[serde(rename = "callState")]
    pub call_state: CallState,
    #[serde(rename = "callText")]
    pub call_text: String,
    #[serde(rename = "callTextId")]
    pub call_text_id: String,
    #[serde(rename = "callbackText")]
    pub callback_text: String,
    #[serde(rename = "callbackTextId")]
    pub callback_text_id: String,
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
    pub fn new(alarm_call_id: String, tenant_id: String, alarm_id: String, callee: String, call_state: CallState, call_text: String, call_text_id: String, callback_text: String, callback_text_id: String, call_accepted_digits: String, callback_accepted_digits: String, pressed_digit: String, last_change: String) -> AlarmCall {
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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallState {
    #[serde(rename = "REQUESTED")]
    REQUESTED,
    #[serde(rename = "AUDIO_EXISTS")]
    AUDIOEXISTS,
    #[serde(rename = "SPOOLED")]
    SPOOLED,
    #[serde(rename = "CONNECTED")]
    CONNECTED,
    #[serde(rename = "PRESSED_DIGIT")]
    PRESSEDDIGIT,
    #[serde(rename = "CALLEE_HANGUP")]
    CALLEEHANGUP,
    #[serde(rename = "DIALER_HANGUP")]
    DIALERHANGUP,
    #[serde(rename = "CALLED_BACK")]
    CALLEDBACK,
    #[serde(rename = "CALLBACK_ENDED")]
    CALLBACKENDED,
    #[serde(rename = "CANCELED")]
    CANCELED,
    #[serde(rename = "DUMMY")]
    DUMMY,
}

impl Default for CallState {
    fn default() -> CallState {
        Self::REQUESTED
    }
}

