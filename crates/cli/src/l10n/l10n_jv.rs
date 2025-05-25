pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktifake latar mburi"#####,
    b"base_name" => r#####"Jeneng dhasar "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Sufiks file bincode"#####,
    b"custom_syntax_set" => r#####"File set sintaks kustom"#####,
    b"custom_theme_set" => r#####"File set tema kustom"#####,
    b"display_config_dir" => r#####"Tampilake direktori konfig glossa"#####,
    b"dsl_suffix" => r#####"Sufiks file DSL (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mode daftar ireng: Aja inisialisasi ID basa sing ana ing daftar"#####
    }
    b"exclude_map_names" => {
      r#####"Aja inisialisasi jeneng peta sing ana ing daftar"#####
    }
    b"include_languages" => {
      r#####"Mode daftar putih: Inisialisasi ID basa sing ana ing daftar"#####
    }
    b"include_map_names" => {
      r#####"Inisialisasi jeneng peta sing ana ing daftar"#####
    }
    b"input" => r#####"Direktori sumber lokalisasi"#####,
    b"list_all_syntaxes" => r#####"Tampilake kabeh jeneng sintaks lan ekstensi"#####,
    b"list_all_themes" => r#####"Tampilake kabeh jeneng tema"#####,
    b"mod_prefix" => r#####"Awalan file mod (default "l10n_")"#####,
    b"outdir" => r#####"Direktori Output"#####,
    b"output_bincode" => r#####"Nggawe file bincode kapisah kanggo basa beda"#####,
    b"output_bincode_all_in_one" => {
      r#####"Ekspor kabeh bincode menyang siji file"#####
    }
    b"output_locales_fn" => r#####"Ekspor fungsi all_locales"#####,
    b"output_match_fn" => {
      r#####"Nggawe file Rust kanthi fungsi match kanggo saben basa"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Ekspor kabeh data menyang siji fungsi match (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Fungsi match nganggo jeneng basa minangka kunci"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Fungsi match nganggo kunci gabung (basa + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Mung nggunakake map_key minangka kunci (ora kalebu map_name)"#####
    }
    b"output_phf" => r#####"Nggawe fungsi phf map kapisah kanggo saben basa"#####,
    b"output_phf_all_in_one" => {
      r#####"Gabung kabeh phf map menyang siji fungsi"#####
    }
    b"output_phf_without_map_name" => {
      r#####"phf map nganggo kunci string biasa (dudu TupleKey)"#####
    }
    b"output_ron" => r#####"Ekspor string nganggo format RON"#####,
    b"suffix" => r#####"Sufiks anyar "Highlight-Map""#####,
    b"syntax_name" => r#####"Jeneng sintaks"#####,
    b"theme_name" => r#####"Jeneng tema"#####,
    b"true_color" => r#####"Warna asli 24-bit"#####,
    b"visibility" => r#####"Katonan kode sing digawe"#####,
    _ => "",
  }
}
