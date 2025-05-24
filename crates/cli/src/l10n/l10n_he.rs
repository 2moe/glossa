pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"הפעל רקע"#####,
    b"base_name" => r#####"שם בסיס ל"Highlight-Map""#####,
    b"bincode_suffix" => r#####"סיומת קובץ bincode"#####,
    b"custom_syntax_set" => r#####"קובץ ערכת תחביר מותאמת"#####,
    b"custom_theme_set" => r#####"קובץ ערכת עיצוב מותאמת"#####,
    b"display_config_dir" => r#####"הצג תיקיית תצורת glossa"#####,
    b"dsl_suffix" => r#####"סיומת קובץ DSL (ברירת מחדל ".dsl")"#####,
    b"exclude_languages" => r#####"מצב רשימה שחורה: אל תאתחל מזהה שפות מהרשימה"#####,
    b"exclude_map_names" => r#####"אל תאתחל שמות מפות מהרשימה"#####,
    b"include_languages" => r#####"מצב רשימה לבנה: אתחל רק מזהה שפות מהרשימה"#####,
    b"include_map_names" => r#####"אתחל רק שמות מפות מהרשימה"#####,
    b"input" => r#####"תיקיית מקור למשאבים מקומיים"#####,
    b"list_all_syntaxes" => r#####"הצג כל שמות תחביר וסיומות"#####,
    b"list_all_themes" => r#####"הצג כל שמות ערכות עיצוב"#####,
    b"mod_prefix" => r#####"קידומת קובץ mod (ברירת מחדל "l10n_")"#####,
    b"outdir" => r#####"תיקיית פלט"#####,
    b"output_bincode" => r#####"צור קבצי bincode נפרדים לשפות שונות"#####,
    b"output_bincode_all_in_one" => r#####"ייצא את כל ה-bincode לקובץ אחד"#####,
    b"output_locales_fn" => r#####"ייצא את פונקציית all_locales"#####,
    b"output_match_fn" => r#####"צור קבצי Rust עם פונקציות match לכל שפה"#####,
    b"output_match_fn_all_in_one" => {
      r#####"ייצא את כל הנתונים לפונקציית match אחת (מחרוזת)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"פונקציית match עם שם שפה כמפתח"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"פונקציית match עם מפתח משולב (שפה + map_key)"#####
    }
    b"output_match_fn_by_key" => {
      r#####"השתמש רק ב-map_key כמפתח (ללא map_name)"#####
    }
    b"output_phf" => r#####"צור פונקציות phf map נפרדות לשפות"#####,
    b"output_phf_all_in_one" => r#####"מזג את כל ה-phf maps לפונקציה אחת"#####,
    b"output_phf_by_key" => {
      r#####"phf map עם מפתחות מחרוזת רגילים (לא TupleKey)"#####
    }
    b"output_ron" => r#####"ייצא מחרוזת בפורמט RON"#####,
    b"suffix" => r#####"סיומת חדשה ל"Highlight-Map""#####,
    b"syntax_name" => r#####"שם תחביר"#####,
    b"theme_name" => r#####"שם ערכת עיצוב"#####,
    b"true_color" => r#####"צבע אמיתי 24-ביט"#####,
    b"visibility" => r#####"נראות קוד שנוצר"#####,
    _ => "",
  }
}
