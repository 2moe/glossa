pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Shandisa kumashure here"#####,
    b"base_name" => r#####"Zita rekutanga re"Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sufikisi yefaira rebincode"#####,
    b"custom_syntax_set" => r#####"Faira resintakisi rakagadzirwa"#####,
    b"custom_theme_set" => r#####"Faira remusoro wakagadzirwa"#####,
    b"display_config_dir" => r#####"Ratidza dhairekitori rekugadzirisa glossa"#####,
    b"dsl_suffix" => r#####"Sufikisi yefaira reDSL (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Modhi yerondedzero nhema: ID dzemitauro dzirondedzero hadzitangi"#####
    }
    b"exclude_map_names" => r#####"Mazita emapu ariporondedzero haatangi"#####,
    b"include_languages" => {
      r#####"Modhi yerondedzero chena: ID dzemitauro dzirondedzero dzinotanga"#####
    }
    b"include_map_names" => r#####"Mazita emapu ariporondedzero anotanga"#####,
    b"input" => r#####"Dhairekitori rekutanga rezviwanikwa zvekushandura"#####,
    b"list_all_syntaxes" => r#####"Ratidza mazita ese esintakisi nemawedzero"#####,
    b"list_all_themes" => r#####"Ratidza mazita ese emisoro"#####,
    b"mod_prefix" => {
      r#####"Chirevo chekutanga chefaira remod (default "l10n_")"#####
    }
    b"outdir" => r#####"Dhairekitori rekuburitsa"#####,
    b"output_bincode" => {
      r#####"Gadzira mafaira ebincode akazvimirira emitauro yakasiyana"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Buritsa mabincode ese mufaira rimwechete"#####
    }
    b"output_locales_fn" => r#####"Buritsa basa reall_locales"#####,
    b"output_match_fn" => {
      r#####"Gadzira mafaira eRust ane mabasa ematch emitauro yakasiyana"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Batanidza data remitauro yese mubasa rimwe rematch (mutsara)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Basa rematch rine zita remutauro sekiyi"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Basa rematch rine kiyi yakabatanidzwa (mutauro + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Shandisa map_key chete sekiyi (hapana map_name)"#####
    }
    b"output_phf" => r#####"Gadzira mapu ephf emitauro yakasiyana"#####,
    b"output_phf_all_in_one" => r#####"Batanidza mapu ese ephf mubasa rimwe"#####,
    b"output_phf_without_map_name" => {
      r#####"Mapu ephf ane kiyi dzemitsara (kwete TupleKey)"#####
    }
    b"output_ron" => r#####"Buritsa mutsara weRON"#####,
    b"suffix" => r#####"Sufikisi ye"Highlight-Map" itsva"#####,
    b"syntax_name" => r#####"Zita resintakisi"#####,
    b"theme_name" => r#####"Zita remusoro"#####,
    b"true_color" => r#####"24-bit mavara echokwadi"#####,
    b"visibility" => r#####"Kuonekwa kwekodhi yakagadzirwa"#####,
    _ => "",
  }
}
