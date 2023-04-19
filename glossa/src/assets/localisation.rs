// Version: 0.0.1-alpha.6
#![allow(dead_code)]

use super::{lang_id_consts, HashMap, LangID};

pub(crate) type L10nMap = ::phf::Map<&'static str, &'static str>;
pub(crate) type SubLocaleMap = ::phf::Map<&'static str, fn() -> L10nMap>;
pub(crate) type LocaleMap = ::phf::Map<&'static str, fn() -> SubLocaleMap>;
pub(crate) type LocaleHashMap = HashMap<LangID, SubLocaleMap>;

/// Language ID: af;
/// Map name: "error";
/// Description: Afrikaans, Latyn, Suid-Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Geen gelokaliseerde teks gevind nie");
/// ```
pub(crate) const fn get_af_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Geen gelokaliseerde teks gevind nie"##),
    ],
}
}

/// af: Afrikaans, Latyn, Suid-Afrika
pub(crate) const fn get_af_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_af_map_error),
    ],
}
}

/// Language ID: am;
/// Map name: "error";
/// Description: አማርኛ, ኢትዮፒክ, ኢትዮጵያ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "የተካሄደው ጽሑፍ አልተገኘም");
/// ```
pub(crate) const fn get_am_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"የተካሄደው ጽሑፍ አልተገኘም"##),
    ],
}
}

/// am: አማርኛ, ኢትዮፒክ, ኢትዮጵያ
pub(crate) const fn get_am_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_am_map_error),
    ],
}
}

/// Language ID: ar;
/// Map name: "error";
/// Description: العربية, العربية, مصر;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "لم يتم العثور على نص محلي");
/// ```
pub(crate) const fn get_ar_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"لم يتم العثور على نص محلي"##),
    ],
}
}

/// ar: العربية, العربية, مصر
pub(crate) const fn get_ar_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ar_map_error),
    ],
}
}

/// Language ID: az;
/// Map name: "error";
/// Description: azərbaycan, latın, Azərbaycan;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Yerli mətn tapılmadı");
/// ```
pub(crate) const fn get_az_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Yerli mətn tapılmadı"##),
    ],
}
}

/// az: azərbaycan, latın, Azərbaycan
pub(crate) const fn get_az_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_az_map_error),
    ],
}
}

/// Language ID: be;
/// Map name: "error";
/// Description: беларуская, кірыліца, Беларусь;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ніякага лакалізаванага тэксту не знойдзена");
/// ```
pub(crate) const fn get_be_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ніякага лакалізаванага тэксту не знойдзена"##),
    ],
}
}

/// be: беларуская, кірыліца, Беларусь
pub(crate) const fn get_be_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_be_map_error),
    ],
}
}

/// Language ID: bg;
/// Map name: "error";
/// Description: български, кирилица, България;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Не е намерен локализиран текст");
/// ```
pub(crate) const fn get_bg_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Не е намерен локализиран текст"##),
    ],
}
}

/// bg: български, кирилица, България
pub(crate) const fn get_bg_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_bg_map_error),
    ],
}
}

/// Language ID: bn;
/// Map name: "error";
/// Description: বাংলা, বাংলা, বাংলাদেশ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "স\u{9cd}থ\u{9be}নীয\u{9bc} কোনও প\u{9be}ঠ\u{9cd}য প\u{9be}ওয\u{9bc}\u{9be} য\u{9be}য\u{9bc} নি");
/// ```
pub(crate) const fn get_bn_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"স্থানীয় কোনও পাঠ্য পাওয়া যায় নি"##),
    ],
}
}

/// bn: বাংলা, বাংলা, বাংলাদেশ
pub(crate) const fn get_bn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_bn_map_error),
    ],
}
}

/// Language ID: bs;
/// Map name: "error";
/// Description: bosanski, latinica, Bosna i Hercegovina;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nije pronađen lokalizirani tekst");
/// ```
pub(crate) const fn get_bs_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nije pronađen lokalizirani tekst"##),
    ],
}
}

/// bs: bosanski, latinica, Bosna i Hercegovina
pub(crate) const fn get_bs_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_bs_map_error),
    ],
}
}

/// Language ID: ca;
/// Map name: "error";
/// Description: català, llatí, Espanya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "No s'ha trobat cap text localitzat");
/// ```
pub(crate) const fn get_ca_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"No s'ha trobat cap text localitzat"##),
    ],
}
}

/// ca: català, llatí, Espanya
pub(crate) const fn get_ca_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ca_map_error),
    ],
}
}

/// Language ID: ceb;
/// Map name: "error";
/// Description: Cebuano, Latin, Pilipinas;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Wala'y nakit-an nga lokal nga teksto");
/// ```
pub(crate) const fn get_ceb_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Wala'y nakit-an nga lokal nga teksto"##),
    ],
}
}

/// ceb: Cebuano, Latin, Pilipinas
pub(crate) const fn get_ceb_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ceb_map_error),
    ],
}
}

/// Language ID: co;
/// Map name: "error";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nisun testu localizatu truvatu");
/// ```
pub(crate) const fn get_co_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nisun testu localizatu truvatu"##),
    ],
}
}

/// co: co-Latn-FR
pub(crate) const fn get_co_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_co_map_error),
    ],
}
}

/// Language ID: cs;
/// Map name: "error";
/// Description: čeština, latinka, Česko;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Žádný lokalizovaný text");
/// ```
pub(crate) const fn get_cs_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Žádný lokalizovaný text"##),
    ],
}
}

/// cs: čeština, latinka, Česko
pub(crate) const fn get_cs_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_cs_map_error),
    ],
}
}

/// Language ID: cy;
/// Map name: "error";
/// Description: Cymraeg, Lladin, Y Deyrnas Unedig;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ni ddarganfuwyd testun lleol");
/// ```
pub(crate) const fn get_cy_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ni ddarganfuwyd testun lleol"##),
    ],
}
}

/// cy: Cymraeg, Lladin, Y Deyrnas Unedig
pub(crate) const fn get_cy_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_cy_map_error),
    ],
}
}

/// Language ID: da;
/// Map name: "error";
/// Description: dansk, latinsk, Danmark;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ingen lokaliseret tekst fundet");
/// ```
pub(crate) const fn get_da_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ingen lokaliseret tekst fundet"##),
    ],
}
}

/// da: dansk, latinsk, Danmark
pub(crate) const fn get_da_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_da_map_error),
    ],
}
}

/// Language ID: de;
/// Map name: "error";
/// Description: Deutsch, Lateinisch, Deutschland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Kein lokalisierter Text gefunden");
/// ```
pub(crate) const fn get_de_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Kein lokalisierter Text gefunden"##),
    ],
}
}

/// de: Deutsch, Lateinisch, Deutschland
pub(crate) const fn get_de_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_de_map_error),
    ],
}
}

/// Language ID: el;
/// Map name: "error";
/// Description: Ελληνικά, Ελληνικό, Ελλάδα;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Δεν βρέθηκε κανένα τοπικό κείμενο");
/// ```
pub(crate) const fn get_el_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Δεν βρέθηκε κανένα τοπικό κείμενο"##),
    ],
}
}

/// el: Ελληνικά, Ελληνικό, Ελλάδα
pub(crate) const fn get_el_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_el_map_error),
    ],
}
}

/// Language ID: en;
/// Map name: "error";
/// Description: English, Latin, United States;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "No localized text found");
/// ```
pub(crate) const fn get_en_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"No localized text found"##),
    ],
}
}

