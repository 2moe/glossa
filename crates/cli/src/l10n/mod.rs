// use glossa_shared::lang_id;
use glossa_codegen::glossa_shared::lang_id;

pub(crate) mod locale_registry;

pub(crate) mod router;

#[cfg(feature = "l10n-af")]
mod l10n_af;

#[cfg(feature = "l10n-ak")]
mod l10n_ak;

#[cfg(feature = "l10n-am")]
mod l10n_am;

#[cfg(feature = "l10n-ar")]
mod l10n_ar;

#[cfg(feature = "l10n-as")]
mod l10n_as;

#[cfg(feature = "l10n-az")]
mod l10n_az;

#[cfg(feature = "l10n-bal")]
mod l10n_bal;

#[cfg(feature = "l10n-be")]
mod l10n_be;

#[cfg(feature = "l10n-bem")]
mod l10n_bem;

#[cfg(feature = "l10n-bg")]
mod l10n_bg;

#[cfg(feature = "l10n-bho")]
mod l10n_bho;

#[cfg(feature = "l10n-bm")]
mod l10n_bm;

#[cfg(feature = "l10n-bn")]
mod l10n_bn;

#[cfg(feature = "l10n-bs")]
mod l10n_bs;

#[cfg(feature = "l10n-ca")]
mod l10n_ca;

#[cfg(feature = "l10n-ceb")]
mod l10n_ceb;

#[cfg(feature = "l10n-cgg")]
mod l10n_cgg;

#[cfg(feature = "l10n-ckb")]
mod l10n_ckb;

#[cfg(feature = "l10n-co")]
mod l10n_co;

#[cfg(feature = "l10n-cs")]
mod l10n_cs;

#[cfg(feature = "l10n-cy")]
mod l10n_cy;

#[cfg(feature = "l10n-da")]
mod l10n_da;

#[cfg(feature = "l10n-de")]
mod l10n_de;

#[cfg(feature = "l10n-doi")]
mod l10n_doi;

#[cfg(feature = "l10n-el")]
mod l10n_el;

#[cfg(feature = "l10n-en")]
mod l10n_en;

#[cfg(feature = "l10n-en-GB")]
mod l10n_en_gb;

#[cfg(feature = "l10n-eo")]
mod l10n_eo;

#[cfg(feature = "l10n-es")]
mod l10n_es;

#[cfg(feature = "l10n-et")]
mod l10n_et;

#[cfg(feature = "l10n-eu")]
mod l10n_eu;

#[cfg(feature = "l10n-fa")]
mod l10n_fa;

#[cfg(feature = "l10n-fi")]
mod l10n_fi;

#[cfg(feature = "l10n-fil")]
mod l10n_fil;

#[cfg(feature = "l10n-fr")]
mod l10n_fr;

#[cfg(feature = "l10n-fur")]
mod l10n_fur;

#[cfg(feature = "l10n-fy")]
mod l10n_fy;

#[cfg(feature = "l10n-ga")]
mod l10n_ga;

#[cfg(feature = "l10n-gd")]
mod l10n_gd;

#[cfg(feature = "l10n-gl")]
mod l10n_gl;

#[cfg(feature = "l10n-gu")]
mod l10n_gu;

#[cfg(feature = "l10n-ha")]
mod l10n_ha;

#[cfg(feature = "l10n-he")]
mod l10n_he;

#[cfg(feature = "l10n-hi")]
mod l10n_hi;

#[cfg(feature = "l10n-hr")]
mod l10n_hr;

#[cfg(feature = "l10n-ht")]
mod l10n_ht;

#[cfg(feature = "l10n-hu")]
mod l10n_hu;

#[cfg(feature = "l10n-hy")]
mod l10n_hy;

#[cfg(feature = "l10n-id")]
mod l10n_id;

#[cfg(feature = "l10n-ig")]
mod l10n_ig;

#[cfg(feature = "l10n-is")]
mod l10n_is;

#[cfg(feature = "l10n-it")]
mod l10n_it;

#[cfg(feature = "l10n-ja")]
mod l10n_ja;

#[cfg(feature = "l10n-ja-Latn-JP")]
mod l10n_ja_latn_jp;

#[cfg(feature = "l10n-jv")]
mod l10n_jv;

#[cfg(feature = "l10n-ka")]
mod l10n_ka;

#[cfg(feature = "l10n-kk")]
mod l10n_kk;

#[cfg(feature = "l10n-km")]
mod l10n_km;

#[cfg(feature = "l10n-kn")]
mod l10n_kn;

#[cfg(feature = "l10n-ko")]
mod l10n_ko;

#[cfg(feature = "l10n-ku")]
mod l10n_ku;

#[cfg(feature = "l10n-ky")]
mod l10n_ky;

#[cfg(feature = "l10n-la")]
mod l10n_la;

#[cfg(feature = "l10n-lb")]
mod l10n_lb;

