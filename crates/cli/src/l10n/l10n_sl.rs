pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Omogoči ozadje"#####,
    b"base_name" => r#####"Osnovno ime "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Pripona bincode datoteke"#####,
    b"custom_syntax_set" => r#####"Datoteka po meri za sintakso"#####,
    b"custom_theme_set" => r#####"Datoteka po meri za teme"#####,
    b"display_config_dir" => r#####"Prikaži konfiguracijski imenik za Glossa"#####,
    b"dsl_suffix" => r#####"Pripona DSL datoteke (privzeto ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Črni seznam: Ne inicializiraj ID-jev jezikov iz seznama"#####
    }
    b"exclude_map_names" => r#####"Ne inicializiraj imen map iz seznama"#####,
    b"include_languages" => {
      r#####"Beli seznam: Inicializiraj samo ID-je jezikov iz seznama"#####
    }
    b"include_map_names" => r#####"Inicializiraj samo imena map iz seznama"#####,
    b"input" => r#####"Izvorni imenik lokalizacijskih virov"#####,
    b"list_all_syntaxes" => r#####"Prikaži vsa imena sintaks in razširitve"#####,
    b"list_all_themes" => r#####"Prikaži vsa imena tem"#####,
    b"mod_prefix" => r#####"Predpona mod datoteke (privzeto "l10n_")"#####,
    b"outdir" => r#####"Izhodni imenik"#####,
    b"output_bincode" => {
      r#####"Ustvari ločene bincode datoteke za različne jezike"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Izvozi vse bincode jezikov v eno datoteko"#####
    }
    b"output_locales_fn" => r#####"Izvozi all_locales funkcijo"#####,
    b"output_match_fn" => {
      r#####"Ustvari Rust datoteke z match izrazi za jezike"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Združi vse podatke v eno match funkcijo (niz)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match funkcija z imenom jezika kot ključ"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match funkcija s kombiniranim ključem (jezik + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Uporabi samo map_key kot ključ (brez map_name)"#####
    }
    b"output_phf" => r#####"Ustvari phf map funkcije za jezike"#####,
    b"output_phf_all_in_one" => r#####"Združi vse phf mape v eno funkcijo"#####,
    b"output_phf_by_key" => {
      r#####"phf map z navadnimi nizi kot ključi (ne TupleKey)"#####
    }
    b"output_ron" => r#####"Izvozi niz v formatu RON"#####,
    b"suffix" => r#####"Nova pripona za "Highlight-Map""#####,
    b"syntax_name" => r#####"Ime sintakse"#####,
    b"theme_name" => r#####"Ime teme"#####,
    b"true_color" => r#####"24-bitne resnične barve"#####,
    b"visibility" => r#####"Vidnost generirane kode"#####,
    _ => "",
  }
}
