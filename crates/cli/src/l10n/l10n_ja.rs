pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"背景有効化設定"#####,
    b"base_name" => r#####"基本「Highlight-Map」名称"#####,
    b"bincode_suffix" => r#####"bincodeファイル拡張子"#####,
    b"custom_syntax_set" => r#####"カスタム構文セットファイル"#####,
    b"custom_theme_set" => r#####"カスタムテーマセットファイル"#####,
    b"display_config_dir" => r#####"glossa設定ディレクトリを表示"#####,
    b"dsl_suffix" => r#####"DSLファイル拡張子 (デフォルト ".dsl")"#####,
    b"exclude_languages" => r#####"ブラックリストモード：リスト内言語IDを除外"#####,
    b"exclude_map_names" => r#####"リスト内マップ名を除外"#####,
    b"include_languages" => {
      r#####"ホワイトリストモード：リスト内言語IDのみ初期化"#####
    }
    b"include_map_names" => r#####"リスト内マップ名のみ初期化"#####,
    b"input" => r#####"ローカライズリソースのソースディレクトリ"#####,
    b"list_all_syntaxes" => r#####"全構文名称と拡張子を表示"#####,
    b"list_all_themes" => r#####"全テーマ名称を表示"#####,
    b"mod_prefix" => r#####"modファイル接頭辞 (デフォルト "l10n_")"#####,
    b"outdir" => r#####"出力ディレクトリ"#####,
    b"output_bincode" => r#####"言語ごとに独立したbincodeファイルを生成"#####,
    b"output_bincode_all_in_one" => r#####"全言語のbincodeを単一ファイルに出力"#####,
    b"output_locales_fn" => r#####"all_locales関数を出力"#####,
    b"output_match_fn" => r#####"match式を含むRustコードファイルを言語別に生成"#####,
    b"output_match_fn_all_in_one" => {
      r#####"全データを単一match関数で出力（文字列）"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"言語名をキーとした統合match関数"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"言語名+map_key複合キーのmatch関数"#####
    }
    b"output_match_fn_by_key" => {
      r#####"map_keyのみをキーとして使用（map_name除外）"#####
    }
    b"output_phf" => r#####"言語別phfマップ関数を生成"#####,
    b"output_phf_all_in_one" => r#####"全phfマップを単一関数に統合"#####,
    b"output_phf_by_key" => {
      r#####"通常文字列キーを使用したphfマップ（TupleKey非使用）"#####
    }
    b"output_ron" => r#####"RON形式文字列として出力"#####,
    b"suffix" => r#####"新規「Highlight-Map」接尾辞"#####,
    b"syntax_name" => r#####"構文名称"#####,
    b"theme_name" => r#####"テーマ名称"#####,
    b"true_color" => r#####"24ビットTrue Color"#####,
    b"visibility" => r#####"生成コードの可視性"#####,
    _ => "",
  }
}
