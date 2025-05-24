pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Kozesa ekifo eky'emabega"#####,
    b"base_name" => r#####"Erinnya ery'okusingira "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Endabika ya fayiro ya bincode"#####,
    b"custom_syntax_set" => r#####"Fayilo y'endabika z'okukola ez'omwene"#####,
    b"custom_theme_set" => r#####"Fayilo y'emitwe gy'omwene"#####,
    b"display_config_dir" => r#####"Laga folda ya config ya Glossa"#####,
    b"dsl_suffix" => r#####"Endabika ya DSL fayiro (ekisooka ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Black List: Totandika ID z'ennimi eziri mu lutimbe"#####
    }
    b"exclude_map_names" => r#####"Totandika amannya g'amapu agali mu lutimbe"#####,
    b"include_languages" => {
      r#####"White List: Tandika ID z'ennimi eziri mu lutimbe"#####
    }
    b"include_map_names" => r#####"Tandika amannya g'amapu agali mu lutimbe"#####,
    b"input" => r#####"Folda ey'okutandikirako ebikozesebwa mu lokalize"#####,
    b"list_all_syntaxes" => r#####"Laga ennimi zonna n'endabika zaazo"#####,
    b"list_all_themes" => r#####"Laga emitwe gyonna"#####,
    b"mod_prefix" => r#####"Endabika ya mod fayiro (ekisooka "l10n_")"#####,
    b"outdir" => r#####"Folda ey'okufuula"#####,
    b"output_bincode" => {
      r#####"Kola fayiro za bincode ez'enjawulo mu nnimi ez'enjawulo"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Teka bincode z'ennimi zonna mu fayiro emu"#####
    }
    b"output_locales_fn" => r#####"Tuma all_locales function"#####,
    b"output_match_fn" => {
      r#####"Kola fayiro za Rust code ez'enjawulo nga zikozesa match functions"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Teka data yonna mu function emu ya match (mukutu)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match function erina erinnya ly'olulimi nga ggulu"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match function nga ggulu liri erinnya ly'olulimi ne map_key"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Kozesa map_key okuba ggulu (tewali map_name)"#####
    }
    b"output_phf" => r#####"Kola phf map functions ez'enjawulo mu nnimi"#####,
    b"output_phf_all_in_one" => r#####"Gatta phf maps zonna mu function emu"#####,
    b"output_phf_by_key" => {
      r#####"Phf map erina ggulu ez'omukutu ogw'okutendekera (si TupleKey)"#####
    }
    b"output_ron" => r#####"Tuma mukutu mu nkola ya RON"#####,
    b"suffix" => r#####"Endabika empya ya "Highlight-Map""#####,
    b"syntax_name" => r#####"Erinnya ly'endabika y'okukola"#####,
    b"theme_name" => r#####"Erinnya ly'omutwe"#####,
    b"true_color" => r#####"Langi ey'okutuufu eya 24-bit"#####,
    b"visibility" => r#####"Okulabika kwa koodi ey'okuzimbwa"#####,
    _ => "",
  }
}