/// Language ID: en;
/// Map name: "test";
/// Description: English, Latin, United States;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("test", "hello");
///
/// assert_eq!(msg, "world");
/// ```
pub(crate) const fn get_en_map_test() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("👋🌐", r##"hello world"##),
        ("hello", r##"world"##),
    ],
}
}

/// Language ID: en;
/// Map name: "test.yml";
/// Description: English, Latin, United States;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("test.yml", "hello");
///
/// assert_eq!(msg, "world");
/// ```
pub(crate) const fn get_en_map_test_0() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("👋🌐", r##"hello world"##),
        ("hello", r##"world"##),
    ],
}
}

/// en: English, Latin, United States
pub(crate) const fn get_en_map() -> SubLocaleMap {
    ::phf::Map {
    key: 15467950696543387533,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_en_map_error),
        ("test.yml", get_en_map_test_0),
        ("test", get_en_map_test),
    ],
}
}

/// Language ID: en-GB;
/// Map name: "error";
/// Description: English, Latin, United Kingdom;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "No localised text found");
/// ```
pub(crate) const fn get_en_gb_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"No localised text found"##),
    ],
}
}

/// en-GB: English, Latin, United Kingdom
pub(crate) const fn get_en_gb_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_en_gb_map_error),
    ],
}
}

/// Language ID: eo;
/// Map name: "error";
/// Description: esperanto, Latn, Mondo;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Neniu lokalizita teksto trovita");
/// ```
pub(crate) const fn get_eo_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Neniu lokalizita teksto trovita"##),
    ],
}
}

/// eo: esperanto, Latn, Mondo
pub(crate) const fn get_eo_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_eo_map_error),
    ],
}
}

/// Language ID: es;
/// Map name: "error";
/// Description: español, latino, España;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "No se encontró texto localizado");
/// ```
pub(crate) const fn get_es_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"No se encontró texto localizado"##),
    ],
}
}

/// es: español, latino, España
pub(crate) const fn get_es_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_es_map_error),
    ],
}
}

/// Language ID: et;
/// Map name: "error";
/// Description: eesti, ladina, Eesti;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Lokaliseeritud teksti ei leitud");
/// ```
pub(crate) const fn get_et_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Lokaliseeritud teksti ei leitud"##),
    ],
}
}

/// et: eesti, ladina, Eesti
pub(crate) const fn get_et_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_et_map_error),
    ],
}
}

/// Language ID: eu;
/// Map name: "error";
/// Description: euskara, latinoa, Espainia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ez da testurik aurkitu");
/// ```
pub(crate) const fn get_eu_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ez da testurik aurkitu"##),
    ],
}
}

/// eu: euskara, latinoa, Espainia
pub(crate) const fn get_eu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_eu_map_error),
    ],
}
}

/// Language ID: fa;
/// Map name: "error";
/// Description: فارسی, عربی, ایران;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "هیچ متن محلی یافت نشده است");
/// ```
pub(crate) const fn get_fa_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"هیچ متن محلی یافت نشده است"##),
    ],
}
}

/// fa: فارسی, عربی, ایران
pub(crate) const fn get_fa_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_fa_map_error),
    ],
}
}

/// Language ID: fi;
/// Map name: "error";
/// Description: suomi, latinalainen, Suomi;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Paikallista tekstiä ei löydy");
/// ```
pub(crate) const fn get_fi_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Paikallista tekstiä ei löydy"##),
    ],
}
}

/// fi: suomi, latinalainen, Suomi
pub(crate) const fn get_fi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_fi_map_error),
    ],
}
}

/// Language ID: fr;
/// Map name: "error";
/// Description: français, latin, France;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Aucun texte localisé trouvé");
/// ```
pub(crate) const fn get_fr_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Aucun texte localisé trouvé"##),
    ],
}
}

/// fr: français, latin, France
pub(crate) const fn get_fr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_fr_map_error),
    ],
}
}

/// Language ID: fy;
/// Map name: "error";
/// Description: Frysk, Latyn, Nederlân;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Gjin lokaliseare tekst fûn");
/// ```
pub(crate) const fn get_fy_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Gjin lokaliseare tekst fûn"##),
    ],
}
}

/// fy: Frysk, Latyn, Nederlân
pub(crate) const fn get_fy_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_fy_map_error),
    ],
}
}

/// Language ID: ga;
/// Map name: "error";
/// Description: Gaeilge, Laidineach, Éire;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ní bhfuarthas aon téacs logánta");
/// ```
pub(crate) const fn get_ga_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ní bhfuarthas aon téacs logánta"##),
    ],
}
}

/// ga: Gaeilge, Laidineach, Éire
pub(crate) const fn get_ga_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ga_map_error),
    ],
}
}

/// Language ID: gd;
/// Map name: "error";
/// Description: Gàidhlig, Laideann, An Rìoghachd Aonaichte;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Cha deach teacsa ionadail a lorg");
/// ```
pub(crate) const fn get_gd_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Cha deach teacsa ionadail a lorg"##),
    ],
}
}

/// gd: Gàidhlig, Laideann, An Rìoghachd Aonaichte
pub(crate) const fn get_gd_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_gd_map_error),
    ],
}
}

/// Language ID: gl;
/// Map name: "error";
/// Description: galego, latino, España;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Non se atopou texto localizado");
/// ```
pub(crate) const fn get_gl_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Non se atopou texto localizado"##),
    ],
}
}

/// gl: galego, latino, España
pub(crate) const fn get_gl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_gl_map_error),
    ],
}
}

/// Language ID: gu;
/// Map name: "error";
/// Description: ગુજરાતી, ગુજરાતી, ભારત;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "કોઈ સ\u{acd}થાનિક લખાણ મળ\u{acd}ય\u{ac1}\u{a82} નથી");
/// ```
pub(crate) const fn get_gu_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"કોઈ સ્થાનિક લખાણ મળ્યું નથી"##),
    ],
}
}

/// gu: ગુજરાતી, ગુજરાતી, ભારત
pub(crate) const fn get_gu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_gu_map_error),
    ],
}
}

/// Language ID: ha;
/// Map name: "error";
/// Description: Hausa, Latin, Nijeriya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ba a sami rubutu mara nauyi ba");
/// ```
pub(crate) const fn get_ha_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ba a sami rubutu mara nauyi ba"##),
    ],
}
}

/// ha: Hausa, Latin, Nijeriya
pub(crate) const fn get_ha_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ha_map_error),
    ],
}
}

/// Language ID: haw;
/// Map name: "error";
/// Description: ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "ʻAʻohe mea iʻikeʻia");
/// ```
pub(crate) const fn get_haw_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"ʻAʻohe mea iʻikeʻia"##),
    ],
}
}

/// haw: ʻŌlelo Hawaiʻi, Latn, ʻAmelika Hui Pū ʻIa
pub(crate) const fn get_haw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_haw_map_error),
    ],
}
}

/// Language ID: he;
/// Map name: "error";
/// Description: עברית, עברי, ישראל;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "לא נמצא טקסט מקומי");
/// ```
pub(crate) const fn get_he_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"לא נמצא טקסט מקומי"##),
    ],
}
}

/// he: עברית, עברי, ישראל
pub(crate) const fn get_he_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_he_map_error),
    ],
}
}

