pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"배경 활성화 여부"#####,
    b"base_name" => r#####"기본 하이라이트 맵 이름"#####,
    b"bincode_suffix" => r#####"bincode 파일 확장자"#####,
    b"custom_syntax_set" => r#####"사용자 정의 문법 세트 파일"#####,
    b"custom_theme_set" => r#####"사용자 정의 테마 세트 파일"#####,
    b"display_config_dir" => r#####"Glossa 설정 디렉터리 표시"#####,
    b"dsl_suffix" => r#####"DSL 파일 확장자 (기본값 ".dsl")"#####,
    b"exclude_languages" => r#####"블랙리스트 모드: 목록 내 언어 ID 제외"#####,
    b"exclude_map_names" => r#####"목록 내 map_names 제외"#####,
    b"include_languages" => r#####"화이트리스트 모드: 목록 내 언어 ID만 초기화"#####,
    b"include_map_names" => r#####"목록 내 map_names만 초기화"#####,
    b"input" => r#####"지역화 리소스 소스 디렉터리"#####,
    b"list_all_syntaxes" => r#####"모든 문법 이름 및 확장자 표시"#####,
    b"list_all_themes" => r#####"모든 테마 이름 표시"#####,
    b"mod_prefix" => r#####"mod 파일 접두사 (기본값 "l10n_")"#####,
    b"outdir" => r#####"출력 디렉터리"#####,
    b"output_bincode" => r#####"다른 언어용 독립적인 bincode 파일 생성"#####,
    b"output_bincode_all_in_one" => {
      r#####"모든 언어의 bincode를 단일 파일로 출력"#####
    }
    b"output_locales_fn" => r#####"all_locales 함수 출력"#####,
    b"output_match_fn" => {
      r#####"match 표현식 함수 포함 Rust 코드 파일 생성 (언어별)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"모든 언어 데이터를 단일 match 함수로 출력 (문자열)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"언어명을 키로 사용한 단일 match 함수 출력"#####
    }
    b"output_match_fn_all_in_one_by_language_and_key" => {
      r#####"언어명 + map_key 조합 키 사용 match 함수"#####
    }
    b"output_match_fn_by_key" => r#####"map_key만 키로 사용 (map_name 제외)"#####,
    b"output_phf" => r#####"언어별 독립적인 phf map 함수 생성"#####,
    b"output_phf_all_in_one" => r#####"모든 phf map을 단일 함수로 통합"#####,
    b"output_phf_by_key" => {
      r#####"일반 문자열 키 사용 phf map (TupleKey 미사용)"#####
    }
    b"output_ron" => r#####"RON 형식 문자열 출력"#####,
    b"suffix" => r#####"신규 하이라이트 맵 접미사"#####,
    b"syntax_name" => r#####"문법 이름"#####,
    b"theme_name" => r#####"테마 이름"#####,
    b"true_color" => r#####"24비트 트루 컬러"#####,
    b"visibility" => r#####"생성된 코드의 가시성"#####,
    _ => "",
  }
}
