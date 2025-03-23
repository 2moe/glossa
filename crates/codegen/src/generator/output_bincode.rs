use std::{
  fs::File,
  io::{self, BufWriter},
  path::Path,
};

use compact_str::format_compact;
use glossa_shared::MiniStr;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tap::Pipe;

use crate::{
  AnyResult,
  generator::{Generator, MapType},
  resources::L10nResMap,
};

impl<'h> Generator<'_, 'h> {
  pub(crate) fn get_l10n_res_map(&self) -> &L10nResMap {
    self
      .get_resources()
      .get_or_init_data()
  }

  fn encode_bincode<T, D>(&self, lang_id: D, data: T) -> AnyResult<()>
  where
    T: serde::Serialize,
    D: core::fmt::Display,
  {
    bincode::serde::encode_into_std_write(
      data,
      &mut self.create_bincode_file(lang_id)?,
      bincode::config::standard(),
    )?;
    Ok(())
  }

  /// `>` "outdir()/all{bincode_suffix}.bincode"
  pub fn output_all_in_one_bincode(&'h self, map_type: MapType) -> AnyResult<()> {
    let all = "all";

    if map_type.is_template() {
      return match self.get_or_init_template_maps() {
        x if x.is_empty() => Ok(()),
        data => self.encode_bincode(all, data),
      };
    }

    let data = map_type.get_non_template_maps(self)?;
    self.encode_bincode(all, data)
  }

  /// `>` "outdir()/{language}{bincode_suffix}.bincode"
  pub fn output_bincode(&'h self, map_type: MapType) -> AnyResult<()> {
    if map_type.is_template() {
      return match self.get_or_init_template_maps() {
        x if x.is_empty() => Ok(()),
        iter => iter
          .par_iter()
          .filter(|(_, data)| !data.is_empty())
          .try_for_each(|(lang, data)| self.encode_bincode(lang, data)),
      };
    }

    map_type
      .get_non_template_maps(self)?
      .par_iter()
      .filter(|(_, data)| !data.is_empty())
      .try_for_each(|(lang, data)| self.encode_bincode(lang, data))
  }

  pub(crate) fn create_bincode_file<D: core::fmt::Display>(
    &self,
    language: D,
  ) -> io::Result<BufWriter<File>> {
    let suffix = self.get_bincode_suffix();
    let bincode_name = format_compact!("{language}{suffix}.bincode");
    let out_dir = self.get_outdir().as_deref();

    create_buf_writer(out_dir, bincode_name)
  }
}

pub(crate) fn create_buf_writer(
  out_dir: Option<&Path>,
  bincode_name: MiniStr,
) -> io::Result<BufWriter<File>> {
  out_dir
    .ok_or_else(|| io::Error::other("Invalid outdir"))?
    .join(bincode_name)
    .pipe(File::create)?
    .pipe(BufWriter::new)
    .pipe(Ok)
}

#[cfg(test)]
mod tests {
  use std::io::BufReader;

  use testutils::simple_benchmark;

  use super::*;
  use crate::generator::{dbg_generator::new_generator, flattening::L10nMaps};

  #[ignore]
  #[test]
  fn test_output_tmpl_maps_to_bincode() -> AnyResult<()> {
    new_generator()
      .with_bincode_suffix(".tmpl".into())
      .output_bincode(MapType::Template)
  }

  #[ignore]
  #[test]
  fn test_single_bincode_file() -> AnyResult<()> {
    new_generator()
      .with_bincode_suffix("_all-in-one".into())
      .output_all_in_one_bincode(MapType::Regular)
  }

  #[ignore]
  #[test]
  fn test_deser_bincode_file() -> AnyResult<()> {
    let mut file = Path::new("tmp")
      .join("und_all-in-one.bincode")
      .pipe(File::open)?
      .pipe(BufReader::new);

    let mut data = || {
      bincode::serde::decode_from_std_read::<L10nMaps, _, _>(
        &mut file,
        bincode::config::standard(),
      )
    };

    simple_benchmark(|| {
      let _ = data();
    });

    // dbg!(data);

    Ok(())
  }
}