/// Language ID: hi;
/// Map name: "error";
/// Description: हिन्दी, देवनागरी, भारत;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "कोई स\u{94d}थानीय पाठ नही\u{902} मिला");
/// ```
pub(crate) const fn get_hi_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"कोई स्थानीय पाठ नहीं मिला"##),
    ],
}
}

/// hi: हिन्दी, देवनागरी, भारत
pub(crate) const fn get_hi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_hi_map_error),
    ],
}
}

/// Language ID: hr;
/// Map name: "error";
/// Description: hrvatski, latinica, Hrvatska;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nije pronađen lokalizirani tekst");
/// ```
pub(crate) const fn get_hr_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nije pronađen lokalizirani tekst"##),
    ],
}
}

/// hr: hrvatski, latinica, Hrvatska
pub(crate) const fn get_hr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_hr_map_error),
    ],
}
}

/// Language ID: ht;
/// Map name: "error";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Pa gen tèks lokalize yo te jwenn");
/// ```
pub(crate) const fn get_ht_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Pa gen tèks lokalize yo te jwenn"##),
    ],
}
}

/// ht: ht-Latn-HT
pub(crate) const fn get_ht_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ht_map_error),
    ],
}
}

/// Language ID: hu;
/// Map name: "error";
/// Description: magyar, Latin, Magyarország;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nem található lokalizált szöveg");
/// ```
pub(crate) const fn get_hu_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nem található lokalizált szöveg"##),
    ],
}
}

/// hu: magyar, Latin, Magyarország
pub(crate) const fn get_hu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_hu_map_error),
    ],
}
}

/// Language ID: hy;
/// Map name: "error";
/// Description: հայերեն, հայկական, Հայաստան;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Տեղայնացված տեքստ չի գտնվել");
/// ```
pub(crate) const fn get_hy_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Տեղայնացված տեքստ չի գտնվել"##),
    ],
}
}

/// hy: հայերեն, հայկական, Հայաստան
pub(crate) const fn get_hy_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_hy_map_error),
    ],
}
}

/// Language ID: id;
/// Map name: "error";
/// Description: Indonesia, Latin, Indonesia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Tidak ada teks lokal yang ditemukan");
/// ```
pub(crate) const fn get_id_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Tidak ada teks lokal yang ditemukan"##),
    ],
}
}

/// id: Indonesia, Latin, Indonesia
pub(crate) const fn get_id_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_id_map_error),
    ],
}
}

/// Language ID: ig;
/// Map name: "error";
/// Description: Igbo, Latin, Naịjịrịa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Enweghị ederede edepụtara");
/// ```
pub(crate) const fn get_ig_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Enweghị ederede edepụtara"##),
    ],
}
}

/// ig: Igbo, Latin, Naịjịrịa
pub(crate) const fn get_ig_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ig_map_error),
    ],
}
}

/// Language ID: is;
/// Map name: "error";
/// Description: íslenska, latneskt, Ísland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Enginn staðbundinn texti fannst");
/// ```
pub(crate) const fn get_is_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Enginn staðbundinn texti fannst"##),
    ],
}
}

/// is: íslenska, latneskt, Ísland
pub(crate) const fn get_is_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_is_map_error),
    ],
}
}

/// Language ID: it;
/// Map name: "error";
/// Description: italiano, latino, Italia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nessun testo localizzato trovato");
/// ```
pub(crate) const fn get_it_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nessun testo localizzato trovato"##),
    ],
}
}

/// it: italiano, latino, Italia
pub(crate) const fn get_it_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_it_map_error),
    ],
}
}

/// Language ID: iw;
/// Map name: "error";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "לא נמצא טקסט מקומי");
/// ```
pub(crate) const fn get_iw_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"לא נמצא טקסט מקומי"##),
    ],
}
}

/// iw: iw-Hebr-IL
pub(crate) const fn get_iw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_iw_map_error),
    ],
}
}

/// Language ID: ja;
/// Map name: "error";
/// Description: 日本語, 日本語の文字, 日本;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "ローカライズされたテキストは見つかりません");
/// ```
pub(crate) const fn get_ja_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"ローカライズされたテキストは見つかりません"##),
    ],
}
}

/// ja: 日本語, 日本語の文字, 日本
pub(crate) const fn get_ja_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ja_map_error),
    ],
}
}

/// Language ID: jw;
/// Map name: "error";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ora ditemokake teks lokal");
/// ```
pub(crate) const fn get_jw_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ora ditemokake teks lokal"##),
    ],
}
}

/// jw: jw-Latn-ID
pub(crate) const fn get_jw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_jw_map_error),
    ],
}
}

/// Language ID: ka;
/// Map name: "error";
/// Description: ქართული, ქართული, საქართველო;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "ლოკალიზებული ტექსტი ვერ მოიძებნა");
/// ```
pub(crate) const fn get_ka_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"ლოკალიზებული ტექსტი ვერ მოიძებნა"##),
    ],
}
}

/// ka: ქართული, ქართული, საქართველო
pub(crate) const fn get_ka_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ka_map_error),
    ],
}
}

/// Language ID: kk;
/// Map name: "error";
/// Description: қазақ тілі, кирилл жазуы, Қазақстан;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Локализацияланған мәтін табылған жоқ");
/// ```
pub(crate) const fn get_kk_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Локализацияланған мәтін табылған жоқ"##),
    ],
}
}

/// kk: қазақ тілі, кирилл жазуы, Қазақстан
pub(crate) const fn get_kk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_kk_map_error),
    ],
}
}

/// Language ID: km;
/// Map name: "error";
/// Description: ខ្មែរ, ខ្មែរ, កម្ពុជា;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "រកម\u{17b7}នឃើញអត\u{17d2}ថបទដែលបានធ\u{17d2}វើម\u{17bc}លដ\u{17d2}ឋាន\u{17b8}យកម\u{17d2}ម");
/// ```
pub(crate) const fn get_km_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"រកមិនឃើញអត្ថបទដែលបានធ្វើមូលដ្ឋានីយកម្ម"##),
    ],
}
}

/// km: ខ្មែរ, ខ្មែរ, កម្ពុជា
pub(crate) const fn get_km_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_km_map_error),
    ],
}
}

/// Language ID: kn;
/// Map name: "error";
/// Description: ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "ಯಾವುದೇ ಸ\u{ccd}ಥಳೀಯ ಪಠ\u{ccd}ಯ ಕಂಡುಬಂದ\u{cbf}ಲ\u{ccd}ಲ");
/// ```
pub(crate) const fn get_kn_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"ಯಾವುದೇ ಸ್ಥಳೀಯ ಪಠ್ಯ ಕಂಡುಬಂದಿಲ್ಲ"##),
    ],
}
}

/// kn: ಕನ್ನಡ, ಕನ್ನಡ, ಭಾರತ
pub(crate) const fn get_kn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_kn_map_error),
    ],
}
}

/// Language ID: ko;
/// Map name: "error";
/// Description: 한국어, 한국 문자, 대한민국;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "현지화 된 텍스트가 없습니다");
/// ```
pub(crate) const fn get_ko_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"현지화 된 텍스트가 없습니다"##),
    ],
}
}

/// ko: 한국어, 한국 문자, 대한민국
pub(crate) const fn get_ko_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ko_map_error),
    ],
}
}

/// Language ID: ku;
/// Map name: "error";
/// Description: kurdî, latînî, Tirkiye;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nivîsek herêmî nehat dîtin");
/// ```
pub(crate) const fn get_ku_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nivîsek herêmî nehat dîtin"##),
    ],
}
}

