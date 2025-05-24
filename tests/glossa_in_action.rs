mod cli_logger;

use std::path::Path;

use cli_logger::init_logger;
use compact_str::ToCompactString;
use glossa::{self, LangID, LocaleContext, MiniStr, traits::ChainProvider};
use glossa_shared::{
  PhfL10nOrderedMap, PhfTupleKey,
  lang_id::{self, consts::lang_id_en_150},
};
use itertools::Itertools;
use tap::Pipe;
// use testutils::new_once_lock;

pub const fn all_locales() -> [lang_id::LangID; 10] {
  use lang_id::consts::*;
  [
    lang_id_cs(),
    lang_id_de(),
    lang_id_en(),
    lang_id_es(),
    lang_id_fr(),
    lang_id_ja(),
    lang_id_ko(),
    lang_id_ru(),
    // lang_id_zh(),
    lang_id_zh_hant(),
    lang_id_zh_pinyin(),
  ]
}

pub const fn map(language: &[u8], key: &[u8]) -> &'static str {
  match (language, key) {
    (b"cs", b"cancel") => r#####"Zrušit"#####,
    (b"cs", b"no") => r#####"Ne"#####,
    (b"cs", b"yes") => r#####"Ano"#####,
    (b"de", b"cancel") => r#####"Abbrechen"#####,
    (b"de", b"no") => r#####"Nein"#####,
    (b"de", b"yes") => r#####"Ja"#####,
    (b"en", b"cancel") => r#####"Cancel"#####,
    (b"en", b"confirm") => r#####"Confirm"#####,
    (b"en", b"no") => r#####"No"#####,
    (b"en", b"ok") => r#####"OK"#####,
    (b"en", b"yes") => r#####"Yes"#####,
    (b"es", b"cancel") => r#####"Cancelar"#####,
    (b"es", b"ok") => r#####"Aceptar"#####,
    (b"es", b"yes") => r#####"Sí"#####,
    (b"fr", b"cancel") => r#####"Annuler"#####,
    (b"fr", b"no") => r#####"Non"#####,
    (b"fr", b"yes") => r#####"Oui"#####,
    (b"ja", b"cancel") => r#####"取消"#####,
    (b"ja", b"no") => r#####"いいえ"#####,
    (b"ja", b"ok") => r#####"了解"#####,
    (b"ja", b"yes") => r#####"はい"#####,
    (b"ko", b"cancel") => r#####"취소"#####,
    (b"ko", b"no") => r#####"아니오"#####,
    (b"ko", b"ok") => r#####"확인"#####,
    (b"ko", b"yes") => r#####"예"#####,
    (b"ru", b"no") => r#####"Нет"#####,
    (b"ru", b"yes") => r#####"Да"#####,
    (b"zh-Hant", b"cancel") => r#####"取消"#####,
    (b"zh-Hant", b"no") => r#####"否"#####,
    (b"zh-Hant", b"ok") => r#####"確定"#####,
    (b"zh-Hant", b"yes") => r#####"是"#####,
    (b"zh-Latn-CN", b"cancel") => r#####"QuXiao"#####,
    (b"zh-Latn-CN", b"no") => r#####"Fou"#####,
    (b"zh-Latn-CN", b"ok") => r#####"QueDing"#####,
    (b"zh-Latn-CN", b"yes") => r#####"Shi"#####,
    _ => "",
  }
}

trait GetL10nText: ChainProvider {
  fn try_get_by_key<'t>(&self, key: &[u8]) -> Option<&'t str> {
    let lookup = |(language, key)| match map(language, key) {
      "" => None,
      s => Some(s),
    };

    self
      .provide_chain()?
      .iter()
      .map(|id| (id.as_bytes(), key))
      .find_map(lookup)
  }
}

impl GetL10nText for LocaleContext {}

