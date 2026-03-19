use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SupportedLanguage {
    #[serde(rename = "af")]
    Afrikaans,
    #[serde(rename = "sq")]
    Albanian,
    #[serde(rename = "ar_dz")]
    ArabicAlgeria,
    #[serde(rename = "ar_bh")]
    ArabicBahrain,
    #[serde(rename = "ar_eg")]
    ArabicEgypt,
    #[serde(rename = "ar_iq")]
    ArabicIraq,
    #[serde(rename = "ar_jo")]
    ArabicJordan,
    #[serde(rename = "ar_kw")]
    ArabicKuwait,
    #[serde(rename = "ar_lb")]
    ArabicLebanon,
    #[serde(rename = "ar_ly")]
    ArabicLibya,
    #[serde(rename = "ar_ma")]
    ArabicMorocco,
    #[serde(rename = "ar_om")]
    ArabicOman,
    #[serde(rename = "ar_qa")]
    ArabicQatar,
    #[serde(rename = "ar_sa")]
    ArabicSaudiArabia,
    #[serde(rename = "ar_sy")]
    ArabicSyria,
    #[serde(rename = "ar_tn")]
    ArabicTunisia,
    #[serde(rename = "ar_ae")]
    ArabicUAE,
    #[serde(rename = "ar_ye")]
    ArabicYemen,
    #[serde(rename = "eu")]
    Basque,
    #[serde(rename = "be")]
    Belarusian,
    #[serde(rename = "bg")]
    Bulgarian,
    #[serde(rename = "ca")]
    Catalan,
    #[serde(rename = "zh_hk")]
    ChineseHongKong,
    #[serde(rename = "zh_cn")]
    ChinesePRC,
    #[serde(rename = "zh_sg")]
    ChineseSingapore,
    #[serde(rename = "zh_tw")]
    ChineseTaiwan,
    #[serde(rename = "hr")]
    Croatian,
    #[serde(rename = "cs")]
    Czech,
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "nl_be")]
    DutchBelgium,
    #[serde(rename = "nl")]
    DutchStandard,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "en_au")]
    EnglishAustralia,
    #[serde(rename = "en_bz")]
    EnglishBelize,
    #[serde(rename = "en_ca")]
    EnglishCanada,
    #[serde(rename = "en_ie")]
    EnglishIreland,
    #[serde(rename = "en_jm")]
    EnglishJamaica,
    #[serde(rename = "en_nz")]
    EnglishNewZealand,
    #[serde(rename = "en_za")]
    EnglishSouthAfrica,
    #[serde(rename = "en_tt")]
    EnglishTrinidad,
    #[serde(rename = "en_gb")]
    EnglishUnitedKingdom,
    #[serde(rename = "en_us")]
    EnglishUnitedStates,
    #[serde(rename = "et")]
    Estonian,
    #[serde(rename = "fo")]
    Faeroese,
    #[serde(rename = "fa")]
    Farsi,
    #[serde(rename = "fi")]
    Finnish,
    #[serde(rename = "fr_be")]
    FrenchBelgium,
    #[serde(rename = "fr_ca")]
    FrenchCanada,
    #[serde(rename = "fr_lu")]
    FrenchLuxembourg,
    #[serde(rename = "fr")]
    FrenchStandard,
    #[serde(rename = "fr_ch")]
    FrenchSwitzerland,
    #[serde(rename = "gd")]
    GaelicScotland,
    #[serde(rename = "de_at")]
    GermanAustria,
    #[serde(rename = "de_li")]
    GermanLiechtenstein,
    #[serde(rename = "de_lu")]
    GermanLuxembourg,
    #[serde(rename = "de")]
    GermanStandard,
    #[serde(rename = "de_ch")]
    GermanSwitzerland,
    #[serde(rename = "el")]
    Greek,
    #[serde(rename = "he")]
    Hebrew,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "hu")]
    Hungarian,
    #[serde(rename = "is")]
    Icelandic,
    #[serde(rename = "id")]
    Indonesian,
    #[serde(rename = "ga")]
    Irish,
    #[serde(rename = "it")]
    ItalianStandard,
    #[serde(rename = "it_ch")]
    ItalianSwitzerland,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "ko_jo")]
    KoreanJohab,
    #[serde(rename = "ku")]
    Kurdish,
    #[serde(rename = "lv")]
    Latvian,
    #[serde(rename = "lt")]
    Lithuanian,
    #[serde(rename = "mk")]
    MacedonianFYROM,
    #[serde(rename = "ml")]
    Malayalam,
    #[serde(rename = "ms")]
    Malaysian,
    #[serde(rename = "mt")]
    Maltese,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "nb")]
    NorwegianBokmal,
    #[serde(rename = "nn")]
    NorwegianNynorsk,
    #[serde(rename = "pl")]
    Polish,
    #[serde(rename = "pt_br")]
    PortugueseBrazil,
    #[serde(rename = "pt")]
    PortuguesePortugal,
    #[serde(rename = "pa")]
    Punjabi,
    #[serde(rename = "rm")]
    RhaetoRomanic,
    #[serde(rename = "ro")]
    Romanian,
    #[serde(rename = "ro_md")]
    RomanianRepublicofMoldova,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "ru_md")]
    RussianRepublicofMoldova,
    #[serde(rename = "sr")]
    Serbian,
    #[serde(rename = "sk")]
    Slovak,
    #[serde(rename = "sl")]
    Slovenian,
    #[serde(rename = "sb")]
    Sorbian,
    #[serde(rename = "es_ar")]
    SpanishArgentina,
    #[serde(rename = "es_bo")]
    SpanishBolivia,
    #[serde(rename = "es_cl")]
    SpanishChile,
    #[serde(rename = "es_co")]
    SpanishColombia,
    #[serde(rename = "es_cr")]
    SpanishCostaRica,
    #[serde(rename = "es_do")]
    SpanishDominicanRepublic,
    #[serde(rename = "es_ec")]
    SpanishEcuador,
    #[serde(rename = "es_sv")]
    SpanishElSalvador,
    #[serde(rename = "es_gt")]
    SpanishGuatemala,
    #[serde(rename = "es_hn")]
    SpanishHonduras,
    #[serde(rename = "es_mx")]
    SpanishMexico,
    #[serde(rename = "es_ni")]
    SpanishNicaragua,
    #[serde(rename = "es_pa")]
    SpanishPanama,
    #[serde(rename = "es_py")]
    SpanishParaguay,
    #[serde(rename = "es_pe")]
    SpanishPeru,
    #[serde(rename = "es_pr")]
    SpanishPuertoRico,
    #[serde(rename = "es")]
    SpanishSpain,
    #[serde(rename = "es_uy")]
    SpanishUruguay,
    #[serde(rename = "es_ve")]
    SpanishVenezuela,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "sv_fi")]
    SwedishFinland,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "ts")]
    Tsonga,
    #[serde(rename = "tn")]
    Tswana,
    #[serde(rename = "tr")]
    Turkish,
    #[serde(rename = "ua")]
    Ukrainian,
    #[serde(rename = "ur")]
    Urdu,
    #[serde(rename = "ve")]
    Venda,
    #[serde(rename = "vi")]
    Vietnamese,
    #[serde(rename = "cy")]
    Welsh,
    #[serde(rename = "xh")]
    Xhosa,
    #[serde(rename = "ji")]
    Yiddish,
    #[serde(rename = "zu")]
    Zulu,
}