/// ku: kurdî, latînî, Tirkiye
pub(crate) const fn get_ku_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ku_map_error),
    ],
}
}

/// Language ID: ky;
/// Map name: "error";
/// Description: кыргызча, Кирилл, Кыргызстан;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Локалдаштырылган текст табылган жок");
/// ```
pub(crate) const fn get_ky_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Локалдаштырылган текст табылган жок"##),
    ],
}
}

/// ky: кыргызча, Кирилл, Кыргызстан
pub(crate) const fn get_ky_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ky_map_error),
    ],
}
}

/// Language ID: la;
/// Map name: "error";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Non localized illud invenitur");
/// ```
pub(crate) const fn get_la_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Non localized illud invenitur"##),
    ],
}
}

/// la: la-Latn-VA
pub(crate) const fn get_la_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_la_map_error),
    ],
}
}

/// Language ID: lb;
/// Map name: "error";
/// Description: Lëtzebuergesch, Laténgesch, Lëtzebuerg;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Kee lokalen Text fonnt");
/// ```
pub(crate) const fn get_lb_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Kee lokalen Text fonnt"##),
    ],
}
}

/// lb: Lëtzebuergesch, Laténgesch, Lëtzebuerg
pub(crate) const fn get_lb_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_lb_map_error),
    ],
}
}

/// Language ID: lo;
/// Map name: "error";
/// Description: ລາວ, ລາວ, ລາວ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "ບ\u{ecd}\u{ec8}ພ\u{ebb}ບຂ\u{ecd}\u{ec9}ຄວາມທ\u{ec9}ອງຖ\u{eb4}\u{ec8}ນ");
/// ```
pub(crate) const fn get_lo_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"ບໍ່ພົບຂໍ້ຄວາມທ້ອງຖິ່ນ"##),
    ],
}
}

/// lo: ລາວ, ລາວ, ລາວ
pub(crate) const fn get_lo_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_lo_map_error),
    ],
}
}

/// Language ID: lt;
/// Map name: "error";
/// Description: lietuvių, lotynų, Lietuva;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nerasta lokalizuoto teksto");
/// ```
pub(crate) const fn get_lt_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nerasta lokalizuoto teksto"##),
    ],
}
}

/// lt: lietuvių, lotynų, Lietuva
pub(crate) const fn get_lt_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_lt_map_error),
    ],
}
}

/// Language ID: lv;
/// Map name: "error";
/// Description: latviešu, latīņu, Latvija;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nav atrasts lokalizēts teksts");
/// ```
pub(crate) const fn get_lv_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nav atrasts lokalizēts teksts"##),
    ],
}
}

/// lv: latviešu, latīņu, Latvija
pub(crate) const fn get_lv_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_lv_map_error),
    ],
}
}

/// Language ID: mg;
/// Map name: "error";
/// Description: Malagasy, Latn, Madagasikara;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Tsy nisy lahatsoratra hita teo an-toerana hita");
/// ```
pub(crate) const fn get_mg_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Tsy nisy lahatsoratra hita teo an-toerana hita"##),
    ],
}
}

/// mg: Malagasy, Latn, Madagasikara
pub(crate) const fn get_mg_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_mg_map_error),
    ],
}
}

/// Language ID: mi;
/// Map name: "error";
/// Description: Māori, Rātina, Aotearoa;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Kaore i kitea he tuhinga kua kitea");
/// ```
pub(crate) const fn get_mi_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Kaore i kitea he tuhinga kua kitea"##),
    ],
}
}

/// mi: Māori, Rātina, Aotearoa
pub(crate) const fn get_mi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_mi_map_error),
    ],
}
}

/// Language ID: mk;
/// Map name: "error";
/// Description: македонски, кирилско писмо, Северна Македонија;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Не е пронајден локализиран текст");
/// ```
pub(crate) const fn get_mk_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Не е пронајден локализиран текст"##),
    ],
}
}

/// mk: македонски, кирилско писмо, Северна Македонија
pub(crate) const fn get_mk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_mk_map_error),
    ],
}
}

/// Language ID: ml;
/// Map name: "error";
/// Description: മലയാളം, മലയാളം, ഇന്ത്യ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "പ\u{d4d}ര\u{d3e}ദേശികവൽക\u{d4d}കരിച\u{d4d}ച വ\u{d3e}ചകം കണ\u{d4d}ടെത\u{d4d}തിയില\u{d4d}ല");
/// ```
pub(crate) const fn get_ml_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"പ്രാദേശികവൽക്കരിച്ച വാചകം കണ്ടെത്തിയില്ല"##),
    ],
}
}

/// ml: മലയാളം, മലയാളം, ഇന്ത്യ
pub(crate) const fn get_ml_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ml_map_error),
    ],
}
}

/// Language ID: mn;
/// Map name: "error";
/// Description: монгол, кирилл, Монгол;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Орон нутгийн текст олдсонгүй");
/// ```
pub(crate) const fn get_mn_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Орон нутгийн текст олдсонгүй"##),
    ],
}
}

/// mn: монгол, кирилл, Монгол
pub(crate) const fn get_mn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_mn_map_error),
    ],
}
}

/// Language ID: mr;
/// Map name: "error";
/// Description: मराठी, देवनागरी, भारत;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "कोणताही स\u{94d}थानिक मजक\u{942}र सापडला नाही");
/// ```
pub(crate) const fn get_mr_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"कोणताही स्थानिक मजकूर सापडला नाही"##),
    ],
}
}

/// mr: मराठी, देवनागरी, भारत
pub(crate) const fn get_mr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_mr_map_error),
    ],
}
}

/// Language ID: ms;
/// Map name: "error";
/// Description: Melayu, Latin, Malaysia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Tidak ada teks setempat yang dijumpai");
/// ```
pub(crate) const fn get_ms_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Tidak ada teks setempat yang dijumpai"##),
    ],
}
}

/// ms: Melayu, Latin, Malaysia
pub(crate) const fn get_ms_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ms_map_error),
    ],
}
}

/// Language ID: mt;
/// Map name: "error";
/// Description: Malti, Latin, Malta;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ma nstab l-ebda test lokalizzat");
/// ```
pub(crate) const fn get_mt_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ma nstab l-ebda test lokalizzat"##),
    ],
}
}

/// mt: Malti, Latin, Malta
pub(crate) const fn get_mt_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_mt_map_error),
    ],
}
}

/// Language ID: my;
/// Map name: "error";
/// Description: မြန်မာ, မြန်မာ, မြန်မာ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "အဘယ\u{103a}သ\u{1030}မျ\u{103e}မဒေသဆ\u{102d}\u{102f}င\u{103a}ရာစာသားက\u{102d}\u{102f}ရ\u{103e}ာမတ\u{103d}ေ\u{1037}ပါ");
/// ```
pub(crate) const fn get_my_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"အဘယ်သူမျှမဒေသဆိုင်ရာစာသားကိုရှာမတွေ့ပါ"##),
    ],
}
}

/// my: မြန်မာ, မြန်မာ, မြန်မာ
pub(crate) const fn get_my_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_my_map_error),
    ],
}
}

