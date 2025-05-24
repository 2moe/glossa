pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"Arxa planı aktivləşdir"#####,
    b"base_name" => r#####""Highlight-Map" əsas adı"#####,
    b"bincode_suffix" => r#####"bincode fayl sonluğu"#####,
    b"custom_syntax_set" => r#####"Xüsusi sintaksis kolleksiyası faylı"#####,
    b"custom_theme_set" => r#####"Xüsusi mövzu kolleksiyası faylı"#####,
    b"display_config_dir" => r#####"Glossa konfiqurasiya qovluğunu göstər"#####,
    b"dsl_suffix" => r#####"DSL fayl sonluğu (standart ".dsl")"#####,
    b"exclude_languages" => {
      r#####"Qara siyahı rejimi: Siyahıdakı dil ID'lərini işə salma"#####
    }
    b"exclude_map_names" => r#####"Siyahıdakı xəritə adlarını işə salma"#####,
    b"include_languages" => {
      r#####"Ağ siyahı rejimi: Yalnız siyahıdakı dil ID'lərini işə sal"#####
    }
    b"include_map_names" => r#####"Yalnız siyahıdakı xəritə adlarını işə sal"#####,
    b"input" => r#####"Lokalizasiya resurslarının mənbə qovluğu"#####,
    b"list_all_syntaxes" => {
      r#####"Bütün sintaksis adlarını və genişlənmələri göstər"#####
    }
    b"list_all_themes" => r#####"Bütün mövzu adlarını göstər"#####,
    b"mod_prefix" => r#####"mod fayl prefiksi (standart "l10n_")"#####,
    b"outdir" => r#####"Çıxış qovluğu"#####,
    b"output_bincode" => {
      r#####"Müxtəlif dillər üçün ayrı-ayrı bincode faylları yarat"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Bütün dillərin bincode'larını bir fayla çıxar"#####
    }
    b"output_locales_fn" => r#####"all_locales funksiyasını çıxar"#####,
    b"output_match_fn" => {
      r#####"Rust kod faylları yarat (match ifadəli funksiyalar)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Bütün məlumatları bir match funksiyasına çıxar (sətir)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Dil adını açar kimi istifadə edən match funksiyası"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"Birləşmiş açar (dil + map_key) ilə match funksiyası"#####
    }
    b"output_match_fn_by_key" => {
      r#####"Yalnız map_key'ni açar kimi istifadə et (map_name olmadan)"#####
    }
    b"output_phf" => {
      r#####"Müxtəlif dillər üçün ayrı phf xəritə funksiyaları yarat"#####
    }
    b"output_phf_all_in_one" => {
      r#####"Bütün phf xəritələri bir funksiyada birləşdir"#####
    }
    b"output_phf_by_key" => {
      r#####"Adi sətir açarlı phf xəritə (TupleKey deyil)"#####
    }
    b"output_ron" => r#####"RON formatında sətir çıxar"#####,
    b"suffix" => r#####""Highlight-Map" yeni sonluğu"#####,
    b"syntax_name" => r#####"Sintaksis adı"#####,
    b"theme_name" => r#####"Mövzu adı"#####,
    b"true_color" => r#####"24-bit real rəng"#####,
    b"visibility" => r#####"Yaradılan kodun görünürlüyü"#####,
    _ => "",
  }
}
