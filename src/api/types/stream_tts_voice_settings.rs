pub use crate::prelude::*;

/// Advanced settings for voice customization.
///
/// Attributes:
/// enhance_reference_audio_quality: Whether to enhance the quality of the
/// reference audio before cloning. Defaults to False.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StreamTtsVoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhance_reference_audio_quality: Option<Option<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain_source_accent: Option<Option<bool>>,
}