/// Language ID: ne;
/// Map name: "error";
/// Description: नेपाली, देवानागरी, नेपाल;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "क\u{941}न\u{948} स\u{94d}थानीय पद फ\u{947}ला पर\u{947}न");
/// ```
pub(crate) const fn get_ne_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"कुनै स्थानीय पद फेला परेन"##),
    ],
}
}

/// ne: नेपाली, देवानागरी, नेपाल
pub(crate) const fn get_ne_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ne_map_error),
    ],
}
}

/// Language ID: nl;
/// Map name: "error";
/// Description: Nederlands, Latijns, Nederland;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Geen gelokaliseerde tekst gevonden");
/// ```
pub(crate) const fn get_nl_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Geen gelokaliseerde tekst gevonden"##),
    ],
}
}

/// nl: Nederlands, Latijns, Nederland
pub(crate) const fn get_nl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_nl_map_error),
    ],
}
}

/// Language ID: no;
/// Map name: "error";
/// Description: norsk, latinsk, Norge;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ingen lokalisert tekst funnet");
/// ```
pub(crate) const fn get_no_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ingen lokalisert tekst funnet"##),
    ],
}
}

/// no: norsk, latinsk, Norge
pub(crate) const fn get_no_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_no_map_error),
    ],
}
}

/// Language ID: ny;
/// Map name: "error";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Palibe zolemba zomwe zapezeka");
/// ```
pub(crate) const fn get_ny_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Palibe zolemba zomwe zapezeka"##),
    ],
}
}

/// ny: ny-Latn-MW
pub(crate) const fn get_ny_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ny_map_error),
    ],
}
}

/// Language ID: or;
/// Map name: "error";
/// Description: ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "କ local ଣସ\u{b3f} ସ\u{b4d}ଥ\u{b3e}ନୀୟ ପ\u{b3e}ଠ\u{b4d}ୟ ମ\u{b3f}ଳ\u{b3f}ଲ\u{b3e} ନ\u{b3e}ହ\u{b3f}\u{b01} |");
/// ```
pub(crate) const fn get_or_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"କ local ଣସି ସ୍ଥାନୀୟ ପାଠ୍ୟ ମିଳିଲା ନାହିଁ |"##),
    ],
}
}

/// or: ଓଡ଼ିଆ, ଓଡ଼ିଆ, ଭାରତ
pub(crate) const fn get_or_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_or_map_error),
    ],
}
}

/// Language ID: pa;
/// Map name: "error";
/// Description: ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "ਕ\u{a4b}ਈ ਸਥਾਨਕ ਨਹੀ\u{a02} ਲ\u{a71}ਭਿਆ");
/// ```
pub(crate) const fn get_pa_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"ਕੋਈ ਸਥਾਨਕ ਨਹੀਂ ਲੱਭਿਆ"##),
    ],
}
}

/// pa: ਪੰਜਾਬੀ, ਗੁਰਮੁਖੀ, ਭਾਰਤ
pub(crate) const fn get_pa_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_pa_map_error),
    ],
}
}

/// Language ID: pl;
/// Map name: "error";
/// Description: polski, łacińskie, Polska;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nie znaleziono zlokalizowanego tekstu");
/// ```
pub(crate) const fn get_pl_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nie znaleziono zlokalizowanego tekstu"##),
    ],
}
}

/// pl: polski, łacińskie, Polska
pub(crate) const fn get_pl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_pl_map_error),
    ],
}
}

/// Language ID: ps;
/// Map name: "error";
/// Description: پښتو, عربي, افغانستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "هیڅ ځایی شوی متن وموندل شو");
/// ```
pub(crate) const fn get_ps_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"هیڅ ځایی شوی متن وموندل شو"##),
    ],
}
}

/// ps: پښتو, عربي, افغانستان
pub(crate) const fn get_ps_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ps_map_error),
    ],
}
}

/// Language ID: pt;
/// Map name: "error";
/// Description: português, latim, Brasil;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nenhum texto localizado encontrado");
/// ```
pub(crate) const fn get_pt_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nenhum texto localizado encontrado"##),
    ],
}
}

/// pt: português, latim, Brasil
pub(crate) const fn get_pt_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_pt_map_error),
    ],
}
}

/// Language ID: ro;
/// Map name: "error";
/// Description: română, latină, România;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nu a fost găsit niciun text localizat");
/// ```
pub(crate) const fn get_ro_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nu a fost găsit niciun text localizat"##),
    ],
}
}

/// ro: română, latină, România
pub(crate) const fn get_ro_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ro_map_error),
    ],
}
}

/// Language ID: ru;
/// Map name: "error";
/// Description: русский, кириллица, Россия;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Локализованный текст не найден");
/// ```
pub(crate) const fn get_ru_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Локализованный текст не найден"##),
    ],
}
}

/// ru: русский, кириллица, Россия
pub(crate) const fn get_ru_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ru_map_error),
    ],
}
}

/// Language ID: sd;
/// Map name: "error";
/// Description: سنڌي, عربي, پاڪستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "مقامي متن نه مليو");
/// ```
pub(crate) const fn get_sd_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"مقامي متن نه مليو"##),
    ],
}
}

/// sd: سنڌي, عربي, پاڪستان
pub(crate) const fn get_sd_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_sd_map_error),
    ],
}
}

/// Language ID: si;
/// Map name: "error";
/// Description: සිංහල, සිංහල, ශ්‍රී ලංකාව;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "දේශ\u{dd3}යකරණය කළ පෙළක\u{dca} හම\u{dd4} නොව\u{dd3}ය");
/// ```
pub(crate) const fn get_si_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"දේශීයකරණය කළ පෙළක් හමු නොවීය"##),
    ],
}
}

/// si: සිංහල, සිංහල, ශ්‍රී ලංකාව
pub(crate) const fn get_si_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_si_map_error),
    ],
}
}

/// Language ID: sk;
/// Map name: "error";
/// Description: slovenčina, latinka, Slovensko;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Nenašiel sa žiadny lokalizovaný text");
/// ```
pub(crate) const fn get_sk_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Nenašiel sa žiadny lokalizovaný text"##),
    ],
}
}

/// sk: slovenčina, latinka, Slovensko
pub(crate) const fn get_sk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_sk_map_error),
    ],
}
}

/// Language ID: sl;
/// Map name: "error";
/// Description: slovenščina, latinica, Slovenija;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ni bilo najdenega lokaliziranega besedila");
/// ```
pub(crate) const fn get_sl_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ni bilo najdenega lokaliziranega besedila"##),
    ],
}
}

/// sl: slovenščina, latinica, Slovenija
pub(crate) const fn get_sl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_sl_map_error),
    ],
}
}

/// Language ID: sm;
/// Map name: "error";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Leai se tusitusiga i le lotoifale maua");
/// ```
pub(crate) const fn get_sm_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Leai se tusitusiga i le lotoifale maua"##),
    ],
}
}

/// sm: sm-Latn-WS
pub(crate) const fn get_sm_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_sm_map_error),
    ],
}
}

/// Language ID: sn;
/// Map name: "error";
/// Description: chiShona, Latn, Zimbabwe;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Hapana zvinyorwa zvemukati zvinowanikwa");
/// ```
pub(crate) const fn get_sn_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Hapana zvinyorwa zvemukati zvinowanikwa"##),
    ],
}
}

/// sn: chiShona, Latn, Zimbabwe
pub(crate) const fn get_sn_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_sn_map_error),
    ],
}
}

