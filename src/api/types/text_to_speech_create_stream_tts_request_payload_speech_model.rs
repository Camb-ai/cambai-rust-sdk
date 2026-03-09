pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateStreamTtsRequestPayloadSpeechModel {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "mars-8")]
    Mars8,
    #[serde(rename = "mars-8-flash")]
    Mars8Flash,
    #[serde(rename = "mars-8-instruct")]
    Mars8Instruct,
    #[serde(rename = "mars-7")]
    Mars7,
    #[serde(rename = "mars-6")]
    Mars6,
    #[serde(rename = "mars-pro")]
    MarsPro,
    #[serde(rename = "mars-flash")]
    MarsFlash,
    #[serde(rename = "mars-instruct")]
    MarsInstruct,
}
impl fmt::Display for CreateStreamTtsRequestPayloadSpeechModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Auto => "auto",
            Self::Mars8 => "mars-8",
            Self::Mars8Flash => "mars-8-flash",
            Self::Mars8Instruct => "mars-8-instruct",
            Self::Mars7 => "mars-7",
            Self::Mars6 => "mars-6",
            Self::MarsPro => "mars-pro",
            Self::MarsFlash => "mars-flash",
            Self::MarsInstruct => "mars-instruct",
        };
        write!(f, "{}", s)
    }
}
