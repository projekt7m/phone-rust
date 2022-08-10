/*
 * Phone and Queue Backend
 *
 * # API for managing phone services  This is the API of the service at P7M that manages phone services.  For all current endpoints, the caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. If your interacting with this API using the Swagger interface, you need to set the JWT token by clicking on the **Authorize** button on the right side of the header. As the value don't forget that the Authorization header starts with the fixed value `Bearer` followed by a space followed by the actual JWT token value.  **Attention:** _this API will probably still change a lot in the future, it's not at all stable yet_  If anything is unclear or you found a bug in the documentation, please contact <tech@p7m.de>. 
 *
 * The version of the OpenAPI document: 0.3.3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct QueueCall {
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "caller_id_num", skip_serializing_if = "Option::is_none")]
    pub caller_id_num: Option<String>,
    #[serde(rename = "caller_id_name", skip_serializing_if = "Option::is_none")]
    pub caller_id_name: Option<String>,
    #[serde(rename = "call_time")]
    pub call_time: i32,
    #[serde(rename = "wait_time")]
    pub wait_time: i32,
    #[serde(rename = "talk_time", skip_serializing_if = "Option::is_none")]
    pub talk_time: Option<i32>,
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(rename = "unique_id")]
    pub unique_id: String,
    #[serde(rename = "queue_pos", skip_serializing_if = "Option::is_none")]
    pub queue_pos: Option<i64>,
}

impl QueueCall {
    pub fn new(state: State, call_time: i32, wait_time: i32, unique_id: String) -> QueueCall {
        QueueCall {
            state,
            caller_id_num: None,
            caller_id_name: None,
            call_time,
            wait_time,
            talk_time: None,
            priority: None,
            unique_id,
            queue_pos: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ENTERING")]
    ENTERING,
    #[serde(rename = "ANNOUNCE_LIMIT")]
    ANNOUNCELIMIT,
    #[serde(rename = "WAITING")]
    WAITING,
    #[serde(rename = "RINGING")]
    RINGING,
    #[serde(rename = "CONNECTED")]
    CONNECTED,
}

impl Default for State {
    fn default() -> State {
        Self::ENTERING
    }
}