/// Language ID: so;
/// Map name: "error";
/// Description: Soomaali, Laatiin, Soomaaliya;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ma jiro qoraal maxalli ah oo la helay");
/// ```
pub(crate) const fn get_so_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ma jiro qoraal maxalli ah oo la helay"##),
    ],
}
}

/// so: Soomaali, Laatiin, Soomaaliya
pub(crate) const fn get_so_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_so_map_error),
    ],
}
}

/// Language ID: sq;
/// Map name: "error";
/// Description: shqip, latin, Shqipëri;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Asnjë tekst i lokalizuar nuk u gjet");
/// ```
pub(crate) const fn get_sq_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Asnjë tekst i lokalizuar nuk u gjet"##),
    ],
}
}

/// sq: shqip, latin, Shqipëri
pub(crate) const fn get_sq_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_sq_map_error),
    ],
}
}

/// Language ID: sr;
/// Map name: "error";
/// Description: српски, ћирилица, Србија;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Није пронађен ниједан локализовани текст");
/// ```
pub(crate) const fn get_sr_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Није пронађен ниједан локализовани текст"##),
    ],
}
}

/// sr: српски, ћирилица, Србија
pub(crate) const fn get_sr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_sr_map_error),
    ],
}
}

/// Language ID: st;
/// Map name: "error";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ha ho na sengoloa sa lehae se fumanoeng");
/// ```
pub(crate) const fn get_st_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ha ho na sengoloa sa lehae se fumanoeng"##),
    ],
}
}

/// st: st-Latn-ZA
pub(crate) const fn get_st_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_st_map_error),
    ],
}
}

/// Language ID: su;
/// Map name: "error";
/// Description: Basa Sunda, Latin, Indonesia;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Henteu aya téks anu dilereskeun");
/// ```
pub(crate) const fn get_su_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Henteu aya téks anu dilereskeun"##),
    ],
}
}

/// su: Basa Sunda, Latin, Indonesia
pub(crate) const fn get_su_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_su_map_error),
    ],
}
}

/// Language ID: sv;
/// Map name: "error";
/// Description: svenska, latinska, Sverige;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ingen lokaliserad text hittades");
/// ```
pub(crate) const fn get_sv_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ingen lokaliserad text hittades"##),
    ],
}
}

/// sv: svenska, latinska, Sverige
pub(crate) const fn get_sv_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_sv_map_error),
    ],
}
}

/// Language ID: sw;
/// Map name: "error";
/// Description: Kiswahili, Kilatini, Tanzania;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Hakuna maandishi ya ndani yaliyopatikana");
/// ```
pub(crate) const fn get_sw_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Hakuna maandishi ya ndani yaliyopatikana"##),
    ],
}
}

/// sw: Kiswahili, Kilatini, Tanzania
pub(crate) const fn get_sw_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_sw_map_error),
    ],
}
}

/// Language ID: ta;
/// Map name: "error";
/// Description: தமிழ், தமிழ், இந்தியா;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "உள\u{bcd}ளூர\u{bcd}மயம\u{bbe}க\u{bcd}கப\u{bcd}பட\u{bcd}ட உரை எதுவும\u{bcd} கிடைக\u{bcd}கவில\u{bcd}லை");
/// ```
pub(crate) const fn get_ta_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"உள்ளூர்மயமாக்கப்பட்ட உரை எதுவும் கிடைக்கவில்லை"##),
    ],
}
}

/// ta: தமிழ், தமிழ், இந்தியா
pub(crate) const fn get_ta_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ta_map_error),
    ],
}
}

/// Language ID: te;
/// Map name: "error";
/// Description: తెలుగు, తెలుగు, భారతదేశం;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "స\u{c4d}థ\u{c3e}న\u{c3f}క\u{c40}కర\u{c3f}ంచ\u{c3f}న వచనం కనుగ\u{c4a}నబడల\u{c47}దు");
/// ```
pub(crate) const fn get_te_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"స్థానికీకరించిన వచనం కనుగొనబడలేదు"##),
    ],
}
}

/// te: తెలుగు, తెలుగు, భారతదేశం
pub(crate) const fn get_te_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_te_map_error),
    ],
}
}

/// Language ID: tg;
/// Map name: "error";
/// Description: тоҷикӣ, Кириллӣ, Тоҷикистон;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ягон матни маҳаллӣ ёфт нашуд");
/// ```
pub(crate) const fn get_tg_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ягон матни маҳаллӣ ёфт нашуд"##),
    ],
}
}

/// tg: тоҷикӣ, Кириллӣ, Тоҷикистон
pub(crate) const fn get_tg_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_tg_map_error),
    ],
}
}

/// Language ID: th;
/// Map name: "error";
/// Description: ไทย, ไทย, ไทย;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "ไม\u{e48}พบข\u{e49}อความท\u{e35}\u{e48}แปลเป\u{e47}นภาษาท\u{e49}องถ\u{e34}\u{e48}น");
/// ```
pub(crate) const fn get_th_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"ไม่พบข้อความที่แปลเป็นภาษาท้องถิ่น"##),
    ],
}
}

/// th: ไทย, ไทย, ไทย
pub(crate) const fn get_th_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_th_map_error),
    ],
}
}

/// Language ID: tl;
/// Map name: "error";
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Walang nahanap na naisalokal na teksto");
/// ```
pub(crate) const fn get_tl_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Walang nahanap na naisalokal na teksto"##),
    ],
}
}

/// tl: tl-Latn-PH
pub(crate) const fn get_tl_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_tl_map_error),
    ],
}
}

/// Language ID: tr;
/// Map name: "error";
/// Description: Türkçe, Latin, Türkiye;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Yerelleştirilmiş metin bulunamadı");
/// ```
pub(crate) const fn get_tr_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Yerelleştirilmiş metin bulunamadı"##),
    ],
}
}

/// tr: Türkçe, Latin, Türkiye
pub(crate) const fn get_tr_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_tr_map_error),
    ],
}
}

/// Language ID: ug;
/// Map name: "error";
/// Description: ئۇيغۇرچە, ئەرەب, جۇڭگو;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "يەرلىكلەشتۈرۈلگەن تېكىست تېپىلمىدى");
/// ```
pub(crate) const fn get_ug_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"يەرلىكلەشتۈرۈلگەن تېكىست تېپىلمىدى"##),
    ],
}
}

/// ug: ئۇيغۇرچە, ئەرەب, جۇڭگو
pub(crate) const fn get_ug_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ug_map_error),
    ],
}
}

/// Language ID: uk;
/// Map name: "error";
/// Description: українська, кирилиця, Україна;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Не знайдено локалізованого тексту");
/// ```
pub(crate) const fn get_uk_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Не знайдено локалізованого тексту"##),
    ],
}
}

/// uk: українська, кирилиця, Україна
pub(crate) const fn get_uk_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_uk_map_error),
    ],
}
}

/// Language ID: ur;
/// Map name: "error";
/// Description: اردو, عربی, پاکستان;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "کوئی مقامی متن نہیں ملا");
/// ```
pub(crate) const fn get_ur_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"کوئی مقامی متن نہیں ملا"##),
    ],
}
}

/// ur: اردو, عربی, پاکستان
pub(crate) const fn get_ur_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_ur_map_error),
    ],
}
}

/// Language ID: uz;
/// Map name: "error";
/// Description: o‘zbek, lotin, Oʻzbekiston;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Mahalliy matn topilmadi");
/// ```
pub(crate) const fn get_uz_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Mahalliy matn topilmadi"##),
    ],
}
}

