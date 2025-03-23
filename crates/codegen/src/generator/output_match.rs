use std::io::{self, Write};

use compact_str::{ToCompactString, format_compact};
use tap::{Pipe, Tap};

use crate::{
  MiniStr,
  generator::{Generator, MapType},
};

impl<'h> Generator<'_, 'h> {
  // pub(crate) fn less_or_eq_to_match_bound(&self) -> bool {
  //   self
  //     .get_l10n_res_map()
  //     .values()
  //     .flatten()
  //     .flat_map(|x| x.get_data())
  //     .count()
  //     <= self.match_bound
  // }

  pub fn output_all_in_one_match_fn(
    &'h self,
    map_type: MapType,
  ) -> io::Result<String> {
    const S_HEADER: &str = r##"const fn map(lang: &[u8], map_name: &[u8], key: &[u8]) ->
  &'static str {
    match (lang, map_name, key) {
    "##;

    let new_header = || self.new_match_fn_header(S_HEADER);

    map_type
      .get_non_template_maps(self)?
      .iter()
      .filter(|(_, data)| !data.is_empty())
      .flat_map(|(lang, map_entry)| {
        map_entry
          .iter()
          .map(move |e| (lang, e))
      })
      .fold(new_header(), |mut acc, (lang, ((name, key), value))| {
        [
          "(b\"",
          lang
            .to_compact_string()
            .as_str(),
          "\", ",
          key_as_bytes(name).as_str(),
          ", ",
          key_as_bytes(key).as_str(),
          ") => r#####",
          "\"",
          value.as_str(),
          "\"",
          r###########"#####,"###########,
          "\n",
        ]
        .map(|s| acc.push_str(s));
        acc
      })
      .tap_mut(|x| x.push_str("    _ => \"\",\n}}"))
      .pipe(Ok)
  }

  fn new_match_fn_header(&'h self, header: &str) -> String {
    let vis_fn = self.get_visibility().as_str();
    String::with_capacity(8192).tap_mut(|buf| {
      [vis_fn, " ", header].map(|s| buf.push_str(s));
    })
  }

  pub fn output_match_fn(&'h self, map_type: MapType) -> io::Result<()> {
    const HEADER: &str = r##"const fn map(map_name: &[u8], key: &[u8]) -> &'static str {
    match (map_name, key) {
    "##;

    let new_header = || self.new_match_fn_header(HEADER);

    map_type
      .get_non_template_maps(self)?
      .iter()
      .filter(|(_, data)| !data.is_empty())
      .map(|(lang, map_entry)| {
        let match_fn_string = map_entry
          .iter()
          .fold(
            new_header(), //
            |mut acc, ((map_name, data_k), data_v)| {
              [
                "(",
                key_as_bytes(map_name).as_str(),
                ", ",
                key_as_bytes(data_k).as_str(),
                r#") => r#####"#,
                "\"", // "
                data_v.as_str(),
                "\"",                            // "
                r###########"#####,"###########, // ###,
                "\n",
              ]
              .map(|s| acc.push_str(s));
              acc
            },
          )
          .tap_mut(|buf| buf.push_str("    _ => \"\",\n}}"));
        (lang, match_fn_string)
      })
      .try_for_each(|(lang, s)| {
        self
          .create_rs_mod_file(lang)?
          .write_all(s.as_bytes())
      })
  }
}

fn key_as_bytes(key: &str) -> MiniStr {
  use format_compact as fmt;

  match key.is_ascii() {
    true => fmt!("b{key:?}"),
    _ => fmt!("{:?}", key.as_bytes()),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    AnyResult,
    generator::dbg_generator::{en_generator, new_generator},
  };

  #[ignore]
  #[test]
  fn test_output_match_fn() -> AnyResult<()> {
    new_generator().output_match_fn(MapType::Regular)?;
    Ok(())
  }

  #[ignore]
  #[test]
  fn test_output_a1o_match_fn() -> AnyResult<()> {
    en_generator()
      .output_all_in_one_match_fn(MapType::Regular)?
      .tap(|s| println!("{s}"));
    Ok(())
  }

  const fn map(language: &[u8], map_name: &[u8], key: &[u8]) -> &'static str {
    match (language, map_name, key) {
      (b"de", b"error", b"text-not-found") => {
        r###"Kein lokalisierter Text gefunden"###
      }
      (b"el", b"error", b"text-not-found") => {
        r###"Δεν βρέθηκε κανένα τοπικό κείμενο"###
      }
      (b"en", b"error", b"text-not-found") => r###"No localized text found"###,
      (b"en", b"test", [240, 159, 145, 139, 240, 159, 140, 144]) => {
        r###"hello world"###
      }
      (b"en", b"test", b"hello") => r###"world"###,
      (b"en-GB", b"error", b"text-not-found") => r###"No localised text found"###,
      (b"zh", b"error", b"text-not-found") => r###"未找到本地化文本"###,
      (b"zh", b"test", b"quote") => r###"""no"''""###,
      (b"zh-Hant", b"error", b"text-not-found") => r###"沒有找到本地化文本"###,
      _ => "",
    }
  }

  #[ignore]
  #[test]
  fn test_get_match_map() {
    const S: &str = map(b"de", b"error", "text-not-found".as_bytes());
    assert!(!S.is_empty());
    println!("{S}");
  }
}
