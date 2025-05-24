pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Galluogi cefndir"#####,
    b"base_name" => r#####"Enw Craidd "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Ôl-ddodiad Ffeil Bincode"#####,
    b"custom_syntax_set" => r#####"Ffeil Set Cystrawen Wedi'i Addasu"#####,
    b"custom_theme_set" => r#####"Ffeil Set Themâu Wedi'i Addasu"#####,
    b"display_config_dir" => r#####"Dangos cyfeiriadur cyfluniad glossa"#####,
    b"dsl_suffix" => r#####"Ôl-ddodiad Ffeil DSL (diofyn ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Modd Rhestr Ddu: Peidio ymgychwyn ID iaith mewn rhestr"#####
    }
    b"exclude_map_names" => r#####"Peidio ymgychwyn enwau map mewn rhestr"#####,
    b"include_languages" => {
      r#####"Modd Rhestr Gwyn: Ymgychwyn ID iaith yn unig os yw rhestr yn wag"#####
    }
    b"include_map_names" => {
      r#####"Ymgychwyn enwau map yn unig os yw rhestr yn wag"#####
    }
    b"input" => r#####"Cyfeiriadur Ffynhonnell Adnoddau Lleoleiddio"#####,
    b"list_all_syntaxes" => r#####"Dangos pob enw cystrawen ac estyniadau"#####,
    b"list_all_themes" => r#####"Dangos pob enw themâu"#####,
    b"mod_prefix" => r#####"Rhagddodiad Ffeil Mod (diofyn "l10n_")"#####,
    b"outdir" => r#####"Cyfeiriadur Allbwn"#####,
    b"output_bincode" => {
      r#####"Cynhyrchu ffeiliau bincode annibynnol ar gyfer ieithoedd gwahanol"#####
    }
    b"output_bincode_all_in_one" => r#####"Allbynnu pob bincode i un ffeil"#####,
    b"output_locales_fn" => r#####"Allbynnu swyddogaeth all_locales"#####,
    b"output_match_fn" => {
      r#####"Cynhyrchu ffeiliau Rust gyda swyddogaethau match ar gyfer ieithoedd"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Cyfuno pob data mewn swyddogaeth match unigol (llinyn)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Swyddogaeth match gydag enw iaith fel allwedd"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Swyddogaeth match gydag allwedd cyfansawdd (iaith + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Defnyddio map_key yn unig fel allwedd (heb map_name)"#####
    }
    b"output_phf" => r#####"Cynhyrchu mapiau phf annibynnol ar gyfer ieithoedd"#####,
    b"output_phf_all_in_one" => r#####"Cyfuno pob map phf mewn un swyddogaeth"#####,
    b"output_phf_by_key" => {
      r#####"Mapiau phf gydag allweddi llinyn cyffredin (Nid TupleKey)"#####
    }
    b"output_ron" => r#####"Allbynnu llinyn ar ffurf RON"#####,
    b"suffix" => r#####"Ôl-ddodiad "Highlight-Map" newydd"#####,
    b"syntax_name" => r#####"Enw Cystrawen"#####,
    b"theme_name" => r#####"Enw Themâu"#####,
    b"true_color" => r#####"Lliw Cywir 24-bit"#####,
    b"visibility" => r#####"Gwelededd y Cod a Gynhyrchwyd"#####,
    _ => "",
  }
}
