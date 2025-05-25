pub(crate) const fn map(key: &[u8]) -> &'static str {
  match key {
    b"background" => r#####"ShiFou QiYong BeiJing"#####,
    b"base_name" => r#####"JiChu " Gao Liang Map" De MingCheng"#####,
    b"bincode_suffix" => r#####"Bincode WenJian HouZhui"#####,
    b"custom_syntax_set" => r#####"ZiDingYi YuFa Ji WenJian"#####,
    b"custom_theme_set" => r#####"ZiDingYi ZhuTi Ji WenJian"#####,
    b"display_config_dir" => r#####"XianShi Glossa De PeiZhi MuLu"#####,
    b"dsl_suffix" => r#####"DSL  WenJian HouZhui ( MoRen  ".dsl")"#####,
    b"exclude_languages" => {
      r#####"HeiMingDan MoShi. WeiYu HeiMingDan Zhong De YuYan  id  BuHui Bei ChuShiHua"#####
    }
    b"exclude_map_names" => {
      r#####"WeiYu LieBiao Zhong De  map_names  BuHui Bei ChuShiHua"#####
    }
    b"include_languages" => {
      r#####"BaiMingDan MoShi. Dang Qi BuWei Kong Shi, ZhiYou WeiYu LieBiao Zhong De YuYan  id  Cai Hui Bei ChuShiHua"#####
    }
    b"include_map_names" => {
      r#####"Dang Qi BuWei Kong Shi, ZhiYou WeiYu LieBiao Zhong De  map_names  Cai Hui Bei ChuShiHua"#####
    }
    b"input" => r#####"BenDiHua ZiYuan De YuanMuLu"#####,
    b"list_all_syntaxes" => {
      r#####"XianShi SuoYou YuFa MingCheng JiQi KuoZhanMing"#####
    }
    b"list_all_themes" => r#####"XianShi SuoYou ZhuTi MingCheng"#####,
    b"mod_prefix" => r#####"Mod  WenJian QianZhui  ( MoRen  "l10n_")"#####,
    b"outdir" => r#####"ShuChu De MuLu"#####,
    b"output_bincode" => {
      r#####"Wei BuTong De YuYan ShengCheng DuLi De  bincode  WenJian"#####
    }
    b"output_bincode_all_in_one" => {
      r#####"Jiang SuoYou YuYan De  bincode  ShuChu Dao TongYiGe WenJian"#####
    }
    b"output_locales_fn" => r#####"ShuChu  all_locales  HanShu"#####,
    b"output_match_fn" => {
      r#####"Wei BuTong De YuYan ShengCheng DuLi De  rust  DaiMa De WenJian  ( BaoHan Match BiaoDaShi De HanShu )"#####
    }
    b"output_match_fn_all_in_one" => {
      r#####"Jiang SuoYou YuYan De ShuJu Dou ShuChu Wei YiGe Match HanShu （ ZiFuChuan ）"#####
    }
    b"output_match_fn_all_in_one_by_language" => {
      r#####"Jiang SuoYou YuYan De ShuJu ShuChu Wei TongYiGe Match  HanShu （ ZiFuChuan ）, key  Wei YuYan Ming"#####
    }
    b"output_match_fn_all_in_one_without_map_name" => {
      r#####"Jiang SuoYou YuYan De ShuJu ShuChu Wei TongYiGe  match  HanShu （ ZiFuChuan ）, key  Wei YuYan Ming He  map_key"#####
    }
    b"output_match_fn_without_map_name" => {
      r#####"LeiSi Yu  output_match_fn, Dan ShengCheng De HanShu Zhi Yi  map_key  ZuoWei  key,  Bu BaoHan  map_name"#####
    }
    b"output_phf" => {
      r#####"Wei BuTong YuYan ShengCheng DuLi De  phf map  HanShu"#####
    }
    b"output_phf_all_in_one" => {
      r#####"Jiang SuoYou YuYan De  phf map  ShuChu Dao TongYiGe HanShu"#####
    }
    b"output_phf_without_map_name" => {
      r#####"LeiSi Yu  output_phf,  Dan ShengCheng De HanShu De Key  Wei PuTong ZiFuChuan, Er BuShi  TupleKey"#####
    }
    b"output_ron" => r#####"ShuChu Wei  ron  GeShi De ZiFuChuan"#####,
    b"suffix" => r#####"Xin ShengCheng De " Gao Liang Map" De HouZhui"#####,
    b"syntax_name" => r#####"YuFa MingCheng"#####,
    b"theme_name" => r#####"ZhuTi MingCheng"#####,
    b"true_color" => r#####"24 Wei ZhenCaiSe"#####,
    b"visibility" => r#####"ShengCheng De DaiMa De KeJian Xing"#####,
    _ => "",
  }
}
