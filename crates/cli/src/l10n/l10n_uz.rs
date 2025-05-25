pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Fonni faollashtirish"#####,
    b"base_name" => r#####""Highlight-Map"ning asosiy nomi"#####,
    b"bincode_suffix" => r#####"bincode fayl kengaytmasi"#####,
    b"custom_syntax_set" => r#####"Maxsus sintaksis toʻplami fayli"#####,
    b"custom_theme_set" => r#####"Maxsus mavzular toʻplami fayli"#####,
    b"display_config_dir" => {
      r#####"Glossa konfiguratsiya katalogini koʻrsatish"#####
    }
    b"dsl_suffix" => r#####"DSL fayl kengaytmasi (standart ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Qora roʻyxat rejimi: Roʻyxatdagi til ID'larini ishga tushirmaslik"#####
    }
    b"exclude_map_names" => {
      r#####"Roʻyxatdagi xarita nomlarini ishga tushirmaslik"#####
    }
    b"include_languages" => {
      r#####"Oq roʻyxat rejimi: Roʻyxatdagi til ID'larini faqat ishga tushirish"#####
    }
    b"include_map_names" => {
      r#####"Roʻyxatdagi xarita nomlarini faqat ishga tushirish"#####
    }
    b"input" => r#####"Lokalizatsiya resurslarining manba katalogi"#####,
    b"list_all_syntaxes" => {
      r#####"Barcha sintaksis nomlari va kengaytmalarini koʻrsatish"#####
    }
    b"list_all_themes" => r#####"Barcha mavzu nomlarini koʻrsatish"#####,
    b"mod_prefix" => r#####"mod fayl prefiksi (standart "l10n_")"#####,
    b"outdir" => r#####"Chiqish katalogi"#####,
    b"output_bincode" => {
      r#####"Turli tillar uchun alohida bincode fayllarini yaratish"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Barcha tillarning bincode'larini bitta faylga eksport qilish"#####
    }
    b"output_locales_fn" => r#####"all_locales funktsiyasini eksport qilish"#####,
    b"output_match_fn" => {
      r#####"Rust kodi fayllarini yaratish (match ifodalari bilan)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Barcha maʼlumotlarni bitta match funktsiyasiga eksport qilish (satr)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Til nomi kalit boʻlgan match funktsiyasi"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Birlashtirilgan kalit (til+map_key) bilan match funktsiyasi"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Faqat map_key'ni kalit sifatida ishlatish (map_name kiritilmaydi)"#####
    }
    b"output_phf" => r#####"Har bir til uchun alohida phf xarita funktsiyalari"#####,
    b"output_phf_all_in_one" => {
      r#####"Barcha phf xaritalarni bitta funktsiyada birlashtirish"#####
    }
    b"output_phf_without_map_name" => {
      r#####"Oddiy satr kalitli phf xarita (TupleKey emas)"#####
    }
    b"output_ron" => r#####"RON formatida satrni eksport qilish"#####,
    b"suffix" => r#####""Highlight-Map"ning yangi kengaytmasi"#####,
    b"syntax_name" => r#####"Sintaksis nomi"#####,
    b"theme_name" => r#####"Mavzu nomi"#####,
    b"true_color" => r#####"24-bitli haqiqiy rang"#####,
    b"visibility" => r#####"Yaratilgan kodning koʻrinishi"#####,
    _ => "",
  }
}