impl std::convert::AsRef<str> for SupportedLanguage {
    fn as_ref(&self) -> &str {
        match self {
            Self::Afrikaans => "af",
            Self::Albanian => "sq",
            Self::ArabicAlgeria => "ar_dz",
            Self::ArabicBahrain => "ar_bh",
            Self::ArabicEgypt => "ar_eg",
            Self::ArabicIraq => "ar_iq",
            Self::ArabicJordan => "ar_jo",
            Self::ArabicKuwait => "ar_kw",
            Self::ArabicLebanon => "ar_lb",
            Self::ArabicLibya => "ar_ly",
            Self::ArabicMorocco => "ar_ma",
            Self::ArabicOman => "ar_om",
            Self::ArabicQatar => "ar_qa",
            Self::ArabicSaudiArabia => "ar_sa",
            Self::ArabicSyria => "ar_sy",
            Self::ArabicTunisia => "ar_tn",
            Self::ArabicUAE => "ar_ae",
            Self::ArabicYemen => "ar_ye",
            Self::Basque => "eu",
            Self::Belarusian => "be",
            Self::Bulgarian => "bg",
            Self::Catalan => "ca",
            Self::ChineseHongKong => "zh_hk",
            Self::ChinesePRC => "zh_cn",
            Self::ChineseSingapore => "zh_sg",
            Self::ChineseTaiwan => "zh_tw",
            Self::Croatian => "hr",
            Self::Czech => "cs",
            Self::Danish => "da",
            Self::DutchBelgium => "nl_be",
            Self::DutchStandard => "nl",
            Self::English => "en",
            Self::EnglishAustralia => "en_au",
            Self::EnglishBelize => "en_bz",
            Self::EnglishCanada => "en_ca",
            Self::EnglishIreland => "en_ie",
            Self::EnglishJamaica => "en_jm",
            Self::EnglishNewZealand => "en_nz",
            Self::EnglishSouthAfrica => "en_za",
            Self::EnglishTrinidad => "en_tt",
            Self::EnglishUnitedKingdom => "en_gb",
            Self::EnglishUnitedStates => "en_us",
            Self::Estonian => "et",
            Self::Faeroese => "fo",
            Self::Farsi => "fa",
            Self::Finnish => "fi",
            Self::FrenchBelgium => "fr_be",
            Self::FrenchCanada => "fr_ca",
            Self::FrenchLuxembourg => "fr_lu",
            Self::FrenchStandard => "fr",
            Self::FrenchSwitzerland => "fr_ch",
            Self::GaelicScotland => "gd",
            Self::GermanAustria => "de_at",
            Self::GermanLiechtenstein => "de_li",
            Self::GermanLuxembourg => "de_lu",
            Self::GermanStandard => "de",
            Self::GermanSwitzerland => "de_ch",
            Self::Greek => "el",
            Self::Hebrew => "he",
            Self::Hindi => "hi",
            Self::Hungarian => "hu",
            Self::Icelandic => "is",
            Self::Indonesian => "id",
            Self::Irish => "ga",
            Self::ItalianStandard => "it",
            Self::ItalianSwitzerland => "it_ch",
            Self::Japanese => "ja",
            Self::Korean => "ko",
            Self::KoreanJohab => "ko",
            Self::Kurdish => "ku",
            Self::Latvian => "lv",
            Self::Lithuanian => "lt",
            Self::MacedonianFYROM => "mk",
            Self::Malayalam => "ml",
            Self::Malaysian => "ms",
            Self::Maltese => "mt",
            Self::Norwegian => "no",
            Self::NorwegianBokmal => "nb",
            Self::NorwegianNynorsk => "nn",
            Self::Polish => "pl",
            Self::PortugueseBrazil => "pt_br",
            Self::PortuguesePortugal => "pt",
            Self::Punjabi => "pa",
            Self::RhaetoRomanic => "rm",
            Self::Romanian => "ro",
            Self::RomanianRepublicofMoldova => "ro_md",
            Self::Russian => "ru",
            Self::RussianRepublicofMoldova => "ru_md",
            Self::Serbian => "sr",
            Self::Slovak => "sk",
            Self::Slovenian => "sl",
            Self::Sorbian => "sb",
            Self::SpanishArgentina => "es_ar",
            Self::SpanishBolivia => "es_bo",
            Self::SpanishChile => "es_cl",
            Self::SpanishColombia => "es_co",
            Self::SpanishCostaRica => "es_cr",
            Self::SpanishDominicanRepublic => "es_do",
            Self::SpanishEcuador => "es_ec",
            Self::SpanishElSalvador => "es_sv",
            Self::SpanishGuatemala => "es_gt",
            Self::SpanishHonduras => "es_hn",
            Self::SpanishMexico => "es_mx",
            Self::SpanishNicaragua => "es_ni",
            Self::SpanishPanama => "es_pa",
            Self::SpanishParaguay => "es_py",
            Self::SpanishPeru => "es_pe",
            Self::SpanishPuertoRico => "es_pr",
            Self::SpanishSpain => "es",
            Self::SpanishUruguay => "es_uy",
            Self::SpanishVenezuela => "es_ve",
            Self::Swedish => "sv",
            Self::SwedishFinland => "sv_fi",
            Self::Thai => "th",
            Self::Tsonga => "ts",
            Self::Tswana => "tn",
            Self::Turkish => "tr",
            Self::Ukrainian => "ua",
            Self::Urdu => "ur",
            Self::Venda => "ve",
            Self::Vietnamese => "vi",
            Self::Welsh => "cy",
            Self::Xhosa => "xh",
            Self::Yiddish => "ji",
            Self::Zulu => "zu",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_serialize() {
        let values = [
            (SupportedLanguage::EnglishUnitedStates, "en_us"),
            (SupportedLanguage::ChinesePRC, "zh_cn"),
        ];

        for (value, value_str) in values {
            let commit = json!({
                "value": value,
            });

            let json = serde_json::to_string(&commit).expect("Failed to serialize value");
            assert_eq!(json, format!("{{\"value\":\"{value_str}\"}}"))
        }
    }

    #[test]
    fn test_deserialize() {
        let values = [
            (SupportedLanguage::EnglishUnitedStates, "en_us"),
            (SupportedLanguage::ChinesePRC, "zh_cn"),
        ];

        for (value, value_str) in values {
            let commit = json!({
                "value": value,
            });
            let commit_str = format!("{{\"value\":\"{value_str}\"}}");

            let json: serde_json::Value =
                serde_json::from_str(&commit_str).expect("Failed to deserialize value");
            assert_eq!(commit, json)
        }
    }
}
