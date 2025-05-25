pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Дэвсгэр идэвхжүүлэх эсэх"#####,
    b"base_name" => r#####""Highlight-Map"-н суурь нэр"#####,
    b"bincode_suffix" => r#####"bincode файлын суффикс"#####,
    b"custom_syntax_set" => r#####"Захиалгат синтакс цуглуулгын файл"#####,
    b"custom_theme_set" => r#####"Захиалгат сэдэв цуглуулгын файл"#####,
    b"display_config_dir" => r#####"Glossa-гийн тохиргооны лавлахыг харуулах"#####,
    b"dsl_suffix" => r#####"DSL файлын суффикс (default ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Хар жагсаалтын горим: Жагсаалтад байгаа хэлний ID-г идэвхжүүлэхгүй"#####
    }
    b"exclude_map_names" => {
      r#####"Жагсаалтад байгаа газрын нэрсийг идэвхжүүлэхгүй"#####
    }
    b"include_languages" => {
      r#####"Цагаан жагсаалтын горим: Жагсаалтад байгаа хэлний ID-г л идэвхжүүлэх"#####
    }
    b"include_map_names" => {
      r#####"Жагсаалтад байгаа газрын нэрсийг л идэвхжүүлэх"#####
    }
    b"input" => r#####"Орон нутгийн нөөцийн эх лавлах"#####,
    b"list_all_syntaxes" => r#####"Бүх синтакс нэр/өргөтгөлүүдийг харуулах"#####,
    b"list_all_themes" => r#####"Бүх сэдэв нэрсийг харуулах"#####,
    b"mod_prefix" => r#####"mod файлын угтвар (default "l10n_")"#####,
    b"outdir" => r#####"Гарцны лавлах"#####,
    b"output_bincode" => {
      r#####"Өөр өөр хэлэнд зориулсан тусдаа bincode файл үүсгэх"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Бүх хэлний bincode-г нэг файлд нэгтгэх"#####
    }
    b"output_locales_fn" => r#####"all_locales функцийг гаргах"#####,
    b"output_match_fn" => {
      r#####"Хэл бүрт зориулсан Rust match функцтэй файл үүсгэх"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Бүх өгөгдлийг нэг match функцэд нэгтгэх (тэмдэгт мөр)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Хэлний нэрээр түлхүүрлэсэн match функц"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Хэл+map_key хослолоор түлхүүрлэсэн match функц"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"Зөвхөн map_key-г түлхүүр болгон ашиглах (map_name оруулахгүй)"#####
    }
    b"output_phf" => r#####"Хэл бүрт зориулсан phf map функц үүсгэх"#####,
    b"output_phf_all_in_one" => r#####"Бүх phf map-ийг нэг функцэд нэгтгэх"#####,
    b"output_phf_without_map_name" => {
      r#####"Энгийн тэмдэгт түлхүүртэй phf map (TupleKey биш)"#####
    }
    b"output_ron" => r#####"RON форматаар тэмдэгт мөр гаргах"#####,
    b"suffix" => r#####""Highlight-Map"-н шинэ суффикс"#####,
    b"syntax_name" => r#####"Синтаксын нэр"#####,
    b"theme_name" => r#####"Сэдвийн нэр"#####,
    b"true_color" => r#####"24 битийн жинхэнэ өнгө"#####,
    b"visibility" => r#####"Үүсгэсэн кодын харагдац"#####,
    _ => "",
  }
}
