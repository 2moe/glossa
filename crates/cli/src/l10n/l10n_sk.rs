pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktivovať podporu pozadia"#####,
    b"base_name" => r#####"Základný názov "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Prípona súboru bincode"#####,
    b"custom_syntax_set" => r#####"Vlastný súbor syntaxových nastavení"#####,
    b"custom_theme_set" => r#####"Vlastný súbor tematických nastavení"#####,
    b"display_config_dir" => r#####"Zobraziť konfiguračný priečinok glossa"#####,
    b"dsl_suffix" => r#####"Prípona DSL súboru (predvolené ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Režim čiernej zoznamu: Neinicializovať uvedené jazykové ID"#####
    }
    b"exclude_map_names" => r#####"Neinicializovať uvedené mapové názvy"#####,
    b"include_languages" => {
      r#####"Režim bielej zoznamu: Inicializovať iba uvedené jazykové ID"#####
    }
    b"include_map_names" => r#####"Inicializovať iba uvedené mapové názvy"#####,
    b"input" => r#####"Zdrojový priečinok lokalizačných zdrojov"#####,
    b"list_all_syntaxes" => {
      r#####"Zobraziť všetky názvy syntaxí s rozšíreniami"#####
    }
    b"list_all_themes" => r#####"Zobraziť všetky názvy tém"#####,
    b"mod_prefix" => r#####"Predpona mod súboru (predvolené "l10n_")"#####,
    b"outdir" => r#####"Výstupný priečinok"#####,
    b"output_bincode" => {
      r#####"Generovať samostatné bincode súbory pre rôzne jazyky"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Exportovať všetky jazykové bincode do jedného súboru"#####
    }
    b"output_locales_fn" => r#####"Exportovať funkciu all_locales"#####,
    b"output_match_fn" => {
      r#####"Generovať Rust súbory s match funkciami pre každý jazyk"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Konsolidovať všetky dáta do jednej match funkcie (reťazec)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Match funkcia s názvom jazyka ako kľúčom"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Match funkcia s kombinovaným kľúčom (jazyk + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Použiť iba map_key ako kľúč (bez map_name)"#####
    }
    b"output_phf" => r#####"Generovať PHF mapy pre jednotlivé jazyky"#####,
    b"output_phf_all_in_one" => {
      r#####"Konsolidovať všetky PHF mapy do jednej funkcie"#####
    }
    b"output_phf_by_key" => {
      r#####"PHF mapy s obyčajnými reťazcovými kľúčmi (nie TupleKey)"#####
    }
    b"output_ron" => r#####"Exportovať ako reťazec vo formáte RON"#####,
    b"suffix" => r#####"Prípona pre nové "Highlight-Map""#####,
    b"syntax_name" => r#####"Názov syntaxe"#####,
    b"theme_name" => r#####"Názov témy"#####,
    b"true_color" => r#####"24-bitová verná farba"#####,
    b"visibility" => r#####"Viditeľnosť vygenerovaného kódu"#####,
    _ => "",
  }
}
