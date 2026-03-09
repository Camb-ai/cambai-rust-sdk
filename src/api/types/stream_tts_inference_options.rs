pub use crate::prelude::*;

/// Advanced options for controlling the TTS inference process.
///
/// These options are only supported for the MARS-8 family of speech models.
///
/// Attributes:
/// stability: A value between 0.0 and 1.0 that controls the stability of
/// the output audio.
/// temperature: A value between 0.01 and 4.0 that controls the randomness
/// of the output. Higher values mean more randomness.
/// inference_steps: The number of steps for the diffusion model. A value
/// between 1 and 1000.
/// speaker_similarity: A value between 0.0 and 1.0 that controls how
/// similar the output should be to the reference speaker.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StreamTtsInferenceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<Option<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Option<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_steps: Option<Option<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_similarity: Option<Option<f64>>,
}
