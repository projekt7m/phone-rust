/*
 * Phone and Queue Backend
 *
 * API for managing phone services  This is the API of the service at P7M that manages phone services.  **Attention:** this API will probably still change a lot in the future, it's not at all stable yet
 *
 * The version of the OpenAPI document: 0.5.0
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct QueueOpeningData {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::QueueOpening>,
}

impl QueueOpeningData {
    pub fn new(data: Vec<crate::models::QueueOpening>) -> QueueOpeningData {
        QueueOpeningData {
            data,
        }
    }
}


