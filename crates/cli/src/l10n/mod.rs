// use glossa_shared::lang_id;
use glossa_codegen::glossa_shared::lang_id;

pub(crate) mod locale_registry;

pub(crate) mod router;

#[cfg(feature = "l10n_af")]
mod l10n_af;

#[cfg(feature = "l10n_ak")]
mod l10n_ak;

#[cfg(feature = "l10n_am")]
mod l10n_am;

#[cfg(feature = "l10n_ar")]
mod l10n_ar;

#[cfg(feature = "l10n_as")]
mod l10n_as;

#[cfg(feature = "l10n_az")]
mod l10n_az;

#[cfg(feature = "l10n_bal")]
mod l10n_bal;

#[cfg(feature = "l10n_be")]
mod l10n_be;

#[cfg(feature = "l10n_bem")]
mod l10n_bem;

#[cfg(feature = "l10n_bg")]
mod l10n_bg;

#[cfg(feature = "l10n_bho")]
mod l10n_bho;

#[cfg(feature = "l10n_bm")]
mod l10n_bm;

#[cfg(feature = "l10n_bn")]
mod l10n_bn;

#[cfg(feature = "l10n_bs")]
mod l10n_bs;

#[cfg(feature = "l10n_ca")]
mod l10n_ca;

#[cfg(feature = "l10n_ceb")]
mod l10n_ceb;

#[cfg(feature = "l10n_cgg")]
mod l10n_cgg;

#[cfg(feature = "l10n_ckb")]
mod l10n_ckb;

#[cfg(feature = "l10n_co")]
mod l10n_co;

#[cfg(feature = "l10n_cs")]
mod l10n_cs;

#[cfg(feature = "l10n_cy")]
mod l10n_cy;

#[cfg(feature = "l10n_da")]
mod l10n_da;

#[cfg(feature = "l10n_de")]
mod l10n_de;

#[cfg(feature = "l10n_doi")]
mod l10n_doi;

#[cfg(feature = "l10n_el")]
mod l10n_el;

#[cfg(feature = "l10n_en")]
mod l10n_en;

#[cfg(feature = "l10n_en-GB")]
mod l10n_en_gb;

#[cfg(feature = "l10n_eo")]
mod l10n_eo;

#[cfg(feature = "l10n_es")]
mod l10n_es;

#[cfg(feature = "l10n_et")]
mod l10n_et;

#[cfg(feature = "l10n_eu")]
mod l10n_eu;

#[cfg(feature = "l10n_fa")]
mod l10n_fa;

#[cfg(feature = "l10n_fi")]
mod l10n_fi;

#[cfg(feature = "l10n_fil")]
mod l10n_fil;

#[cfg(feature = "l10n_fr")]
mod l10n_fr;

#[cfg(feature = "l10n_fur")]
mod l10n_fur;

#[cfg(feature = "l10n_fy")]
mod l10n_fy;

#[cfg(feature = "l10n_ga")]
mod l10n_ga;

#[cfg(feature = "l10n_gd")]
mod l10n_gd;

#[cfg(feature = "l10n_gl")]
mod l10n_gl;

#[cfg(feature = "l10n_gu")]
mod l10n_gu;

#[cfg(feature = "l10n_ha")]
mod l10n_ha;

#[cfg(feature = "l10n_he")]
mod l10n_he;

#[cfg(feature = "l10n_hi")]
mod l10n_hi;

#[cfg(feature = "l10n_hr")]
mod l10n_hr;

#[cfg(feature = "l10n_ht")]
mod l10n_ht;

#[cfg(feature = "l10n_hu")]
mod l10n_hu;

#[cfg(feature = "l10n_hy")]
mod l10n_hy;

#[cfg(feature = "l10n_id")]
mod l10n_id;

#[cfg(feature = "l10n_ig")]
mod l10n_ig;

#[cfg(feature = "l10n_is")]
mod l10n_is;

#[cfg(feature = "l10n_it")]
mod l10n_it;

#[cfg(feature = "l10n_ja")]
mod l10n_ja;

#[cfg(feature = "l10n_ja-Latn-JP")]
mod l10n_ja_latn_jp;

#[cfg(feature = "l10n_jv")]
mod l10n_jv;

#[cfg(feature = "l10n_ka")]
mod l10n_ka;

#[cfg(feature = "l10n_kk")]
mod l10n_kk;

#[cfg(feature = "l10n_km")]
mod l10n_km;

#[cfg(feature = "l10n_kn")]
mod l10n_kn;

#[cfg(feature = "l10n_ko")]
mod l10n_ko;

#[cfg(feature = "l10n_ku")]
mod l10n_ku;

#[cfg(feature = "l10n_ky")]
mod l10n_ky;

#[cfg(feature = "l10n_la")]
mod l10n_la;

#[cfg(feature = "l10n_lb")]
mod l10n_lb;

