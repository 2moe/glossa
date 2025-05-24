pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Ĉu aktivigi fonon"#####,
    b"base_name" => r#####"Baza nomo de Highlight-Map"#####,
    b"bincode_suffix" => r#####"Dosiersufikso de bincode"#####,
    b"custom_syntax_set" => r#####"Propra sintaksa aro-dosiero"#####,
    b"custom_theme_set" => r#####"Propra tema aro-dosiero"#####,
    b"display_config_dir" => r#####"Montri agordan dosierujon de Glossa"#####,
    b"dsl_suffix" => r#####"Dosiersufikso de DSL (defaŭlte ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Nigra listo: listigitaj lingvo-IDoj ne pravaloriziĝas"#####
    }
    b"exclude_map_names" => r#####"Listigitaj map-nomoj ne pravaloriziĝas"#####,
    b"include_languages" => {
      r#####"Blanka listo: nur listigitaj lingvo-IDoj pravaloriziĝas"#####
    }
    b"include_map_names" => r#####"Nur listigitaj map-nomoj pravaloriziĝas"#####,
    b"input" => r#####"Fonta dosierujo de lokaligaj rimedoj"#####,
    b"list_all_syntaxes" => r#####"Montri ĉiujn sintaksnomojn kun etendaĵoj"#####,
    b"list_all_themes" => r#####"Montri ĉiujn temnomojn"#####,
    b"mod_prefix" => r#####"Dosierprefikso de mod (defaŭlte "l10n_")"#####,
    b"outdir" => r#####"Eliga dosierujo"#####,
    b"output_bincode" => {
      r#####"Generi apartajn bincode-dosierojn por ĉiu lingvo"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Eligi ĉiujn bincode-ojn en ununuran dosieron"#####
    }
    b"output_locales_fn" => r#####"Eligi all_locales-funkcion"#####,
    b"output_match_fn" => {
      r#####"Generi Rust-dosierojn kun match-funkcioj por ĉiu lingvo"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Kombini ĉiujn datumojn en unu match-funkcio (kiel ĉeno)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match-funkcio kun lingvo-nomo kiel ŝlosilo"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match-funkcio kun kombina ŝlosilo (lingvo + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Simila al output_match_fn, sed nur uzas map_key kiel ŝlosilon (sen map_name)"#####
    }
    b"output_phf" => r#####"Generi PHF-mapojn por ĉiu lingvo"#####,
    b"output_phf_all_in_one" => {
      r#####"Kombini ĉiujn PHF-mapojn en unu funkcion"#####
    }
    b"output_phf_by_key" => {
      r#####"PHF-mapoj kun simplaj ĉenaj ŝlosiloj (ne TupleKey)"#####
    }
    b"output_ron" => r#####"Eligi kiel RON-formatan ĉenon"#####,
    b"suffix" => r#####"Sufikso por nova Highlight-Map"#####,
    b"syntax_name" => r#####"Nomo de sintakso"#####,
    b"theme_name" => r#####"Nomo de temo"#####,
    b"true_color" => r#####"24-bita vera koloro"#####,
    b"visibility" => r#####"Videbleco de generita kodo"#####,
    _ => "",
  }
}
