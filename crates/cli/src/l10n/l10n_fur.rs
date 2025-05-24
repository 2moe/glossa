pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Ativâ supuart pal fond"#####,
    b"base_name" => r#####"Non basâl di "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sufîs file bincode"#####,
    b"custom_syntax_set" => r#####"File di sintassi personalizade"#####,
    b"custom_theme_set" => r#####"File di temis personalizâts"#####,
    b"display_config_dir" => r#####"Mostrâ diretori di configurazion glossa"#####,
    b"dsl_suffix" => r#####"Sufîs file DSL (predefinît ".dsl")"#####,
    b"exclude_languages" => r#####"Mode liste nere: Ignorâ IDs listâts"#####,
    b"exclude_map_names" => r#####"Ignorâ map_names listâts"#####,
    b"include_languages" => {
      r#####"Mode liste blancje: Inizialize dome IDs listâts"#####
    }
    b"include_map_names" => r#####"Inizialize dome map_names listâts"#####,
    b"input" => r#####"Diretori sorzint di risorsis di localizazion"#####,
    b"list_all_syntaxes" => {
      r#####"Mostrâ ducj i nons di sintassi cun estensions"#####
    }
    b"list_all_themes" => r#####"Mostrâ ducj i nons dai temis"#####,
    b"mod_prefix" => r#####"Prefîs file mod (predefinît "l10n_")"#####,
    b"outdir" => r#####"Diretori di esportazion"#####,
    b"output_bincode" => r#####"Gjenerâ files bincode separâts par ogni lenghe"#####,
    b"output_bincode_all_in_one" => {
      r#####"Espuartâ ducj i bincode intun unicon file"#####
    }
    b"output_locales_fn" => r#####"Espuartâ funzion all_locales"#####,
    b"output_match_fn" => {
      r#####"Gjenerâ files Rust separâts cun funzion di espression match"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Unificâ dâts di dutis lenghis intun funzion match (stringhe)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Funzion match cun non lenghe come clâf"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Funzion match cun clâf combinade (non lenghe + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Dopre dome map_key come clâf (cence map_name)"#####
    }
    b"output_phf" => r#####"Gjenerâ funzions phf map separadis par lenghis"#####,
    b"output_phf_all_in_one" => r#####"Unificâ ducj i phf map intun funzion"#####,
    b"output_phf_by_key" => {
      r#####"phf map cun clâf stringhe normâl (no TupleKey)"#####
    }
    b"output_ron" => r#####"Espuartâ stringhe in format RON"#####,
    b"suffix" => r#####"Sufîs pe gnove Highlight-Map"#####,
    b"syntax_name" => r#####"Non de sintassi"#####,
    b"theme_name" => r#####"Non dal teme"#####,
    b"true_color" => r#####"Color reâl 24-bit"#####,
    b"visibility" => r#####"Visibilitât dal codi gjenerât"#####,
    _ => "",
  }
}
