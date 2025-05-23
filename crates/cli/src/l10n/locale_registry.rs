pub(crate) const fn all_locales() -> [super::lang_id::LangID; 2] {
  #[allow(unused_imports)]
  use super::lang_id::RawID;
  use super::lang_id::consts::*;
  [lang_id_en(), lang_id_zh()]
}
