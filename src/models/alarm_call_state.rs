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
pub enum AlarmCallState {
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

impl ToString for AlarmCallState {
    fn to_string(&self) -> String {
        match self {
            Self::REQUESTED => String::from("REQUESTED"),
            Self::AUDIOEXISTS => String::from("AUDIO_EXISTS"),
            Self::SPOOLED => String::from("SPOOLED"),
            Self::CONNECTED => String::from("CONNECTED"),
            Self::PRESSEDDIGIT => String::from("PRESSED_DIGIT"),
            Self::CALLEEHANGUP => String::from("CALLEE_HANGUP"),
            Self::DIALERHANGUP => String::from("DIALER_HANGUP"),
            Self::CALLEDBACK => String::from("CALLED_BACK"),
            Self::CALLBACKENDED => String::from("CALLBACK_ENDED"),
            Self::CANCELED => String::from("CANCELED"),
            Self::DUMMY => String::from("DUMMY"),
        }
    }
}

impl Default for AlarmCallState {
    fn default() -> AlarmCallState {
        Self::REQUESTED
    }
}



