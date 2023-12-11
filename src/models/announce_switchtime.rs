/*
 * Phone and Queue Backend
 *
 * # API for managing phone services  This is the API of the service at P7M that manages phone services.  For all current endpoints, the caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. If your interacting with this API using the Swagger interface, you need to set the JWT token by clicking on the **Authorize** button on the right side of the header. As the value don't forget that the Authorization header starts with the fixed value `Bearer` followed by a space followed by the actual JWT token value.  **Attention:** _this API will probably still change a lot in the future, it's not at all stable yet_  If anything is unclear or you found a bug in the documentation, please contact <tech@p7m.de>. 
 *
 * The version of the OpenAPI document: 0.3.4
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AnnounceSwitchtime {
    #[serde(rename = "switchtime_id")]
    pub switchtime_id: String,
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    #[serde(rename = "queue_id")]
    pub queue_id: String,
    #[serde(rename = "weekday")]
    pub weekday: Weekday,
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "end_time")]
    pub end_time: String,
    #[serde(rename = "announce", skip_serializing_if = "Option::is_none")]
    pub announce: Option<Announce>,
}

impl AnnounceSwitchtime {
    pub fn new(switchtime_id: String, tenant_id: String, queue_id: String, weekday: Weekday, start_time: String, end_time: String) -> AnnounceSwitchtime {
        AnnounceSwitchtime {
            switchtime_id,
            tenant_id,
            queue_id,
            weekday,
            start_time,
            end_time,
            announce: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Weekday {
    #[serde(rename = "MONDAY")]
    MONDAY,
    #[serde(rename = "TUESDAY")]
    TUESDAY,
    #[serde(rename = "WEDNESDAY")]
    WEDNESDAY,
    #[serde(rename = "THURSDAY")]
    THURSDAY,
    #[serde(rename = "FRIDAY")]
    FRIDAY,
    #[serde(rename = "SATURDAY")]
    SATURDAY,
    #[serde(rename = "SUNDAY")]
    SUNDAY,
    #[serde(rename = "HOLIDAY")]
    HOLIDAY,
}

impl Default for Weekday {
    fn default() -> Weekday {
        Self::MONDAY
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Announce {
    #[serde(rename = "CLOSED")]
    CLOSED,
    #[serde(rename = "PUBLIC_HOLIDAY")]
    PUBLICHOLIDAY,
    #[serde(rename = "COMPANY_HOLIDAY")]
    COMPANYHOLIDAY,
}

impl Default for Announce {
    fn default() -> Announce {
        Self::CLOSED
    }
}

