pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TtsLanguage {
    #[serde(rename = "ar-kw")]
    ArKw,
    #[serde(rename = "de-ch")]
    DeCh,
    #[serde(rename = "ko-kr")]
    KoKr,
    #[serde(rename = "th-th")]
    ThTh,
    #[serde(rename = "ml-in")]
    MlIn,
    #[serde(rename = "pt-pt")]
    PtPt,
    #[serde(rename = "kn-in")]
    KnIn,
    #[serde(rename = "fi-fi")]
    FiFi,
    #[serde(rename = "es-mx")]
    EsMx,
    #[serde(rename = "fr-ca")]
    FrCa,
    #[serde(rename = "cs-cz")]
    CsCz,
    #[serde(rename = "pt-br")]
    PtBr,
    #[serde(rename = "hi-in")]
    HiIn,
    #[serde(rename = "ar-sy")]
    ArSy,
    #[serde(rename = "es-us")]
    EsUs,
    #[serde(rename = "bn-bd")]
    BnBd,
    #[serde(rename = "ja-jp")]
    JaJp,
    #[serde(rename = "mr-in")]
    MrIn,
    #[serde(rename = "ar-ma")]
    ArMa,
    #[serde(rename = "es-es")]
    EsEs,
    #[serde(rename = "en-us")]
    EnUs,
    #[serde(rename = "zh-cn")]
    ZhCn,
    #[serde(rename = "el-gr")]
    ElGr,
    #[serde(rename = "pl-pl")]
    PlPl,
    #[serde(rename = "ar-om")]
    ArOm,
    #[serde(rename = "fr-ch")]
    FrCh,
    #[serde(rename = "en-uk")]
    EnUk,
    #[serde(rename = "en-au")]
    EnAu,
    #[serde(rename = "ar-jo")]
    ArJo,
    #[serde(rename = "ar-ae")]
    ArAe,
    #[serde(rename = "tr-tr")]
    TrTr,
    #[serde(rename = "ar-ly")]
    ArLy,
    #[serde(rename = "ru-ru")]
    RuRu,
    #[serde(rename = "en-in")]
    EnIn,
    #[serde(rename = "ar-ye")]
    ArYe,
    #[serde(rename = "ar-eg")]
    ArEg,
    #[serde(rename = "fr-be")]
    FrBe,
    #[serde(rename = "ta-in")]
    TaIn,
    #[serde(rename = "zh-tw")]
    ZhTw,
    #[serde(rename = "vi-vn")]
    ViVn,
    #[serde(rename = "bn-in")]
    BnIn,
    #[serde(rename = "ar-sa")]
    ArSa,
    #[serde(rename = "de-at")]
    DeAt,
    #[serde(rename = "pa-in")]
    PaIn,
    #[serde(rename = "it-it")]
    ItIt,
    #[serde(rename = "nl-nl")]
    NlNl,
    #[serde(rename = "ar-bh")]
    ArBh,
    #[serde(rename = "fr-fr")]
    FrFr,
    #[serde(rename = "ar-qa")]
    ArQa,
    #[serde(rename = "uk-ua")]
    UkUa,
    #[serde(rename = "ar-tn")]
    ArTn,
    #[serde(rename = "de-de")]
    DeDe,
    #[serde(rename = "ar-xa")]
    ArXa,
    #[serde(rename = "ar-lb")]
    ArLb,
    #[serde(rename = "zh-hk")]
    ZhHk,
    #[serde(rename = "ro-ro")]
    RoRo,
    #[serde(rename = "as-in")]
    AsIn,
    #[serde(rename = "ar-iq")]
    ArIq,
    #[serde(rename = "nl-be")]
    NlBe,
    #[serde(rename = "te-in")]
    TeIn,
    #[serde(rename = "id-id")]
    IdId,
    #[serde(rename = "ar-dz")]
    ArDz,
}
impl fmt::Display for TtsLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ArKw => "ar-kw",
            Self::DeCh => "de-ch",
            Self::KoKr => "ko-kr",
            Self::ThTh => "th-th",
            Self::MlIn => "ml-in",
            Self::PtPt => "pt-pt",
            Self::KnIn => "kn-in",
            Self::FiFi => "fi-fi",
            Self::EsMx => "es-mx",
            Self::FrCa => "fr-ca",
            Self::CsCz => "cs-cz",
            Self::PtBr => "pt-br",
            Self::HiIn => "hi-in",
            Self::ArSy => "ar-sy",
            Self::EsUs => "es-us",
            Self::BnBd => "bn-bd",
            Self::JaJp => "ja-jp",
            Self::MrIn => "mr-in",
            Self::ArMa => "ar-ma",
            Self::EsEs => "es-es",
            Self::EnUs => "en-us",
            Self::ZhCn => "zh-cn",
            Self::ElGr => "el-gr",
            Self::PlPl => "pl-pl",
            Self::ArOm => "ar-om",
            Self::FrCh => "fr-ch",
            Self::EnUk => "en-uk",
            Self::EnAu => "en-au",
            Self::ArJo => "ar-jo",
            Self::ArAe => "ar-ae",
            Self::TrTr => "tr-tr",
            Self::ArLy => "ar-ly",
            Self::RuRu => "ru-ru",
            Self::EnIn => "en-in",
            Self::ArYe => "ar-ye",
            Self::ArEg => "ar-eg",
            Self::FrBe => "fr-be",
            Self::TaIn => "ta-in",
            Self::ZhTw => "zh-tw",
            Self::ViVn => "vi-vn",
            Self::BnIn => "bn-in",
            Self::ArSa => "ar-sa",
            Self::DeAt => "de-at",
            Self::PaIn => "pa-in",
            Self::ItIt => "it-it",
            Self::NlNl => "nl-nl",
            Self::ArBh => "ar-bh",
            Self::FrFr => "fr-fr",
            Self::ArQa => "ar-qa",
            Self::UkUa => "uk-ua",
            Self::ArTn => "ar-tn",
            Self::DeDe => "de-de",
            Self::ArXa => "ar-xa",
            Self::ArLb => "ar-lb",
            Self::ZhHk => "zh-hk",
            Self::RoRo => "ro-ro",
            Self::AsIn => "as-in",
            Self::ArIq => "ar-iq",
            Self::NlBe => "nl-be",
            Self::TeIn => "te-in",
            Self::IdId => "id-id",
            Self::ArDz => "ar-dz",
        };
        write!(f, "{}", s)
    }
}