/// uz: o‘zbek, lotin, Oʻzbekiston
pub(crate) const fn get_uz_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_uz_map_error),
    ],
}
}

/// Language ID: vi;
/// Map name: "error";
/// Description: Tiếng Việt, Chữ La tinh, Việt Nam;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Không tìm thấy văn bản bản địa hóa");
/// ```
pub(crate) const fn get_vi_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Không tìm thấy văn bản bản địa hóa"##),
    ],
}
}

/// vi: Tiếng Việt, Chữ La tinh, Việt Nam
pub(crate) const fn get_vi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_vi_map_error),
    ],
}
}

/// Language ID: xh;
/// Map name: "error";
/// Description: IsiXhosa, IsiLatin, EMzantsi Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Akukho sicatshulwa sendawo sifunyenwe");
/// ```
pub(crate) const fn get_xh_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Akukho sicatshulwa sendawo sifunyenwe"##),
    ],
}
}

/// xh: IsiXhosa, IsiLatin, EMzantsi Afrika
pub(crate) const fn get_xh_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_xh_map_error),
    ],
}
}

/// Language ID: yi;
/// Map name: "error";
/// Description: ייִדיש, העברעיש, וועלט;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "קיין לא\u{5b8}וקא\u{5b7}לייזד טעקסט געפ\u{5bf}ונען");
/// ```
pub(crate) const fn get_yi_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"קיין לאָוקאַלייזד טעקסט געפֿונען"##),
    ],
}
}

/// yi: ייִדיש, העברעיש, וועלט
pub(crate) const fn get_yi_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_yi_map_error),
    ],
}
}

/// Language ID: yo;
/// Map name: "error";
/// Description: Èdè Yorùbá, Èdè Látìn, Nàìjíríà;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Ko si ọrọ ti agbegbe ti a rii");
/// ```
pub(crate) const fn get_yo_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Ko si ọrọ ti agbegbe ti a rii"##),
    ],
}
}

/// yo: Èdè Yorùbá, Èdè Látìn, Nàìjíríà
pub(crate) const fn get_yo_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_yo_map_error),
    ],
}
}

/// Language ID: zh;
/// Map name: "error";
/// Description: 简体中文, 中国;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "没有找到本地化文本");
/// ```
pub(crate) const fn get_zh_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"没有找到本地化文本"##),
    ],
}
}

/// Language ID: zh;
/// Map name: "test";
/// Description: 简体中文, 中国;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("test", "quote");
///
/// assert_eq!(msg, "\"\"no\"''\"");
/// ```
pub(crate) const fn get_zh_map_test() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("quote", r##"""no"''""##),
    ],
}
}

/// zh: 简体中文, 中国
pub(crate) const fn get_zh_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (1, 0),
    ],
    entries: &[
        ("test", get_zh_map_test),
        ("error", get_zh_map_error),
    ],
}
}

/// Language ID: zh-Hant;
/// Map name: "error";
/// Description: 正體中文, 中國台灣;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "沒有找到本地化文本");
/// ```
pub(crate) const fn get_zh_hant_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"沒有找到本地化文本"##),
    ],
}
}

/// zh-Hant: 正體中文, 中國台灣
pub(crate) const fn get_zh_hant_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_zh_hant_map_error),
    ],
}
}

/// Language ID: zu;
/// Map name: "error";
/// Description: isiZulu, isi-Latin, iNingizimu Afrika;
///
/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default("error", "text-not-found");
///
/// assert_eq!(msg, "Awukho umbhalo wasendaweni otholakala");
/// ```
pub(crate) const fn get_zu_map_error() -> L10nMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("text-not-found", r##"Awukho umbhalo wasendaweni otholakala"##),
    ],
}
}

/// zu: isiZulu, isi-Latin, iNingizimu Afrika
pub(crate) const fn get_zu_map() -> SubLocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 0),
    ],
    entries: &[
        ("error", get_zu_map_error),
    ],
}
}

/// # Example
///
/// ```no_run
/// let map = locale_map();
///
/// for k in map.keys() {
///     println!("{k}")
/// }
///
/// map.get("en").map(|v| dbg!(v()));
/// ```
pub(crate) const fn locale_map() -> LocaleMap {
    ::phf::Map {
    key: 12913932095322966823,
    disps: &[
        (0, 2),
        (0, 2),
        (1, 1),
        (0, 74),
        (3, 68),
        (5, 53),
        (0, 7),
        (0, 47),
        (0, 14),
        (0, 7),
        (1, 39),
        (0, 105),
        (1, 91),
        (0, 8),
        (1, 5),
        (2, 49),
        (0, 0),
        (27, 100),
        (11, 32),
        (1, 82),
        (0, 3),
        (22, 83),
    ],
    entries: &[
        ("en-GB", get_en_gb_map),
        ("pa", get_pa_map),
        ("ca", get_ca_map),
        ("te", get_te_map),
        ("hr", get_hr_map),
        ("mi", get_mi_map),
        ("th", get_th_map),
        ("ug", get_ug_map),
        ("xh", get_xh_map),
        ("pl", get_pl_map),
        ("uz", get_uz_map),
        ("zh-Hant", get_zh_hant_map),
        ("si", get_si_map),
        ("gl", get_gl_map),
        ("yi", get_yi_map),
        ("bn", get_bn_map),
        ("la", get_la_map),
        ("zu", get_zu_map),
        ("sn", get_sn_map),
        ("ga", get_ga_map),
        ("iw", get_iw_map),
        ("ig", get_ig_map),
        ("sw", get_sw_map),
        ("ms", get_ms_map),
        ("da", get_da_map),
        ("lt", get_lt_map),
        ("ml", get_ml_map),
        ("tg", get_tg_map),
        ("eu", get_eu_map),
        ("el", get_el_map),
        ("lo", get_lo_map),
        ("mt", get_mt_map),
        ("eo", get_eo_map),
        ("st", get_st_map),
        ("sk", get_sk_map),
        ("fy", get_fy_map),
        ("mn", get_mn_map),
        ("is", get_is_map),
        ("ur", get_ur_map),
        ("lv", get_lv_map),
        ("am", get_am_map),
        ("ko", get_ko_map),
        ("fa", get_fa_map),
        ("sq", get_sq_map),
        ("yo", get_yo_map),
        ("pt", get_pt_map),
        ("bg", get_bg_map),
        ("ja", get_ja_map),
        ("bs", get_bs_map),
        ("ky", get_ky_map),
        ("tl", get_tl_map),
        ("fr", get_fr_map),
        ("en", get_en_map),
        ("az", get_az_map),
        ("or", get_or_map),
        ("de", get_de_map),
        ("he", get_he_map),
        ("be", get_be_map),
        ("my", get_my_map),
        ("hu", get_hu_map),
        ("es", get_es_map),
        ("mg", get_mg_map),
        ("sv", get_sv_map),
        ("sm", get_sm_map),
        ("af", get_af_map),
        ("sl", get_sl_map),
        ("vi", get_vi_map),
        ("ar", get_ar_map),
        ("sd", get_sd_map),
        ("gd", get_gd_map),
        ("mk", get_mk_map),
        ("tr", get_tr_map),
        ("ht", get_ht_map),
        ("cs", get_cs_map),
        ("ta", get_ta_map),
        ("km", get_km_map),
        ("ku", get_ku_map),
        ("gu", get_gu_map),
        ("zh", get_zh_map),
        ("ceb", get_ceb_map),
        ("nl", get_nl_map),
        ("ha", get_ha_map),
        ("su", get_su_map),
        ("et", get_et_map),
        ("ne", get_ne_map),
        ("fi", get_fi_map),
        ("id", get_id_map),
        ("ka", get_ka_map),
        ("kn", get_kn_map),
        ("lb", get_lb_map),
        ("ps", get_ps_map),
        ("mr", get_mr_map),
        ("hi", get_hi_map),
        ("hy", get_hy_map),
        ("uk", get_uk_map),
        ("ru", get_ru_map),
        ("ro", get_ro_map),
        ("it", get_it_map),
        ("sr", get_sr_map),
        ("ny", get_ny_map),
        ("co", get_co_map),
        ("haw", get_haw_map),
        ("jw", get_jw_map),
        ("so", get_so_map),
        ("kk", get_kk_map),
        ("cy", get_cy_map),
        ("no", get_no_map),
    ],
}
}

