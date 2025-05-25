pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"dayakan latar belakang"#####,
    b"base_name" => r#####"nama asas Highlight-Map"#####,
    b"bincode_suffix" => r#####"akhiran fail bincode"#####,
    b"custom_syntax_set" => r#####"fail set sintaks tersuai"#####,
    b"custom_theme_set" => r#####"fail set tema tersuai"#####,
    b"display_config_dir" => r#####"paparkan direktori konfigurasi glossa"#####,
    b"dsl_suffix" => r#####"akhiran fail DSL (lalai: ".dsl")"#####,
    b"exclude_languages" => {
      r#####"mod senarai hitam: ID bahasa tersenarai tidak diinisialisasi"#####
    }
    b"exclude_map_names" => r#####"nama peta tersenarai tidak diinisialisasi"#####,
    b"include_languages" => {
      r#####"mod senarai putih: hanya ID bahasa tersenarai diinisialisasi"#####
    }
    b"include_map_names" => r#####"hanya nama peta tersenarai diinisialisasi"#####,
    b"input" => r#####"direktori sumber lokal"#####,
    b"list_all_syntaxes" => {
      r#####"paparkan semua nama sintaks dengan sambungan"#####
    }
    b"list_all_themes" => r#####"paparkan semua nama tema"#####,
    b"mod_prefix" => r#####"awalan fail mod (lalai: "l10n_")"#####,
    b"outdir" => r#####"direktori output"#####,
    b"output_bincode" => {
      r#####"jana fail bincode berasingan untuk setiap bahasa"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"eksport semua bincode bahasa ke satu fail"#####
    }
    b"output_locales_fn" => r#####"eksport fungsi all_locales"#####,
    b"output_match_fn" => {
      r#####"jana fail kod Rust dengan fungsi match expression"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"gabung semua data dalam satu fungsi match (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"fungsi match dengan nama bahasa sebagai kunci"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"fungsi match dengan kunci kombo (bahasa + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"hanya gunakan map_key sebagai kunci (tiada map_name)"#####
    }
    b"output_phf" => r#####"jana fungsi phf map berasingan per bahasa"#####,
    b"output_phf_all_in_one" => r#####"gabung semua phf map dalam satu fungsi"#####,
    b"output_phf_without_map_name" => {
      r#####"phf map dengan kunci string biasa (bukan TupleKey)"#####
    }
    b"output_ron" => r#####"eksport string berformat RON"#####,
    b"suffix" => r#####"akhiran untuk Highlight-Map baru"#####,
    b"syntax_name" => r#####"nama sintaks"#####,
    b"theme_name" => r#####"nama tema"#####,
    b"true_color" => r#####"24-bit warna sebenar"#####,
    b"visibility" => r#####"keterlihatan kod terhasil"#####,
    _ => "",
  }
}