#[cfg(feature = "l10n-lg")]
mod l10n_lg;

#[cfg(feature = "l10n-ln")]
mod l10n_ln;

#[cfg(feature = "l10n-lo")]
mod l10n_lo;

#[cfg(feature = "l10n-lt")]
mod l10n_lt;

#[cfg(feature = "l10n-luo")]
mod l10n_luo;

#[cfg(feature = "l10n-lv")]
mod l10n_lv;

#[cfg(feature = "l10n-mai")]
mod l10n_mai;

#[cfg(feature = "l10n-mfe")]
mod l10n_mfe;

#[cfg(feature = "l10n-mg")]
mod l10n_mg;

#[cfg(feature = "l10n-mi")]
mod l10n_mi;

#[cfg(feature = "l10n-mk")]
mod l10n_mk;

#[cfg(feature = "l10n-ml")]
mod l10n_ml;

#[cfg(feature = "l10n-mn")]
mod l10n_mn;

#[cfg(feature = "l10n-mr")]
mod l10n_mr;

#[cfg(feature = "l10n-ms")]
mod l10n_ms;

#[cfg(feature = "l10n-mt")]
mod l10n_mt;

#[cfg(feature = "l10n-my")]
mod l10n_my;

#[cfg(feature = "l10n-ne")]
mod l10n_ne;

#[cfg(feature = "l10n-nl")]
mod l10n_nl;

#[cfg(feature = "l10n-no")]
mod l10n_no;

#[cfg(feature = "l10n-nus")]
mod l10n_nus;

#[cfg(feature = "l10n-ny")]
mod l10n_ny;

#[cfg(feature = "l10n-om")]
mod l10n_om;

#[cfg(feature = "l10n-or")]
mod l10n_or;

#[cfg(feature = "l10n-pa")]
mod l10n_pa;

#[cfg(feature = "l10n-pl")]
mod l10n_pl;

#[cfg(feature = "l10n-ps")]
mod l10n_ps;

#[cfg(feature = "l10n-pt")]
mod l10n_pt;

#[cfg(feature = "l10n-qu")]
mod l10n_qu;

#[cfg(feature = "l10n-ro")]
mod l10n_ro;

#[cfg(feature = "l10n-ru")]
mod l10n_ru;

#[cfg(feature = "l10n-rw")]
mod l10n_rw;

#[cfg(feature = "l10n-sah")]
mod l10n_sah;

#[cfg(feature = "l10n-sat")]
mod l10n_sat;

#[cfg(feature = "l10n-sd")]
mod l10n_sd;

#[cfg(feature = "l10n-si")]
mod l10n_si;

#[cfg(feature = "l10n-sk")]
mod l10n_sk;

#[cfg(feature = "l10n-sl")]
mod l10n_sl;

#[cfg(feature = "l10n-sm")]
mod l10n_sm;

#[cfg(feature = "l10n-sn")]
mod l10n_sn;

#[cfg(feature = "l10n-so")]
mod l10n_so;

#[cfg(feature = "l10n-sq")]
mod l10n_sq;

#[cfg(feature = "l10n-sr")]
mod l10n_sr;

#[cfg(feature = "l10n-st")]
mod l10n_st;

#[cfg(feature = "l10n-su")]
mod l10n_su;

#[cfg(feature = "l10n-sv")]
mod l10n_sv;

#[cfg(feature = "l10n-sw")]
mod l10n_sw;

#[cfg(feature = "l10n-ta")]
mod l10n_ta;

#[cfg(feature = "l10n-te")]
mod l10n_te;

#[cfg(feature = "l10n-tg")]
mod l10n_tg;

#[cfg(feature = "l10n-th")]
mod l10n_th;

#[cfg(feature = "l10n-ti")]
mod l10n_ti;

#[cfg(feature = "l10n-tk")]
mod l10n_tk;

#[cfg(feature = "l10n-tr")]
mod l10n_tr;

#[cfg(feature = "l10n-tt")]
mod l10n_tt;

#[cfg(feature = "l10n-ug")]
mod l10n_ug;

#[cfg(feature = "l10n-uk")]
mod l10n_uk;

#[cfg(feature = "l10n-ur")]
mod l10n_ur;

#[cfg(feature = "l10n-uz")]
mod l10n_uz;

#[cfg(feature = "l10n-vi")]
mod l10n_vi;

#[cfg(feature = "l10n-xh")]
mod l10n_xh;

#[cfg(feature = "l10n-yi")]
mod l10n_yi;

#[cfg(feature = "l10n-yo")]
mod l10n_yo;

#[cfg(feature = "l10n-zh")]
mod l10n_zh;

#[cfg(feature = "l10n-zh-Hant")]
mod l10n_zh_hant;

#[cfg(feature = "l10n-zh-Latn-CN")]
mod l10n_zh_latn_cn;

#[cfg(feature = "l10n-zu")]
mod l10n_zu;
