pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Virkja bakgrunn"#####,
    b"base_name" => r#####"Grunnheiti "Highlight-Map""#####,
    b"bincode_suffix" => r#####"bincode skráarending"#####,
    b"custom_syntax_set" => r#####"Sérsniðin málfræðiskrá"#####,
    b"custom_theme_set" => r#####"Sérsniðin þemaskrá"#####,
    b"display_config_dir" => r#####"Sýna stillingaskráarsafn Glossa"#####,
    b"dsl_suffix" => r#####"DSL skráarending (sjálfgefið ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Svartur listi: Ekki frumstilla tungumáls ID í lista"#####
    }
    b"exclude_map_names" => r#####"Ekki frumstilla kortanöfn í lista"#####,
    b"include_languages" => {
      r#####"Hvítur listi: Frumstilla aðeins tungumáls ID í lista"#####
    }
    b"include_map_names" => r#####"Frumstilla aðeins kortanöfn í lista"#####,
    b"input" => r#####"Upprunaskráarsafn staðfærðra auðlinda"#####,
    b"list_all_syntaxes" => r#####"Sýna öll málfræðiheiti og endingar"#####,
    b"list_all_themes" => r#####"Sýna öll þemaheiti"#####,
    b"mod_prefix" => r#####"mod forskeyti skráar (sjálfgefið "l10n_")"#####,
    b"outdir" => r#####"Úttaksskráarsafn"#####,
    b"output_bincode" => {
      r#####"Búa til aðskildar bincode skrár fyrir mismunandi tungumál"#####
    }
    b"output_bincode_all_in_one" => r#####"Úttak allra bincode í eina skrá"#####,
    b"output_locales_fn" => r#####"Úttak all_locales virkni"#####,
    b"output_match_fn" => {
      r#####"Búa til Rust kóðaskrár með match virkni fyrir tungumál"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Úttak allra gagna í eina match virkni (strengur)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match virkni með tungumálsheiti sem lykil"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Match virkni með samsettum lykli (tungumál+map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Nota aðeins map_key sem lykil (án map_name)"#####
    }
    b"output_phf" => r#####"Búa til aðskildar phf kortavirknir fyrir tungumál"#####,
    b"output_phf_all_in_one" => r#####"Sameina öll phf kort í eina virkni"#####,
    b"output_phf_without_map_name" => {
      r#####"phf kort með venjulegum strenglykli (ekki TupleKey)"#####
    }
    b"output_ron" => r#####"Úttak streng í RON-sniði"#####,
    b"suffix" => r#####"Ný ending fyrir "Highlight-Map""#####,
    b"syntax_name" => r#####"Málfræðiheiti"#####,
    b"theme_name" => r#####"Þemaheiti"#####,
    b"true_color" => r#####"24-bita sann litur"#####,
    b"visibility" => r#####"Sýnileiki myndaðs kóða"#####,
    _ => "",
  }
}
