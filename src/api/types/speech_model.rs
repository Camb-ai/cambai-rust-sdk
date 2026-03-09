pub use crate::prelude::*;

/// Speech model variant to use for synthesis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpeechModel {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "mars-pro")]
    MarsPro,
    #[serde(rename = "mars-flash")]
    MarsFlash,
    #[serde(rename = "mars-instruct")]
    MarsInstruct,
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
}
impl fmt::Display for SpeechModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Auto => "auto",
            Self::MarsPro => "mars-pro",
            Self::MarsFlash => "mars-flash",
            Self::MarsInstruct => "mars-instruct",
            Self::Mars8 => "mars-8",
            Self::Mars8Flash => "mars-8-flash",
            Self::Mars8Instruct => "mars-8-instruct",
            Self::Mars7 => "mars-7",
            Self::Mars6 => "mars-6",
        };
        write!(f, "{}", s)
    }
}
