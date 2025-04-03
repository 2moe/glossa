use anyhow::Result as AnyResult;
use collect_with::CollectVector;
use compact_str::ToCompactString;
use glossa::try_init_chain_from_slice;

pub(crate) fn init_logger(trace: bool) {
  let level = {
    use log::LevelFilter::*;
    if trace { Trace } else { Debug }
  };

  env_logger::builder()
    .filter_level(level)
    .init()
}

#[test]
fn test_init_zh_hk_chain() -> AnyResult<()> {
  // init_logger(true);

  let chain = try_init_chain_from_slice(
    "zh-HK",
    &[
      "de",
      "ru",
      "zh-Latn", // "zh-Latn-CN-pinyin"
      "zh-Hans-HK",
      "zh-Latn-HK",
      "ar",
      "en",
      "es",
      "pt",
      "zh",
      "zh-Hans",
      "zh-SG",
      "zh-Hant",
      "zh-Hant-TW",
      "zh-Hant-HK",
      "zh-MO",
    ],
  )?;
  // <(id, score)>:
  // [("zh-Hant-HK", 46), ("zh-MO", 45), ("zh-Hant", 42), ("zh-Hant-TW", 42),
  // ("zh-Hans-HK", 32), ("zh", 31), ("zh-Hans", 31), ("zh-SG", 27),
  // ("zh-Latn-HK", 26), ("zh-Latn", 22)]

  let v = chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect_vec_with(|_| 10);
  // dbg!(v);

  assert_eq!(
    v,
    [
      "zh-Hant-HK",
      "zh-MO",
      "zh-Hant",
      "zh-Hant-TW",
      "zh-Hans-HK",
      "zh",
      "zh-Hans",
      "zh-SG",
      "zh-Latn-HK",
      "zh-Latn",
    ]
  );

  Ok(())
}

const fn en_gb_list() -> [&'static str; 95] {
  [
    "en-150", "en-AE", "en-AG", "en-AI", // "en-AS",
    "en-AT", "en-AU", "en-BB", "en-BE", "en-BI", "en-BM", "en-BS", "en-BW", "en-BZ",
    // "en-CA",
    "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK",
    "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG",
    "en-GH", "en-GI", "en-GM", // "en-GU",
    "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM",
    "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG",
    // "en-MH",
    "en-MO", // "en-MP",
    "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG",
    "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", // "en-PH",
    "en-PK", "en-PN", // "en-PR",
    "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI",
    "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV",
    "en-TZ", "en-UG", // "en-UM",
    "en-VC", "en-VG", // "en-VI",
    "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW",
  ]
}
const fn en_us_list() -> [&'static str; 12] {
  [
    "en-AS", "en-CA", "en-GU", "en-MH", "en-MP", "en-PH", "en-PR", "en-UM", "en-US",
    "en-VI", "en-001", "en",
  ]
}
fn language_list<'a>() -> Vec<&'a str> {
  ["de", "ru", "zh-Latn", "ar", "es", "pt", "zh"]
    .into_iter()
    .chain(en_gb_list())
    .chain(en_us_list())
    .collect_vec_with(|_| 128)
}

#[test]
#[ignore]
fn test_init_en_ca_chain() -> AnyResult<()> {
  init_logger(true);

  let chain = try_init_chain_from_slice("en-CA", &language_list())?;
  // <(id, score)>:
  // [("en-CA", 50), ("en", 45), ("en-US", 45), ("en-PR", 44), ("en-VI", 44),
  // ("en-AS", 43), ("en-GU", 43), ("en-MH", 43), ("en-MP", 43), ("en-PH", 43),
  // ("en-UM", 43), ("en-001", 43), ("en-BM", 42), ("en-AG", 41), ("en-AI", 41),
  // ("en-BB", 41), ("en-BS", 41), ("en-BZ", 41), ("en-DM", 41), ("en-GD", 41),
  // ("en-JM", 41), ("en-KN", 41), ("en-KY", 41), ("en-LC", 41), ("en-MS", 41),
  // ("en-SX", 41), ("en-TC", 41), ("en-TT", 41), ("en-VC", 41), ("en-VG", 41),
  // ("en-GB", 40), ...

  let v = chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect_vec_with(|_| 10);

  dbg!(v);
  Ok(())
}

#[test]
#[ignore]
fn test_init_en_au_chain() -> AnyResult<()> {
  init_logger(true);

  let chain = try_init_chain_from_slice("en-AU", &language_list())?;
  // <(id, score)>:
  //  [("en-AU", 50), ("en-GB", 43), ("en-CC", 42), ("en-CX", 42), ("en-NF", 42),
  // ("en-NZ", 42), ("en-UM", 41), ("en-CK", 41), ("en-DG", 41), ("en-FJ", 41),
  // ("en-FM", 41), ("en-KI", 41), ("en-NR", 41), ("en-NU", 41), ("en-PG", 41),
  // ("en-PN", 41), ("en-PW", 41), ("en-SB", 41), ("en-TK", 41), ("en-TO", 41),
  // ("en-TV", 41), ("en-VU", 41), ("en-WS", 41), ("en-AS", 41), ("en-GU", 41),
  // ("en-MH", 41), ("en-MP", 41), ("en-US", 22), ...

  let v = chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect_vec_with(|_| 10);

  dbg!(v);
  Ok(())
}

