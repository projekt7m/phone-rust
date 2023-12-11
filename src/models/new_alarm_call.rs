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
pub struct NewAlarmCall {
    #[serde(rename = "alarmId")]
    pub alarm_id: String,
    #[serde(rename = "callee")]
    pub callee: String,
    #[serde(rename = "callText")]
    pub call_text: String,
    #[serde(rename = "callbackText")]
    pub callback_text: String,
    #[serde(rename = "callAcceptedDigits")]
    pub call_accepted_digits: String,
    #[serde(rename = "callbackAcceptedDigits")]
    pub callback_accepted_digits: String,
    #[serde(rename = "dummy")]
    pub dummy: bool,
}

impl NewAlarmCall {
    pub fn new(alarm_id: String, callee: String, call_text: String, callback_text: String, call_accepted_digits: String, callback_accepted_digits: String, dummy: bool) -> NewAlarmCall {
        NewAlarmCall {
            alarm_id,
            callee,
            call_text,
            callback_text,
            call_accepted_digits,
            callback_accepted_digits,
            dummy,
        }
    }
}