#[cfg(feature = "l10n_lg")]
mod l10n_lg;

#[cfg(feature = "l10n_ln")]
mod l10n_ln;

#[cfg(feature = "l10n_lo")]
mod l10n_lo;

#[cfg(feature = "l10n_lt")]
mod l10n_lt;

#[cfg(feature = "l10n_luo")]
mod l10n_luo;

#[cfg(feature = "l10n_lv")]
mod l10n_lv;

#[cfg(feature = "l10n_mai")]
mod l10n_mai;

#[cfg(feature = "l10n_mfe")]
mod l10n_mfe;

#[cfg(feature = "l10n_mg")]
mod l10n_mg;

#[cfg(feature = "l10n_mi")]
mod l10n_mi;

#[cfg(feature = "l10n_mk")]
mod l10n_mk;

#[cfg(feature = "l10n_ml")]
mod l10n_ml;

#[cfg(feature = "l10n_mn")]
mod l10n_mn;

#[cfg(feature = "l10n_mr")]
mod l10n_mr;

#[cfg(feature = "l10n_ms")]
mod l10n_ms;

#[cfg(feature = "l10n_mt")]
mod l10n_mt;

#[cfg(feature = "l10n_my")]
mod l10n_my;

#[cfg(feature = "l10n_ne")]
mod l10n_ne;

#[cfg(feature = "l10n_nl")]
mod l10n_nl;

#[cfg(feature = "l10n_no")]
mod l10n_no;

#[cfg(feature = "l10n_nus")]
mod l10n_nus;

#[cfg(feature = "l10n_ny")]
mod l10n_ny;

#[cfg(feature = "l10n_om")]
mod l10n_om;

#[cfg(feature = "l10n_or")]
mod l10n_or;

#[cfg(feature = "l10n_pa")]
mod l10n_pa;

#[cfg(feature = "l10n_pl")]
mod l10n_pl;

#[cfg(feature = "l10n_ps")]
mod l10n_ps;

#[cfg(feature = "l10n_pt")]
mod l10n_pt;

#[cfg(feature = "l10n_qu")]
mod l10n_qu;

#[cfg(feature = "l10n_ro")]
mod l10n_ro;

#[cfg(feature = "l10n_ru")]
mod l10n_ru;

#[cfg(feature = "l10n_rw")]
mod l10n_rw;

#[cfg(feature = "l10n_sah")]
mod l10n_sah;

#[cfg(feature = "l10n_sat")]
mod l10n_sat;

#[cfg(feature = "l10n_sd")]
mod l10n_sd;

#[cfg(feature = "l10n_si")]
mod l10n_si;

#[cfg(feature = "l10n_sk")]
mod l10n_sk;

#[cfg(feature = "l10n_sl")]
mod l10n_sl;

#[cfg(feature = "l10n_sm")]
mod l10n_sm;

#[cfg(feature = "l10n_sn")]
mod l10n_sn;

#[cfg(feature = "l10n_so")]
mod l10n_so;

#[cfg(feature = "l10n_sq")]
mod l10n_sq;

#[cfg(feature = "l10n_sr")]
mod l10n_sr;

#[cfg(feature = "l10n_st")]
mod l10n_st;

#[cfg(feature = "l10n_su")]
mod l10n_su;

#[cfg(feature = "l10n_sv")]
mod l10n_sv;

#[cfg(feature = "l10n_sw")]
mod l10n_sw;

#[cfg(feature = "l10n_ta")]
mod l10n_ta;

#[cfg(feature = "l10n_te")]
mod l10n_te;

#[cfg(feature = "l10n_tg")]
mod l10n_tg;

#[cfg(feature = "l10n_th")]
mod l10n_th;

#[cfg(feature = "l10n_ti")]
mod l10n_ti;

#[cfg(feature = "l10n_tk")]
mod l10n_tk;

#[cfg(feature = "l10n_tr")]
mod l10n_tr;

#[cfg(feature = "l10n_tt")]
mod l10n_tt;

#[cfg(feature = "l10n_ug")]
mod l10n_ug;

#[cfg(feature = "l10n_uk")]
mod l10n_uk;

#[cfg(feature = "l10n_ur")]
mod l10n_ur;

#[cfg(feature = "l10n_uz")]
mod l10n_uz;

#[cfg(feature = "l10n_vi")]
mod l10n_vi;

#[cfg(feature = "l10n_xh")]
mod l10n_xh;

#[cfg(feature = "l10n_yi")]
mod l10n_yi;

#[cfg(feature = "l10n_yo")]
mod l10n_yo;

#[cfg(feature = "l10n_zh")]
mod l10n_zh;

#[cfg(feature = "l10n_zh-Hant")]
mod l10n_zh_hant;

#[cfg(feature = "l10n_zh-Latn-CN")]
mod l10n_zh_latn_cn;

#[cfg(feature = "l10n_zu")]
mod l10n_zu;
