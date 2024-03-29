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
pub struct AlarmCallData {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::AlarmCall>,
}

impl AlarmCallData {
    pub fn new(data: Vec<crate::models::AlarmCall>) -> AlarmCallData {
        AlarmCallData {
            data,
        }
    }
}


