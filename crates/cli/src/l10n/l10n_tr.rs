pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Arka plan etkinleştirilsin mi"#####,
    b"base_name" => r#####""Highlight-Map" temel adı"#####,
    b"bincode_suffix" => r#####"bincode dosya uzantısı"#####,
    b"custom_syntax_set" => r#####"Özel sözdizimi seti dosyası"#####,
    b"custom_theme_set" => r#####"Özel tema seti dosyası"#####,
    b"display_config_dir" => r#####"Glossa yapılandırma dizinini göster"#####,
    b"dsl_suffix" => r#####"DSL dosya uzantısı (varsayılan ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Kara liste modu: Listedeki dil ID'leri başlatılmaz"#####
    }
    b"exclude_map_names" => r#####"Listedeki harita adları başlatılmaz"#####,
    b"include_languages" => {
      r#####"Beyaz liste modu: Yalnızca listedeki dil ID'leri başlatılır"#####
    }
    b"include_map_names" => r#####"Yalnızca listedeki harita adları başlatılır"#####,
    b"input" => r#####"Yerelleştirme kaynaklarının kaynak dizini"#####,
    b"list_all_syntaxes" => r#####"Tüm sözdizimi adlarını ve uzantıları göster"#####,
    b"list_all_themes" => r#####"Tüm tema adlarını göster"#####,
    b"mod_prefix" => r#####"mod dosya öneki (varsayılan "l10n_")"#####,
    b"outdir" => r#####"Çıktı dizini"#####,
    b"output_bincode" => {
      r#####"Farklı diller için bağımsız bincode dosyaları oluştur"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Tüm dillerin bincode'larını tek dosyaya aktar"#####
    }
    b"output_locales_fn" => r#####"all_locales fonksiyonunu dışa aktar"#####,
    b"output_match_fn" => {
      r#####"Farklı diller için Rust kod dosyaları oluştur (match ifadeli fonksiyonlar içeren)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Tüm verileri tek match fonksiyonuna aktar (dize)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Dil adını anahtar olarak kullanan match fonksiyonu"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Birleşik anahtarlı match fonksiyonu (dil adı + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Yalnızca map_key'i anahtar olarak kullan (map_name içermez)"#####
    }
    b"output_phf" => {
      r#####"Farklı diller için bağımsız phf harita fonksiyonları oluştur"#####
    }
    b"output_phf_all_in_one" => {
      r#####"Tüm phf haritaları tek fonksiyonda birleştir"#####
    }
    b"output_phf_by_key" => {
      r#####"Sıradan dize anahtarlı phf harita (TupleKey değil)"#####
    }
    b"output_ron" => r#####"RON formatında dize dışa aktar"#####,
    b"suffix" => r#####""Highlight-Map" yeni uzantısı"#####,
    b"syntax_name" => r#####"Sözdizimi adı"#####,
    b"theme_name" => r#####"Tema adı"#####,
    b"true_color" => r#####"24-bit gerçek renk"#####,
    b"visibility" => r#####"Oluşturulan kodun görünürlüğü"#####,
    _ => "",
  }
}
