use glossa_codegen::{
  AnyResult, Generator,
  glossa_shared::{
    display::{puts, puts_dbg},
    tap::Pipe,
  },
};

use crate::options::Cli;

fn output_to_stdout<'a>(args: &Cli, generator: &'a Generator<'a>) -> AnyResult<()> {
  let map_type = args.get_map_type();
  let out_args = args.get_generator();

  if *out_args.get_output_ron() {
    map_type
      .output_ron(generator)?
      .pipe_ref(puts)
  }

  if *out_args.get_output_locales_fn() {
    generator
      .output_locales_fn(*map_type, true)?
      .pipe_ref(puts)
  }

  if *out_args.get_output_raw_locales() {
    generator
      .collect_raw_locales(*map_type)?
      .pipe_ref(puts_dbg)
  }

  if *out_args.get_output_mod_rs() {
    generator
      .output_mod_rs(*map_type)?
      .pipe_ref(puts)
  }

  if *out_args.get_output_cargo_features() {
    generator
      .output_cargo_features(*map_type)?
      .pipe_ref(puts)
  }

  if *out_args.get_output_router_for_match_fns() {
    generator
      .output_router_for_match_fns(*map_type, true)?
      .pipe_ref(puts)
  }

  if *out_args.get_output_router_for_match_fns_without_map_name() {
    generator
      .output_router_for_match_fns(*map_type, false)?
      .pipe_ref(puts)
  }

  if *out_args.get_output_router_for_phf_maps() {
    generator
      .output_router_for_phf_maps(*map_type, true)?
      .pipe_ref(puts)
  }
  if *out_args.get_output_router_for_phf_maps_without_map_name() {
    generator
      .output_router_for_phf_maps(*map_type, false)?
      .pipe_ref(puts)
  }

  Ok(())
}

pub(crate) fn output_data<'a>(
  args: &Cli,
  generator: &'a Generator<'a>,
) -> AnyResult<()> {
  output_to_stdout(args, generator)?;

  let map_type = args.get_map_type();
  let out_args = args.get_generator();

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

  if *out_args.get_output_match_fn_without_map_name() {
    generator.output_match_fn_without_map_name(*map_type)?
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
  if *out_args.get_output_match_fn_all_in_one_without_map_name() {
    generator
      .output_match_fn_all_in_one_without_map_name(*map_type)?
      .pipe_ref(puts)
  }
  // -------
  if *out_args.get_output_phf() {
    generator.output_phf(*map_type)?
  }

  if *out_args.get_output_phf_without_map_name() {
    generator.output_phf_without_map_name(*map_type)?
  }

  if *out_args.get_output_phf_all_in_one() {
    generator
      .output_phf_all_in_one(*map_type)?
      .pipe_ref(puts)
  }
  // -------
  Ok(())
}
