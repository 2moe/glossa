use std::{
  fs::File,
  io::{self, BufWriter, Write},
};

use compact_str::format_compact;
use glossa_shared::PhfTupleKey;
use phf_codegen::OrderedMap;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tap::Tap;

use crate::{
  MiniStr,
  generator::{
    Generator, MapType, flattening::L10nBTreeMap, output_bincode::create_buf_writer,
  },
};

impl<'h> Generator<'_, 'h> {
  pub fn output_phf(&'h self, map_type: MapType) -> io::Result<()> {
    let vis_fn = self.get_visibility().as_str();

    map_type
      .get_non_template_maps(self)?
      .par_iter()
      .filter(|(_, data)| !data.is_empty())
      .map(|(lang, map_entry)| {
        let new_phf_map = assemble_phf_map(map_entry);
        (lang, new_phf_map)
      })
      .try_for_each(|(lang, mut map)| {
        writeln!(
          &mut self.create_rs_mod_file(lang)?,
          "{vis_fn} const fn map<'k>() -> super::PhfL10nOrderedMap<'k>
    {{\n{code}\n}}",
          code = map
            .phf_path("super::phf")
            .build()
        )
      })

    // fn l10n_maps() -> Box<[(lang_id::LangID, PhfL10nOrderedMap<'static>)]> {
    //  use lang_id::consts::*;
    //  vec![
    //    #[cfg(feature = "l10n_en")]
    //    (lang_id_en(), l10n_en::maps),
    //
    //    #[cfg(feature = "l10n_zh")]
    //    (lang_id_zh(), l10n_zh::maps),
    // ].into_boxed_slice()
    // };
  }

  pub(crate) fn create_rs_mod_file<D: core::fmt::Display>(
    &self,
    language: &D,
  ) -> io::Result<BufWriter<File>> {
    let mod_prefix = self.get_mod_prefix();
    let rs_file_name =
      format_compact!("{mod_prefix}{}.rs", to_lower_snake_case(language));

    eprintln!(
      "#[cfg(feature = \"{mod_prefix}{language}\")]\n\
      mod {rs_file_name}\n"
    );

    let out_dir = self.get_outdir().as_deref();

    create_buf_writer(out_dir, rs_file_name)
  }
}

fn assemble_phf_map(map_entry: &L10nBTreeMap) -> OrderedMap<PhfTupleKey<'_>> {
  map_entry
    .iter()
    .map(|((name, k), v)| {
      let tuple_key = PhfTupleKey(name.as_str(), k.as_str());
      let value = format_compact!(r##########"r#####"{v}"#####"##########);
      (tuple_key, value)
    })
    .fold(OrderedMap::new(), |mut acc, x| {
      acc.entry(x.0, &x.1);
      acc
    })
}

/// - en.US => en_us
/// - en-US => en_us
/// - en-Latn-US => en_latn_us
/// - zh-Hans-CN => zh_hans_cn
pub fn to_lower_snake_case<D: core::fmt::Display>(id: D) -> MiniStr {
  format_compact!("{id}")
    .tap_mut(|s| s.make_ascii_lowercase())
    .chars()
    .map(|c| match c {
      '-' | '.' => '_',
      c => c,
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use anyhow::Result as AnyResult;

  use super::*;
  use crate::generator::dbg_generator::en_generator;

  #[ignore]
  #[test]
  fn test_build_phf() -> AnyResult<()> {
    let generator = en_generator();

    generator.output_phf(MapType::Regular)?;
    // generator
    // .output_phf()

    Ok(())
  }

  pub(crate) const fn en_map<'m>() -> phf::OrderedMap<PhfTupleKey<'m>, &'static str>
  {
    phf::OrderedMap {
      key: 12913932095322966823,
      disps: &[(0, 0)],
      idxs: &[0],
      entries: &[(
        super::PhfTupleKey(r#"error"#, r##"text-not-found"##),
        r###"No localised text found"###,
      )],
    }
  }

  #[ignore]
  #[test]
  fn test_get_phf_en_map() {
    let map = en_map();
    let v = map.get(&PhfTupleKey("error", "text-not-found"));
    dbg!(v);
  }
}