#[test]
#[ignore]
fn test_init_en_de_chain() -> AnyResult<()> {
  init_logger(true);

  let chain = try_init_chain_from_slice("en-DE", &language_list())?;
  // <(id, score)>:
  //  [("en-DE", 50), ("en-GB", 44), ("en-AT", 42), ("en-BE", 42), ("en-CH", 42),
  // ("en-NL", 42), ("en-150", 41), ("en-DK", 41), ("en-FI", 41), ("en-GG", 41),
  // ("en-GI", 41), ("en-IE", 41), ("en-IM", 41), ("en-JE", 41), ("en-MT", 41),
  // ("en-SE", 41), ("en-SI", 41), ("en-US", 40), ...

  let v = chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect_vec_with(|_| 10);

  dbg!(v);
  Ok(())
}

#[test]
#[ignore]
fn test_init_zh_cn_chain() -> AnyResult<()> {
  init_logger(true);

  let chain = try_init_chain_from_slice(
    "zh-CN",
    &[
      "de",
      "ru",
      "zh-Latn",
      "zh-Latn-HK",
      "ar",
      "en",
      "es",
      "pt",
      // "zh-CN",
      "zh-Hans-CN",
      "zh",
      "zh-Hans-SG",
      "zh-SG",
      "zh-Hans",
      "zh-Hant",
      "zh-Hant-TW",
      "zh-Hant-HK",
      "zh-Hant-CN",
      "zh-MO",
    ],
  )?;

  // <(id, score)>:
  // [("zh-Hans-CN", 40), ("zh", 40), ("zh-Hans", 40), ("zh-Hans-SG", 35),
  // ("zh-SG", 35), ("zh-Hant-CN", 32), ("zh-Hant-HK", 31), ("zh-MO", 31),
  // ("zh-Hant", 28), ("zh-Hant-TW", 28), ("zh-Latn", 26), ("zh-Latn-HK", 22)]

  let v = chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect_vec_with(|_| 10);
  dbg!(&v);

  assert_eq!(
    v,
    [
      "zh-Hans-CN",
      "zh",
      "zh-Hans",
      "zh-Hans-SG",
      "zh-SG",
      "zh-Hant-CN",
      "zh-Hant-HK",
      "zh-MO",
      "zh-Hant",
      "zh-Hant-TW",
      "zh-Latn",
      "zh-Latn-HK",
    ]
  );

  Ok(())
}

#[test]
#[ignore]
fn test_init_zh_hant_cn_chain() -> AnyResult<()> {
  init_logger(true);

  let chain = try_init_chain_from_slice(
    "zh-Hant-CN",
    &[
      "de",
      "ru",
      "lzh",
      "lzh-Hans",
      "zh-Latn",
      "zh-Latn-HK",
      "zh-Hant-CN",
      "ar",
      "en",
      "es",
      "pt",
      "zh-CN",
      "zh-Hans-CN",
      "zh",
      "zh-Hans-SG",
      "zh-SG",
      "zh-Hans",
      "zh-Hant",
      "zh-Hant-TW",
      "zh-Hant-HK",
      "zh-MO",
    ],
  )?;

  // <(id, score)>:
  // [("zh-Hant-CN", 50), ("zh-Hant", 36), ("zh-Hant-TW", 36), ("zh-Hant-HK", 36),
  // ("zh-MO", 36), ("zh-CN", 35), ("zh-Hans-CN", 35), ("zh", 35), ("zh-Hans",
  // 35), ("lzh", 29), ("zh-Hans-SG", 27), ("zh-SG", 27), ("zh-Latn", 26),
  // ("zh-Latn-HK", 22), ("lzh-Hans", 6)]

  let v = chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect_vec_with(|_| 10);
  dbg!(&v);

  Ok(())
}

#[test]
#[ignore]
fn test_init_zh_pinyin_chain() -> AnyResult<()> {
  init_logger(true);

  let chain = try_init_chain_from_slice(
    "zh-pinyin",
    &[
      "de",
      "ru",
      "zh-Hant-CN",
      "zh-Latn-HK",
      "zh-Latn",
      "ar",
      "en",
      "es",
      "pt",
      "zh-CN",
      "zh-Hans-CN",
      "zh",
      "zh-Hans-SG",
      "zh-SG",
      "zh-Hans",
      "zh-Hant",
      "zh-Hant-TW",
      "zh-Hant-HK",
      "zh-MO",
    ],
  )?;

  // <(id, score)>:
  // [("zh-Latn", 40), ("zh-Latn-HK", 36), ("zh-CN", 35), ("zh-Hans-CN", 35),
  // ("zh", 35), ("zh-Hans", 35), ("zh-Hans-SG", 27), ("zh-SG", 27),
  // ("zh-Hant-CN", 26), ("zh-Hant", 22), ("zh-Hant-TW", 22), ("zh-Hant-HK", 22),
  // ("zh-MO", 22)]

  let v = chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect_vec_with(|_| 10);
  dbg!(&v);

  Ok(())
}

