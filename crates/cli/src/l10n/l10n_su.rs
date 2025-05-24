pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktipkeun latar tukang"#####,
    b"base_name" => r#####"Ngaran dasar "Highlight-Map""#####,
    b"bincode_suffix" => r#####"sufiks file bincode"#####,
    b"custom_syntax_set" => r#####"File set sintaksis khusus"#####,
    b"custom_theme_set" => r#####"File set téma khusus"#####,
    b"display_config_dir" => r#####"Témbongkeun diréktori konfig glossa"#####,
    b"dsl_suffix" => r#####"sufiks file DSL (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mode daptar hideung: Ulah inisialisasi ID basa dina daptar"#####
    }
    b"exclude_map_names" => r#####"Ulah inisialisasi ngaran map dina daptar"#####,
    b"include_languages" => {
      r#####"Mode daptar bodas: Inisialisasi ID basa dina daptar wungkul"#####
    }
    b"include_map_names" => r#####"Inisialisasi ngaran map dina daptar wungkul"#####,
    b"input" => r#####"Diréktori sumber sumber lokal"#####,
    b"list_all_syntaxes" => {
      r#####"Témbongkeun sadaya ngaran sintaksis jeung éksténsi"#####
    }
    b"list_all_themes" => r#####"Témbongkeun sadaya ngaran téma"#####,
    b"mod_prefix" => r#####"préfiks file mod (default "l10n_")"#####,
    b"outdir" => r#####"Diréktori kaluaran"#####,
    b"output_bincode" => r#####"Nyieun file bincode misah pikeun tiap basa"#####,
    b"output_bincode_all_in_one" => {
      r#####"Nyitak sadaya bincode kana hiji file"#####
    }
    b"output_locales_fn" => r#####"Nyitak fungsi all_locales"#####,
    b"output_match_fn" => {
      r#####"Nyieun file Rust kalayan fungsi match pikeun tiap basa"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Nyatak sadaya data kana hiji fungsi match (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Fungsi match maké ngaran basa salaku konci"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Fungsi match kalayan konci gabungan (ngaran basa + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Ngan make map_key salaku konci (teu kaasup map_name)"#####
    }
    b"output_phf" => r#####"Nyieun fungsi phf map pikeun tiap basa"#####,
    b"output_phf_all_in_one" => r#####"Nyatak sadaya phf map kana hiji fungsi"#####,
    b"output_phf_by_key" => {
      r#####"phf map kalayan konci string biasa (sanés TupleKey)"#####
    }
    b"output_ron" => r#####"Nyitak string dina format RON"#####,
    b"suffix" => r#####"Sufiks anyar "Highlight-Map""#####,
    b"syntax_name" => r#####"Ngaran sintaksis"#####,
    b"theme_name" => r#####"Ngaran téma"#####,
    b"true_color" => r#####"Warna asli 24-bit"#####,
    b"visibility" => r#####"katingalina kode anu dihasilkeun"#####,
    _ => "",
  }
}
