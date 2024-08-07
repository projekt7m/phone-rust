/*
 * Phone and Queue Backend
 *
 * API for managing phone services  This is the API of the service at P7M that manages phone services.  **Attention:** this API will probably still change a lot in the future, it's not at all stable yet
 *
 * The version of the OpenAPI document: 0.5.0
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CallState {
    #[serde(rename = "Entering")]
    Entering,
    #[serde(rename = "AnnounceLimit")]
    AnnounceLimit,
    #[serde(rename = "Waiting")]
    Waiting,
    #[serde(rename = "Ringing")]
    Ringing,
    #[serde(rename = "Connected")]
    Connected,

}

impl ToString for CallState {
    fn to_string(&self) -> String {
        match self {
            Self::Entering => String::from("Entering"),
            Self::AnnounceLimit => String::from("AnnounceLimit"),
            Self::Waiting => String::from("Waiting"),
            Self::Ringing => String::from("Ringing"),
            Self::Connected => String::from("Connected"),
        }
    }
}

impl Default for CallState {
    fn default() -> CallState {
        Self::Entering
    }
}




