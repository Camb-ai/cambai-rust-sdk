pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStreamTtsRequestPayload {
    pub text: String,
    pub language: TtsLanguage,
    pub voice_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_model: Option<SpeechModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_instructions: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhance_named_entities_pronunciation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_configuration: Option<StreamTtsOutputConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<StreamTtsVoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_options: Option<StreamTtsInferenceOptions>,
}