/// # Example
///
/// ```no_run
/// let loader = glossa::MapLoader::new(locale_hashmap());
///
/// dbg!(&loader);
/// ```
pub(crate) fn locale_hashmap() -> LocaleHashMap {
    use lang_id_consts::*;

    HashMap::from_iter([
        (unsafe { get_af() }, get_af_map()),
        (unsafe { get_am() }, get_am_map()),
        (unsafe { get_ar() }, get_ar_map()),
        (unsafe { get_az() }, get_az_map()),
        (unsafe { get_be() }, get_be_map()),
        (unsafe { get_bg() }, get_bg_map()),
        (unsafe { get_bn() }, get_bn_map()),
        (unsafe { get_bs() }, get_bs_map()),
        (unsafe { get_ca() }, get_ca_map()),
        (unsafe { get_ceb() }, get_ceb_map()),
        (unsafe { get_co() }, get_co_map()),
        (unsafe { get_cs() }, get_cs_map()),
        (unsafe { get_cy() }, get_cy_map()),
        (unsafe { get_da() }, get_da_map()),
        (unsafe { get_de() }, get_de_map()),
        (unsafe { get_el() }, get_el_map()),
        (unsafe { get_en() }, get_en_map()),
        (unsafe { get_en_gb() }, get_en_gb_map()),
        (unsafe { get_eo() }, get_eo_map()),
        (unsafe { get_es() }, get_es_map()),
        (unsafe { get_et() }, get_et_map()),
        (unsafe { get_eu() }, get_eu_map()),
        (unsafe { get_fa() }, get_fa_map()),
        (unsafe { get_fi() }, get_fi_map()),
        (unsafe { get_fr() }, get_fr_map()),
        (unsafe { get_fy() }, get_fy_map()),
        (unsafe { get_ga() }, get_ga_map()),
        (unsafe { get_gd() }, get_gd_map()),
        (unsafe { get_gl() }, get_gl_map()),
        (unsafe { get_gu() }, get_gu_map()),
        (unsafe { get_ha() }, get_ha_map()),
        (unsafe { get_haw() }, get_haw_map()),
        (unsafe { get_he() }, get_he_map()),
        (unsafe { get_hi() }, get_hi_map()),
        (unsafe { get_hr() }, get_hr_map()),
        (unsafe { get_ht() }, get_ht_map()),
        (unsafe { get_hu() }, get_hu_map()),
        (unsafe { get_hy() }, get_hy_map()),
        (unsafe { get_id() }, get_id_map()),
        (unsafe { get_ig() }, get_ig_map()),
        (unsafe { get_is() }, get_is_map()),
        (unsafe { get_it() }, get_it_map()),
        (unsafe { get_iw() }, get_iw_map()),
        (unsafe { get_ja() }, get_ja_map()),
        (unsafe { get_jw() }, get_jw_map()),
        (unsafe { get_ka() }, get_ka_map()),
        (unsafe { get_kk() }, get_kk_map()),
        (unsafe { get_km() }, get_km_map()),
        (unsafe { get_kn() }, get_kn_map()),
        (unsafe { get_ko() }, get_ko_map()),
        (unsafe { get_ku() }, get_ku_map()),
        (unsafe { get_ky() }, get_ky_map()),
        (unsafe { get_la() }, get_la_map()),
        (unsafe { get_lb() }, get_lb_map()),
        (unsafe { get_lo() }, get_lo_map()),
        (unsafe { get_lt() }, get_lt_map()),
        (unsafe { get_lv() }, get_lv_map()),
        (unsafe { get_mg() }, get_mg_map()),
        (unsafe { get_mi() }, get_mi_map()),
        (unsafe { get_mk() }, get_mk_map()),
        (unsafe { get_ml() }, get_ml_map()),
        (unsafe { get_mn() }, get_mn_map()),
        (unsafe { get_mr() }, get_mr_map()),
        (unsafe { get_ms() }, get_ms_map()),
        (unsafe { get_mt() }, get_mt_map()),
        (unsafe { get_my() }, get_my_map()),
        (unsafe { get_ne() }, get_ne_map()),
        (unsafe { get_nl() }, get_nl_map()),
        (unsafe { get_no() }, get_no_map()),
        (unsafe { get_ny() }, get_ny_map()),
        (unsafe { get_or() }, get_or_map()),
        (unsafe { get_pa() }, get_pa_map()),
        (unsafe { get_pl() }, get_pl_map()),
        (unsafe { get_ps() }, get_ps_map()),
        (unsafe { get_pt() }, get_pt_map()),
        (unsafe { get_ro() }, get_ro_map()),
        (unsafe { get_ru() }, get_ru_map()),
        (unsafe { get_sd() }, get_sd_map()),
        (unsafe { get_si() }, get_si_map()),
        (unsafe { get_sk() }, get_sk_map()),
        (unsafe { get_sl() }, get_sl_map()),
        (unsafe { get_sm() }, get_sm_map()),
        (unsafe { get_sn() }, get_sn_map()),
        (unsafe { get_so() }, get_so_map()),
        (unsafe { get_sq() }, get_sq_map()),
        (unsafe { get_sr() }, get_sr_map()),
        (unsafe { get_st() }, get_st_map()),
        (unsafe { get_su() }, get_su_map()),
        (unsafe { get_sv() }, get_sv_map()),
        (unsafe { get_sw() }, get_sw_map()),
        (unsafe { get_ta() }, get_ta_map()),
        (unsafe { get_te() }, get_te_map()),
        (unsafe { get_tg() }, get_tg_map()),
        (unsafe { get_th() }, get_th_map()),
        (unsafe { get_tl() }, get_tl_map()),
        (unsafe { get_tr() }, get_tr_map()),
        (unsafe { get_ug() }, get_ug_map()),
        (unsafe { get_uk() }, get_uk_map()),
        (unsafe { get_ur() }, get_ur_map()),
        (unsafe { get_uz() }, get_uz_map()),
        (unsafe { get_vi() }, get_vi_map()),
        (unsafe { get_xh() }, get_xh_map()),
        (unsafe { get_yi() }, get_yi_map()),
        (unsafe { get_yo() }, get_yo_map()),
        (unsafe { get_zh() }, get_zh_map()),
        (unsafe { get_zh_hant() }, get_zh_hant_map()),
        (unsafe { get_zu() }, get_zu_map()),
    ])
}
