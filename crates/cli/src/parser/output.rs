use glossa_codegen::{AnyResult, Generator, glossa_shared::tap::Pipe};

use super::puts;
use crate::options::Cli;

pub(crate) fn output_data<'a>(
  args: &Cli,
  generator: &'a Generator<'a>,
) -> AnyResult<()> {
  let map_type = args.get_map_type();
  let out_args = args.get_generator();
  log::info!("output dir: {:?}", out_args.get_outdir());

  if *out_args.get_output_locales_fn() {
    generator
      .output_locales_fn(*map_type, true)?
      .pipe_ref(puts)
  }

  if *out_args.get_output_json() {
    map_type
      .output_json(generator)?
      .pipe_ref(puts)
  }

  if *out_args.get_output_toml() {
    map_type
      .output_toml(generator)?
      .pipe_ref(puts)
  }

  // -------
  if *out_args.get_output_bincode() {
    generator.output_bincode(*map_type)?
  }

  if *out_args.get_output_bincode_all_in_one() {
    generator.output_bincode_all_in_one(*map_type)?
  }

  // -------
  if *out_args.get_output_match_fn() {
    generator.output_match_fn(*map_type)?
  }

  if *out_args.get_output_match_fn_all_in_one() {
    generator
      .output_match_fn_all_in_one(*map_type)?
      .pipe_ref(puts)
  }
  if *out_args.get_output_match_fn_all_in_one_by_language() {
    generator
      .output_match_fn_all_in_one_by_language(*map_type)?
      .pipe_ref(puts)
  }
  if *out_args.get_output_match_fn_all_in_one_by_language_and_key() {
    generator
      .output_match_fn_all_in_one_by_language_and_key(*map_type)?
      .pipe_ref(puts)
  }
  // -------
  if *out_args.get_output_phf() {
    generator.output_phf(*map_type)?
  }
  if *out_args.get_output_phf_all_in_one() {
    generator
      .output_phf_all_in_one(*map_type)?
      .pipe_ref(puts)
  }
  // -------
  Ok(())
}
