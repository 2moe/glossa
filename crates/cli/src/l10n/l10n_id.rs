pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Aktifkan latar belakang"#####,
    b"base_name" => r#####"Nama dasar "Highlight-Map""#####,
    b"bincode_suffix" => r#####"Ekstensi Berkas Bincode"#####,
    b"custom_syntax_set" => r#####"Berkas set sintaks kustom"#####,
    b"custom_theme_set" => r#####"Berkas set tema kustom"#####,
    b"display_config_dir" => r#####"Tampilkan direktori konfigurasi glossa"#####,
    b"dsl_suffix" => r#####"Ekstensi Berkas DSL (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Mode Daftar Hitam: Jangan inisialisasi ID bahasa dalam daftar"#####
    }
    b"exclude_map_names" => r#####"Jangan inisialisasi nama peta dalam daftar"#####,
    b"include_languages" => {
      r#####"Mode Daftar Putih: Inisialisasi hanya ID bahasa dalam daftar"#####
    }
    b"include_map_names" => r#####"Inisialisasi hanya nama peta dalam daftar"#####,
    b"input" => r#####"Direktori sumber lokaliasi"#####,
    b"list_all_syntaxes" => r#####"Tampilkan semua nama sintaks dan ekstensi"#####,
    b"list_all_themes" => r#####"Tampilkan semua nama tema"#####,
    b"mod_prefix" => r#####"Awalan Berkas Mod (default "l10n_")"#####,
    b"outdir" => r#####"Direktori Keluaran"#####,
    b"output_bincode" => r#####"Buat berkas bincode terpisah untuk tiap bahasa"#####,
    b"output_bincode_all_in_one" => r#####"Ekspor semua bincode ke satu berkas"#####,
    b"output_locales_fn" => r#####"Ekspor fungsi all_locales"#####,
    b"output_match_fn" => {
      r#####"Buat berkas Rust terpisah dengan fungsi match per bahasa"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Gabung semua data dalam satu fungsi match (string)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Fungsi match dengan nama bahasa sebagai kunci"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Fungsi match dengan kunci gabungan (bahasa + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Hanya gunakan map_key sebagai kunci (tanpa map_name)"#####
    }
    b"output_phf" => r#####"Buat fungsi phf map terpisah per bahasa"#####,
    b"output_phf_all_in_one" => r#####"Gabung semua phf map dalam satu fungsi"#####,
    b"output_phf_without_map_name" => {
      r#####"phf map dengan kunci string biasa (bukan TupleKey)"#####
    }
    b"output_ron" => r#####"Ekspor string dalam format RON"#####,
    b"suffix" => r#####"Akhiran "Highlight-Map" baru"#####,
    b"syntax_name" => r#####"Nama Sintaks"#####,
    b"theme_name" => r#####"Nama Tema"#####,
    b"true_color" => r#####"Warna asli 24-bit"#####,
    b"visibility" => r#####"Visibilitas Kode yang Dihasilkan"#####,
    _ => "",
  }
}