#[ignore]
#[test]
pub(crate) fn print_l10n_text() {
  init_logger(false);
  let new_ctx = || LocaleContext::default().with_all_locales(all_locales());

  // #[cfg(any(target_os = "macos", target_os = "linux"))]
  let set_env_lang = |value| unsafe { std::env::set_var("LANG", value) };

  let display = |ctx: &LocaleContext, key: &str| {
    let text = ctx
      .try_get_by_key(key.as_bytes())
      .unwrap_or_else(|| panic!("{}", glossa::Error::new_text_not_found(key)));
    println!("{key}: {text}")
  };

  {
    // set_env_lang("gsw_CH.UTF-8");
    let ctx = new_ctx()
      .with_current_locale(Some(glossa_shared::lang_id::consts::lang_id_gsw()));
    // [("de", 26)]
    for key in ["yes", "no", "ok", "cancel"] {
      display(&ctx, key)
    }
  }
  // Output:
  //   yes: Ja
  //   no: Nein
  //   ok: OK
  //   cancel: Abbrechen

  {
    set_env_lang("zh_MO.UTF-8");
    log::debug!("\n---\n--- current locale => zh-MO");
    // [("zh-Hant", 43), ("zh", 31), ("zh-Latn-CN", 22)]
    let ctx = new_ctx().with_current_locale(None);
    for key in ["yes", "no", "ok", "cancel", "confirm"] {
      display(&ctx, key)
    }
  }
  // Output:
  //   yes: 是
  //   no: 否
  //   ok: 確定
  //   cancel: 取消
  //   confirm: Confirm
}

#[ignore]
#[test]
// en-GB, zh-pinyin
fn test_bilingual() {
  init_logger(true);

  use glossa_shared::lang_id::consts::{lang_id_en_gb, lang_id_zh_pinyin};

  let new_ctx = |id| {
    LocaleContext::default()
      .with_all_locales(all_locales())
      .with_current_locale(Some(id))
  };
  let zh_pinyin_ctx = new_ctx(lang_id_zh_pinyin());
  let en_gb_ctx = new_ctx(lang_id_en_gb());

  fn get_text<'a>(ctx: &LocaleContext, key: &str) -> Option<&'a str> {
    let key_bytes = key.as_bytes();
    let lookup = |language| match map(language, key_bytes) {
      "" => None,
      x => Some(x),
    };

    ctx
      .get_or_try_init_chain()?
      .iter()
      .map(|id| id.as_bytes())
      .find_map(lookup)
  }

  let get_cancel_text = |ctx| get_text(ctx, "cancel").unwrap_or_default();

  let zh_pinyin_text = get_cancel_text(&zh_pinyin_ctx);
  let en_gb_text = get_cancel_text(&en_gb_ctx);

  let text = match zh_pinyin_text == en_gb_text {
    true => zh_pinyin_text.into(),
    _ => glossa_shared::fmt_compact!("{en_gb_text}. {zh_pinyin_text}"),
  };

  assert_eq!(text, "Cancel. QuXiao")
}

pub(crate) const fn phf_es_map() -> PhfL10nOrderedMap {
  use PhfTupleKey as Key;
  phf::OrderedMap {
    key: 12913932095322966823,
    disps: &[(0, 0)],
    idxs: &[1, 3, 2, 4, 0],
    entries: &[
      (
        Key(r#"error"#, r##"text-not-found"##),
        r#####"No se encontró texto localizado"#####,
      ),
      (Key(r#"yes-no"#, r##"cancel"##), r#####"Cancelar"#####),
      (Key(r#"yes-no"#, r##"no"##), r#####"No"#####),
      (Key(r#"yes-no"#, r##"ok"##), r#####"Aceptar"#####),
      (Key(r#"yes-no"#, r##"yes"##), r#####"Sí"#####),
    ],
  }
}

fn try_get_phf_value(key: &'static str) -> Option<&'static str> {
  let map = phf_es_map();
  let lookup = |k| map.get(&k);

  PhfTupleKey("yes-no", key)
    .pipe(lookup)
    .copied()
}

#[ignore]
#[test]
fn test_get_phf_map_value() -> glossa::Result<()> {
  for key in ["yes", "cancel"] {
    println!("{:?}", try_get_phf_value(key));
  }

  Ok(())
}

#[ignore]
#[test]
fn test_get_bincode() -> glossa_shared::glossa_dsl::Result<()> {
  let path = Path::new("crates/codegen/tmp/all_regular.bincode");
  let map = glossa_shared::decode::file::decode_file_to_maps(path)?;
  let lookup = |language, tuple_key| {
    map
      .get(language)?
      .get(&tuple_key)
  };
  let ctx = LocaleContext::default()
    .with_current_locale(Some(lang_id_en_150()))
    .with_all_locales(
      ["en-GB", "en"]
        .iter()
        .flat_map(|x| x.parse::<LangID>())
        .collect_vec(),
    );

  let try_get = |map_name: MiniStr, key: MiniStr| {
    ctx
      .get_or_try_init_chain()?
      .iter()
      .find_map(|id| lookup(id, (map_name.clone(), key.clone())))
  };

  let s = try_get(
    "error".to_compact_string(),
    "text-not-found".to_compact_string(),
  )
  .map(|x| x.as_str());

  assert_eq!(s, Some("No localised text found"));

  Ok(())
}