#[test]
// #[ignore]
fn test_init_zh_tw_chain() -> AnyResult<()> {
  // init_logger(true);

  let chain = try_init_chain_from_slice(
    "zh-TW",
    &[
      "de",
      "ru",
      "zh-Hant-CN",
      "zh-Latn-HK",
      "zh-Latn",
      "ar",
      "en",
      "es",
      "pt",
      "zh-CN",
      "zh-Hans-CN",
      "zh",
      "zh-Hans-SG",
      "zh-SG",
      "zh-Hans",
      "zh-Hant",
      "zh-Hant-TW",
      "zh-Hant-HK",
      "zh-MO",
      "zh-TW",
    ],
  )?;

  // <(id, score)>:
  // [("zh-TW", 50), ("zh-Hant", 49), ("zh-Hant-TW", 49), ("zh-Hant-CN", 42),
  // ("zh-Hant-HK", 42), ("zh-MO", 42), ("zh-CN", 31), ("zh-Hans-CN", 31), ("zh",
  // 31), ("zh-Hans", 31), ("zh-Hans-SG", 27), ("zh-SG", 27), ("zh-Latn-HK", 22),
  // ("zh-Latn", 22)]

  let v = chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect_vec_with(|_| 13);
  // dbg!(&v);
  assert_eq!(
    v,
    [
      "zh-TW",
      "zh-Hant",
      "zh-Hant-TW",
      "zh-Hant-CN",
      "zh-Hant-HK",
      "zh-MO",
      "zh-CN",
      "zh-Hans-CN",
      "zh",
      "zh-Hans",
      "zh-Hans-SG",
      "zh-SG",
      "zh-Latn-HK",
      "zh-Latn",
    ]
  );

  Ok(())
}

const fn es_list() -> [&'static str; 28] {
  [
    "es", "es-419", "es-AR", "es-BO", "es-BR", "es-BZ", "es-CL", "es-CO", "es-CR",
    "es-CU", "es-DO", "es-EA", "es-EC", "es-GQ", "es-GT", "es-HN", "es-IC", "es-MX",
    "es-NI", "es-PA", "es-PE", "es-PH", "es-PR", "es-PY", "es-SV", "es-US", "es-UY",
    "es-VE",
  ]
}

#[ignore]
#[test]
fn test_init_es_ar_chain() -> AnyResult<()> {
  init_logger(true);

  let chain = try_init_chain_from_slice("es-Latn-AR", &es_list())?;
  // <(id, score)>:
  // ("es-AR", 40), ("es-BO", 36), ("es-BR", 36), ("es-CL", 36), ("es-CO", 36),
  // ("es-EC", 36), ("es-PE", 36), ("es-PY", 36), ("es-UY", 36), ("es-VE", 36),
  // ("es-419", 35), ("es", 34), ("es-BZ", 34), ("es-CR", 34), ("es-CU", 34),
  // ("es-DO", 34), ("es-EA", 34), ("es-GQ", 34), ("es-GT", 34), ("es-HN", 34),
  // ("es-IC", 34), ("es-MX", 34), ("es-NI", 34), ("es-PA", 34), ("es-PH", 34),
  // ("es-PR", 34), ("es-SV", 34), ("es-US", 34)]

  let v = chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect_vec_with(|_| 10);
  dbg!(&v);

  Ok(())
}

#[ignore]
#[test]
fn test_init_es_mx_chain() -> AnyResult<()> {
  init_logger(true);

  let chain = try_init_chain_from_slice("es-Latn-MX", &es_list())?;
  // <(id, score)>:
  //  ("es-MX", 40), ("es-BZ", 36), ("es-CR", 36), ("es-GT", 36), ("es-HN", 36),
  // ("es-NI", 36), ("es-PA", 36), ("es-SV", 36), ("es-419", 35), ("es-CU", 35),
  // ("es-DO", 35), ("es-PR", 35), ("es-US", 35), ("es", 34), ("es-AR", 34),
  // ("es-BO", 34), ("es-BR", 34), ("es-CL", 34), ("es-CO", 34), ("es-EA", 34),
  // ("es-EC", 34), ("es-GQ", 34), ("es-IC", 34), ("es-PE", 34), ("es-PH", 34),
  // ("es-PY", 34), ("es-UY", 34), ("es-VE", 34)]

  let v = chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect_vec_with(|_| 10);
  dbg!(&v);

  Ok(())
}
