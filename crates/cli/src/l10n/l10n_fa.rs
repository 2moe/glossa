pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"فعال‌سازی پس‌زمینه"#####,
    b"base_name" => r#####"نام پایه "Highlight-Map""#####,
    b"bincode_suffix" => r#####"پسوند فایل bincode"#####,
    b"custom_syntax_set" => r#####"فایل مجموعه سینتکس سفارشی"#####,
    b"custom_theme_set" => r#####"فایل مجموعه تم سفارشی"#####,
    b"display_config_dir" => r#####"نمایش پوشه پیکربندی glossa"#####,
    b"dsl_suffix" => r#####"پسوند فایل DSL (پیش‌فرض ".dsl")"#####,
    b"exclude_languages" => {
      r#####"حالت لیست سیاه: شناسه‌های زبانی موجود در لیست مقداردهی نشوند"#####
    }
    b"exclude_map_names" => r#####"نام‌های map موجود در لیست مقداردهی نشوند"#####,
    b"include_languages" => {
      r#####"حالت لیست سفید: فقط شناسه‌های زبانی موجود در لیست مقداردهی شوند"#####
    }
    b"include_map_names" => r#####"فقط نام‌های map موجود در لیست مقداردهی شوند"#####,
    b"input" => r#####"پوشه منبع منابع محلی‌سازی"#####,
    b"list_all_syntaxes" => r#####"نمایش تمام نام‌های سینتکس و پسوندها"#####,
    b"list_all_themes" => r#####"نمایش تمام نام‌های تم"#####,
    b"mod_prefix" => r#####"پیشوند فایل mod (پیش‌فرض "l10n_")"#####,
    b"outdir" => r#####"پوشه خروجی"#####,
    b"output_bincode" => {
      r#####"ایجاد فایل‌های bincode جداگانه برای زبان‌های مختلف"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"خروجی تمام bincodeها در یک فایل واحد"#####
    }
    b"output_locales_fn" => r#####"خروجی تابع all_locales"#####,
    b"output_match_fn" => {
      r#####"ایجاد فایل‌های کد Rust با توابع match برای زبان‌ها"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"خروجی تمام داده‌ها در یک تابع match (رشته)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"تابع match با نام زبان به عنوان کلید"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"تابع match با کلید ترکیبی (زبان + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"فقط از map_key به عنوان کلید استفاده شود (بدون map_name)"#####
    }
    b"output_phf" => r#####"ایجاد توابع phf map جداگانه برای زبان‌ها"#####,
    b"output_phf_all_in_one" => r#####"ادغام تمام phf mapها در یک تابع"#####,
    b"output_phf_by_key" => {
      r#####"phf map با کلیدهای رشته‌ای ساده (نه TupleKey)"#####
    }
    b"output_ron" => r#####"خروجی رشته با فرمت RON"#####,
    b"suffix" => r#####"پسوند جدید "Highlight-Map""#####,
    b"syntax_name" => r#####"نام سینتکس"#####,
    b"theme_name" => r#####"نام تم"#####,
    b"true_color" => r#####"رنگ واقعی 24 بیتی"#####,
    b"visibility" => r#####"قابلیت مشاهده کد تولیدشده"#####,
    _ => "",
  }
}
