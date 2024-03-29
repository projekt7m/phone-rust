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
pub struct AudioSnippet {
    #[serde(rename = "audioSnippetId")]
    pub audio_snippet_id: String,
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "textHash")]
    pub text_hash: String,
    #[serde(rename = "audioSource")]
    pub audio_source: AudioSource,
    #[serde(rename = "recordingRequested")]
    pub recording_requested: bool,
}

impl AudioSnippet {
    pub fn new(audio_snippet_id: String, tenant_id: String, text: String, text_hash: String, audio_source: AudioSource, recording_requested: bool) -> AudioSnippet {
        AudioSnippet {
            audio_snippet_id,
            tenant_id,
            text,
            text_hash,
            audio_source,
            recording_requested,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AudioSource {
    #[serde(rename = "PENDING")]
    PENDING,
    #[serde(rename = "TTS")]
    TTS,
    #[serde(rename = "SYSTEM")]
    SYSTEM,
    #[serde(rename = "CLIENT")]
    CLIENT,
}

impl Default for AudioSource {
    fn default() -> AudioSource {
        Self::PENDING
    }
}

