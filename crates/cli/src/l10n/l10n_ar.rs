pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"تفعيل الخلفية"#####,
    b"base_name" => r#####"الاسم الأساسي لـ"الخريطة المضيئة""#####,
    b"bincode_suffix" => r#####"لاحقة ملف bincode"#####,
    b"custom_syntax_set" => r#####"ملف مجموعة تراكيب مخصصة"#####,
    b"custom_theme_set" => r#####"ملف مجموعة سمات مخصصة"#####,
    b"display_config_dir" => r#####"عرض دليل تهيئة glossa"#####,
    b"dsl_suffix" => r#####"لاحقة ملف DSL (الافتراضي ".dsl")"#####,
    b"exclude_languages" => {
      r#####"وضع القائمة السوداء: عدم تهيئة معرفات اللغات الموجودة في القائمة"#####
    }
    b"exclude_map_names" => r#####"عدم تهيئة أسماء الخرائط الموجودة في القائمة"#####,
    b"include_languages" => {
      r#####"وضع القائمة البيضاء: تهيئة معرفات اللغات الموجودة في القائمة فقط"#####
    }
    b"include_map_names" => r#####"تهيئة أسماء الخرائط الموجودة في القائمة فقط"#####,
    b"input" => r#####"دليل مصادر المواقع المحلية"#####,
    b"list_all_syntaxes" => r#####"عرض جميع أسماء التراكيب وامتداداتها"#####,
    b"list_all_themes" => r#####"عرض جميع أسماء السمات"#####,
    b"mod_prefix" => r#####"بادئة ملف mod (الافتراضي "l10n_")"#####,
    b"outdir" => r#####"دليل الإخراج"#####,
    b"output_bincode" => r#####"إنشاء ملفات bincode منفصلة للغات المختلفة"#####,
    b"output_bincode_all_in_one" => {
      r#####"تصدير جميع ملفات bincode في ملف واحد"#####
    }
    b"output_locales_fn" => r#####"تصدير دالة all_locales"#####,
    b"output_match_fn" => r#####"إنشاء ملفات كود Rust مع دوال match للغات"#####,
    b"output_match_fn_all_in_one" => {
      r#####"تصدير جميع البيانات في دالة match واحدة (سلسلة نصية)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"دالة match تستخدم اسم اللغة كمفتاح"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"دالة match بمفتاح مركب (اسم اللغة + map_key)"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"استخدام map_key كمفتاح فقط (بدون map_name)"#####
    }
    b"output_phf" => r#####"إنشاء دوال phf map منفصلة للغات"#####,
    b"output_phf_all_in_one" => r#####"دمج جميع خرائط phf في دالة واحدة"#####,
    b"output_phf_without_map_name" => r#####"phf map بمفاتيح نصية عادية (ليست TupleKey)"#####,
    b"output_ron" => r#####"تصدير سلسلة نصية بصيغة RON"#####,
    b"suffix" => r#####"لاحقة جديدة لـ"الخريطة المضيئة""#####,
    b"syntax_name" => r#####"اسم التركيب النحوي"#####,
    b"theme_name" => r#####"اسم السمة"#####,
    b"true_color" => r#####"لون حقيقي 24-بت"#####,
    b"visibility" => r#####"رؤية الكود المُنشأ"#####,
    _ => "",
  }
}
