pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"পটভূমি সক্ৰিয় কৰিব নে"#####,
    b"base_name" => r#####""হাইলাইট-মেপ" ৰ মূল নাম"#####,
    b"bincode_suffix" => r#####"bincode ফাইলৰ প্ৰত্যয়"#####,
    b"custom_syntax_set" => r#####"কাষ্টম চিন্টেক্স ছেট ফাইল"#####,
    b"custom_theme_set" => r#####"কাষ্টম থীম ছেট ফাইল"#####,
    b"display_config_dir" => r#####"গ্ল'ছাৰ কনফিগাৰেচন ডাইৰেক্টৰী দেখুৱাওক"#####,
    b"dsl_suffix" => r#####"DSL ফাইলৰ প্ৰত্যয় (ডিফল্ট ".dsl")"#####,
    b"exclude_languages" => {
      r#####"ক'লা তালিকা ম'ড: তালিকাৰ ভাষা ID বিলাক ইনিচিয়েলাইজ নকৰিব"#####
    }
    b"exclude_map_names" => r#####"তালিকাৰ মেপ নামবিলাক ইনিচিয়েলাইজ নকৰিব"#####,
    b"include_languages" => {
      r#####"বগা তালিকা ম'ড: তালিকাৰ ভাষা ID বিলাকহে ইনিচিয়েলাইজ কৰক"#####
    }
    b"include_map_names" => r#####"তালিকাৰ মেপ নামবিলাকহে ইনিচিয়েলাইজ কৰক"#####,
    b"input" => r#####"স্থানীয়কৰণ সম্পদৰ উৎস ডাইৰেক্টৰী"#####,
    b"list_all_syntaxes" => r#####"সকলো চিন্টেক্স নাম আৰু এক্সটেনচ্যন দেখুৱাওক"#####,
    b"list_all_themes" => r#####"সকলো থীম নাম দেখুৱাওক"#####,
    b"mod_prefix" => r#####"mod ফাইল প্ৰিফিক্স (ডিফল্ট "l10n_")"#####,
    b"outdir" => r#####"আউটপুট ডাইৰেক্টৰী"#####,
    b"output_bincode" => r#####"বিভিন্ন ভাষাৰ বাবে পৃথক bincode ফাইল সৃষ্টি কৰক"#####,
    b"output_bincode_all_in_one" => {
      r#####"সকলো ভাষাৰ bincode এটা ফাইলত এক্সপ'ৰ্ট কৰক"#####
    }
    b"output_locales_fn" => r#####"all_locales ফাংচন এক্সপ'ৰ্ট কৰক"#####,
    b"output_match_fn" => {
      r#####"ৰাষ্ট ক'ড ফাইল সৃষ্টি কৰক (match এক্সপ্ৰেছ্যন থকা ফাংচনসহ)"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"সকলো তথ্য এটা match ফাংচনত এক্সপ'ৰ্ট কৰক (ষ্ট্ৰিং)"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"ভাষাৰ নামৰ সৈতে match ফাংচন"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"সংযুক্ত কী (ভাষা+map_key) ৰে match ফাংচন"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"কেৱল map_key কে কী হিচাপে ব্যৱহাৰ কৰক (map_name নোহোৱা)"#####
    }
    b"output_phf" => r#####"বিভিন্ন ভাষাৰ বাবে পৃথক phf মেপ ফাংচন সৃষ্টি কৰক"#####,
    b"output_phf_all_in_one" => r#####"সকলো phf মেপ এটা ফাংচনত সংযুক্ত কৰক"#####,
    b"output_phf_without_map_name" => r#####"সাধাৰণ ষ্ট্ৰিং কীৰে phf মেপ (TupleKey নহয়)"#####,
    b"output_ron" => r#####"RON ফৰ্মাটত ষ্ট্ৰিং এক্সপ'ৰ্ট কৰক"#####,
    b"suffix" => r#####""হাইলাইট-মেপ" ৰ নতুন প্ৰত্যয়"#####,
    b"syntax_name" => r#####"চিন্টেক্স নাম"#####,
    b"theme_name" => r#####"থীম নাম"#####,
    b"true_color" => r#####"24-বিটৰ প্ৰকৃত ৰং"#####,
    b"visibility" => r#####"জেনেৰেট কৰা ক'ডৰ দৃশ্যমানতা"#####,
    _ => "",
  }
}
