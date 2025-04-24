use alloc::{boxed::Box, vec::Vec};

use collect_with::{CollectVector, TryCollectWith};
use compact_str::{ToCompactString, format_compact};
use itertools::Itertools;
use lang_id::{
  error::LangidResult, maps::MaxLangID,
  matches::territory_containment_name::name_mapping,
};
use log::{debug, trace};
use smallvec::SmallVec;
use tap::{Pipe, Tap};
pub use testutils::dbg_ref;

use crate::{LangID, MiniStr, cldr_fallback_mapping};
/// Type alias for a collection of language identifiers with smallvec
/// optimization
pub type LocaleChain = SmallVec<LangID, 5>;
pub type LocaleStrChain = Box<[MiniStr]>;

/// Converts locale chain (`&[LangID]`) to `Box<[MiniStr]>`
///
/// # Example
///
/// ```
/// use lang_id::RawID;
/// use glossa::LangID;
/// use glossa::fallback::conv_to_str_chain;
/// use tap::Pipe;
///
/// const EN: LangID = lang_id::common::lang_id_en();
/// const DE: LangID = RawID::new(25956, None, None).into_lang_id();
///
/// let str_chain = [EN, DE].as_ref().pipe(conv_to_str_chain);
///
/// assert_eq!(str_chain.as_ref(), ["en", "de"]);
/// ```
pub fn conv_to_str_chain(chain: &[LangID]) -> LocaleStrChain {
  chain
    .iter()
    .map(|x| x.to_compact_string())
    .collect()
}

/// Initializes a locale chain from string slices
///
/// ## Parameters
///
/// - `current` - Current locale identifier as string slice
/// - `all_locales` - Available locale identifiers as string slices
///
/// ## Example
///
/// ```
/// use glossa::{
///   error::GlossaError, fallback::conv_to_str_chain,
///   try_init_chain_from_slice,
/// };
///
/// let chain = try_init_chain_from_slice(
///   "gsw-LI",
///   &[
///      "en", "es", "pt", "zh", "gsw", "gsw-FR", "gsw-LI", "de", "de-AT", "de-BE", "de-CH", "de-IT",
///     "de-LI", "de-LU",
///   ],
/// )?;
/// // <(id, score)>:
/// // [("gsw-LI", 50), ("gsw", 37), ("gsw-FR", 37), ("de-LI", 27), ("de", 26),
///  // ("de-AT", 23), ("de-BE", 23), ("de-CH", 23), ("de-LU", 23), ("de-IT", 22)]
///
/// let v = conv_to_str_chain(&chain);
///
/// assert_eq!(
///   v.as_ref(),
///   [
///     "gsw-LI", "gsw", "gsw-FR", "de-LI", "de", "de-AT", "de-BE", "de-CH",
///     "de-LU", "de-IT",
///   ]
/// );
///
/// # Ok::<(), GlossaError>(())
/// ```
pub fn try_init_chain_from_slice(
  current: &str,
  all_locales: &[&str],
) -> LangidResult<LocaleChain> {
  let current_language = current.parse()?;
  let all_locales = all_locales
    .iter()
    .map(|x| x.parse::<LangID>())
    .try_collect_vec_with(|_| 10)?;

  try_init_chain(&current_language, &all_locales)
}

/// Constructs an optimized string-based locale priority chain
///
/// # Stages:
///
/// 1. Locale chain [initialization](try_init_chain)
/// 2. [append_en()]
/// 3. Compact string [conversion](conv_to_str_chain)
/// 4. Return EN-only chain on failure
pub fn init_str_chain(current: &LangID, all_locales: &[LangID]) -> LocaleStrChain {
  match try_init_chain(current, all_locales) {
    Ok(mut v) => {
      append_en(&mut v);
      conv_to_str_chain(&v)
    }
    _ => [lang_id::common::lang_id_en()]
      .as_ref()
      .pipe(conv_to_str_chain),
  }
}

/// Initializes locale chain with scoring system
///
/// ## Score Calculation Rules
///
/// - exactly the same => 50
/// - same language => +20
/// - same script => +15
/// - same region => +4
/// - CLDR fallback exact match => +3
/// - CLDR fallback lang+script match => +6
/// - Nearby regions (e.g., assume current_region = FR; FR => (Western Europe)
///   `["AT", "BE", "CH", "DE", "FR", "LI", "LU", "MC", "NL"]` if item_region ==
///   "DE") => +2
///   - Continent match (e.g., FR => (Europe) if item_region in Europe) => +1
///
/// See also: [try_init_chain_from_slice]
pub fn try_init_chain(
  current: &LangID,
  all_locales: &[LangID],
) -> LangidResult<LocaleChain> {
  let max_current = MaxLangID::new(current);
  let cur_language = max_current.get_language();

  debug!("---------");
  dbg_ref!(
    cur_language,
    max_current.get_script(),
    max_current.get_region()
  );
  {
    trace!(
      "all_locales: {:?}",
      all_locales
        .iter()
        .map(|x| x.to_compact_string())
        .collect::<SmallVec<_, 10>>()
    )
  }

  let cldr_fallback_ids = try_collect_cldr_ids(&max_current)?;
  {
    trace!("cldr_fallback_ids: [");
    trace!(
      "// {}",
      cldr_fallback_ids
        .iter()
        .map(|x| x.to_compact_string())
        .map(|x| trace!("  {x}, "))
        .count()
    );
    trace!("]");
  }

  // let default_locales = [lang_id_en()];
  // match all_locales {
  //   [] => default_locales.iter(),
  //   x => x.iter(),
  // }
  all_locales
    .iter()
    .filter(|id| {
      id.language == cur_language
        || cldr_fallback_ids
          .iter()
          .map(|x| x.get_language())
          .any(|item| id.language == item)
    })
    .map(|id| {
      let score = match id == current {
        true => 50,
        _ => calculate_locale_similarity(
          &max_current,
          &cldr_fallback_ids,
          MaxLangID::new(id),
        ),
      };
      dbg_ref!(score);
      (id, score)
    })
    .collect_vec_with(|_| 24)
    .tap_mut(|v| v.sort_unstable_by_key(|&(_, score)| core::cmp::Reverse(score)))
    .tap(|v| {
      debug!(
        "locale chain<(id, score)>: {:?}",
        v.iter()
          .map(|(id, score)| (id.to_compact_string(), score))
          .collect::<SmallVec<_, 10>>()
      )
    })
    .into_iter()
    .map(|(id, _score)| id.clone())
    .collect::<LocaleChain>()
    .pipe(Ok)
}

/// Appends [English locale](lang_id::common::lang_id_en()) to the chain if not
/// present.
///
/// Returns `true` if English was added, `false` if already present.
pub fn append_en(chain: &mut LocaleChain) -> bool {
  const EN: LangID = lang_id::common::lang_id_en();

  match chain.iter().any(|x| x == &EN) {
    true => false,
    _ => {
      chain.push(EN);
      true
    }
  }
}

/// Gets hierarchy information for a region
fn get_hierarchy(data: &str) -> Option<&&str> {
  data
    .as_bytes()
    .pipe(name_mapping)
    .first()
}

/// Calculates similarity score between locales
fn calculate_locale_similarity(
  max_current: &MaxLangID,
  cldr_fallback_ids: &[MaxLangID],
  max_item: MaxLangID,
) -> u8 {
  let (cur_language, cur_script, cur_region) = (
    max_current.get_language(),
    max_current.get_script(),
    max_current.get_region(),
  );

  let (item_language, item_script, item_region) = (
    max_item.get_language(),
    max_item.get_script(),
    max_item.get_region(),
  );
  debug!("---------");
  dbg_ref!(item_language, item_script, item_region);

  let base_score = calculate_base_score([
    item_language == cur_language,
    item_script == cur_script,
    item_region == cur_region,
  ]);

  let region_score = {
    let current_nearby = max_current
      .get_region()
      .pipe(get_hierarchy);
    let item_nearby = max_item
      .get_region()
      .pipe(get_hierarchy);

    calculate_region_score(item_region, current_nearby, item_nearby)
  };

  // CLDR fallback list scoring
  let cldr_score = {
    let same_language = |x: &MaxLangID| item_language == x.get_language();
    let same_region = |x: &MaxLangID| item_region == x.get_region();
    let same_script = |x: &MaxLangID| item_script == x.get_script();

    let exactly_same = cldr_fallback_ids
      .iter()
      .any(|x| same_language(x) && same_region(x) && same_script(x));

    let partially_same = cldr_fallback_ids
      .iter()
      .any(|x| same_language(x) && same_script(x));

    calculate_cldr_score(exactly_same, partially_same)
  };

  base_score + region_score + cldr_score
}

/// Region hierarchy scoring
fn calculate_region_score(
  item_region: &str,
  current_nearby: Option<&&str>,
  item_nearby: Option<&&str>,
) -> u8 {
  match current_nearby {
    None => 0,
    Some(r)
      if item_nearby
        .map(|x| x == r)
        .unwrap_or(false) =>
    {
      trace!("Nearby Region, score+2");
      2
    }
    Some(r)
      if r
        .as_bytes()
        .pipe(name_mapping)
        .contains(&item_region) =>
    {
      trace!("current continent contains item_region, score+1");
      1
    }
    Some(r) => match get_hierarchy(r) {
      Some(rr)
        if item_nearby
          .and_then(|x| get_hierarchy(x))
          .map(|x| x == rr)
          .unwrap_or(false) =>
      {
        trace!("Continent, score+1");
        1
      }
      _ => 0,
    },
  }
}

/// Base score calculation for core components
fn calculate_base_score(iter: [bool; 3]) -> u8 {
  iter
    .into_iter()
    .zip([20, 15, 4])
    .filter(|&(cond, _)| cond)
    .map(|(_, score)| score)
    .inspect(|score| match score {
      n @ 20 => trace!("Same Language, score+{n}"),
      n @ 15 => trace!("Same Script, score+{n}"),
      n @ 4 => trace!("Same Region, score+{n}"),
      _ => {}
    })
    .sum()
}

fn calculate_cldr_score(
  cldr_id_exactly_same: bool,
  cldr_partially_same: bool,
) -> u8 {
  [(cldr_id_exactly_same, 3), (cldr_partially_same, 6)]
    .into_iter()
    .filter(|&(cond, _)| cond)
    .map(|(_, score)| score)
    .inspect(|score| match score {
      n @ 3 => trace!("cldr-fallback-id (exactly the same), score +{n}"),
      n @ 6 => trace!("cldr-fallback-id, score +{n}"),
      _ => {}
    })
    .sum()
}

/// Collects CLDR fallback IDs with priority fallback logic
fn try_collect_cldr_ids(max_current: &MaxLangID) -> LangidResult<Vec<MaxLangID>> {
  use collect_cldr_fallback_ids as collect;

  let (cur_language, cur_script, cur_region) = (
    max_current.get_language(),
    max_current.get_script(),
    max_current.get_region(),
  );
  // Generate fallback candidates
  let lang_and_region = format_compact!("{cur_language}-{cur_region}");
  let lang_and_script = format_compact!("{cur_language}-{cur_script}",);

  // Priority-based fallback collection
  match collect(&max_current.to_compact_string())? {
    x if x.is_empty() => match collect(&lang_and_region)? {
      r_list if r_list.is_empty() => match collect(&lang_and_script)? {
        s_list if s_list.is_empty() => collect(cur_language),
        s_list => s_list.pipe(Ok),
      },
      r_list => r_list.pipe(Ok),
    },
    x => x.pipe(Ok),
  }
}

/// Collects CLDR fallback IDs from raw mapping data
fn collect_cldr_fallback_ids(
  current_language: &str,
) -> LangidResult<Vec<MaxLangID>> {
  cldr_fallback_mapping(current_language.as_bytes())
    .tap(|x| trace!("language: {current_language}, raw_cldr_fallback_list: {x:?}"))
    .iter()
    .map(|x| x.parse::<LangID>())
    .map_ok(|x| MaxLangID::new(&x))
    .try_collect_vec_with(|_| 8)
}

#[cfg(test)]
pub(crate) mod dbg_shared {
  pub(crate) fn init_logger(trace: bool) {
    let level = {
      use log::LevelFilter::*;
      match trace {
        true => Trace,
        _ => Debug,
      }
    };

    env_logger::builder()
      .filter_level(level)
      .init()
  }
}

#[cfg(test)]
mod tests {
  pub use anyhow::Result as AnyResult;
  use collect_with::TryCollectWith;
  use compact_str::ToCompactString;
  use itertools::Itertools;
  use lang_id::{
    maps::MaxLangID, matches::territory_containment_name::name_mapping,
  };
  use tap::{Pipe, Tap};

  use super::*;
  use crate::{cldr_fallback_mapping, fallback::dbg_shared::init_logger};

  #[ignore]
  #[test]
  fn test_rev_sort_arr() {
    init_logger(false);
    let mut arr = [("en-US", 7), ("en", 4), ("en-001", 6)];
    arr.sort_unstable_by_key(|&(_, n)| core::cmp::Reverse(n));
    dbg_ref!(arr);
  }

  #[ignore]
  #[test]
  fn test_init_gsw_chain() -> AnyResult<()> {
    init_logger(true);
    let chain = try_init_chain_from_slice(
      "gsw-LI",
      &[
        "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI", "de-LU", "en", "es",
        "pt", "gsw", "gsw-FR", "gsw-LI",
      ],
    )?;
    // <(id, score)>:
    // [("gsw-LI", 50), ("gsw", 37), ("gsw-FR", 37), ("de-LI", 27), ("de", 26),
    // ("de-AT", 23), ("de-BE", 23), ("de-CH", 23), ("de-LU", 23), ("de-IT", 22)]

    let v = chain
      .iter()
      .map(|x| x.to_compact_string())
      .collect_vec_with(|_| 10);

    assert_eq!(
      v,
      [
        "gsw-LI", "gsw", "gsw-FR", "de-LI", "de", "de-AT", "de-BE", "de-CH",
        "de-LU", "de-IT",
      ]
    );

    Ok(())
  }

  #[ignore]
  #[test]
  fn test_init_zh_mo_chain() -> AnyResult<()> {
    init_logger(true);
    let chain = try_init_chain_from_slice(
      "zh-Hant-MO",
      &[
        "de",
        "ru",
        "zh-Latn",
        "ar",
        "en",
        "es",
        "pt",
        "zh-SG",
        "zh",
        "zh-Hans",
        "zh-Hant",
        "zh-Hant-TW",
        "zh-Hant-HK",
        "zh-MO",
      ],
    )?;
    // <(id, score)>:
    //  [("zh-MO", 46), ("zh-Hant-HK", 45), ("zh-Hant", 42), ("zh-Hant-TW", 42),
    // ("zh", 31), ("zh-Hans", 31), ("zh-SG", 27), ("zh-Latn", 22)]

    let v = conv_to_str_chain(&chain);

    assert_eq!(
      v.as_ref(),
      [
        "zh-MO",
        "zh-Hant-HK",
        "zh-Hant",
        "zh-Hant-TW",
        "zh",
        "zh-Hans",
        "zh-SG",
        "zh-Latn",
      ]
    );

    Ok(())
  }

  #[cfg(feature = "std")]
  #[ignore]
  #[test]
  /// prototype implementation
  fn test_collect_gsw_chain() -> AnyResult<()> {
    let current: LangID = "gsw-LI".parse()?;
    let max_current = MaxLangID::new(&current);

    dbg!(
      max_current.get_language(),
      max_current.get_script(),
      max_current.get_region()
    );

    let all_locales = [
      "gsw", "gsw-FR", "gsw-LI", "de", "de-AT", "de-BE", "de-CH", "de-IT", "de-LI",
      "de-LU", "en", "es", "pt",
    ]
    .into_iter()
    .map(|x| x.parse::<LangID>())
    .try_collect_vec_with(|_| 10)?;

    // dbg!(all_locales);
    let cldr_fallback_ids = cldr_fallback_mapping(
      max_current
        .get_language()
        .as_bytes(),
    )
    .into_iter()
    .map(|x| x.parse::<LangID>())
    .map_ok(|x| MaxLangID::new(&x))
    .try_collect_vec_with(|_| 8)?;

    let available_languages = all_locales
      .iter()
      .filter(|id| {
        id.language == current.language.as_str()
          || cldr_fallback_ids
            .iter()
            .map(|x| x.get_language())
            .any(|item| id.language == item)
      })
      .collect_vec();

    let list = available_languages
      .into_iter()
      .map(|id| {
        if id == &current {
          return (id, 32);
        }

        let mut score = 0u8;
        let max_item = MaxLangID::new(id);

        if max_item.get_language() == max_current.get_language() {
          score += 10
        }
        if max_item.get_script() == max_current.get_script() {
          score += 6
        }
        if max_item.get_region() == max_current.get_region() {
          score += 4
        }

        fn get_hierarchy(data: &str) -> Option<&&str> {
          data
            .as_bytes()
            .pipe(name_mapping)
            .first()
        }

        let current_nearby = max_current
          .get_region()
          .pipe(get_hierarchy);

        let item_nearby = max_item
          .get_region()
          .pipe(get_hierarchy);

        match current_nearby {
          Some(r)
            if item_nearby
              .map(|x| x == r)
              .unwrap_or(false) =>
          {
            score += 2
          }
          _ => match current_nearby.and_then(|x| get_hierarchy(x)) {
            Some(rr)
              if item_nearby
                .and_then(|x| get_hierarchy(x))
                .map(|x| x == rr)
                .unwrap_or(false) =>
            {
              score += 1
            }
            _ => {}
          },
        }

        let same_language =
          |x: &MaxLangID| max_item.get_language() == x.get_language();
        let same_region = |x: &MaxLangID| max_item.get_region() == x.get_region();
        let same_script = |x: &MaxLangID| max_item.get_script() == x.get_script();

        if cldr_fallback_ids
          .iter()
          .any(|x| same_language(x) && same_region(x) && same_script(x))
        {
          score += 3
        }

        if cldr_fallback_ids
          .iter()
          .any(|x| {
            // let same_lang_and_region = same_language(x) && same_region(x);
            let same_lang_and_script = same_language(x) && same_script(x);
            same_lang_and_script
          })
        {
          score += 1
        }
        (id, score)
      })
      .map(|(id, score)| (id.to_compact_string(), score))
      .collect_vec()
      .tap_mut(|v| v.sort_unstable_by_key(|&(_, score)| core::cmp::Reverse(score)));

    println!("{list:?}");
    assert_eq!(
      list,
      [
        ("gsw-LI", 32),
        ("gsw", 18),
        ("gsw-FR", 18),
        ("de-LI", 13),
        ("de", 12),
        ("de-AT", 9),
        ("de-BE", 9),
        ("de-CH", 9),
        ("de-LU", 9),
        ("de-IT", 8)
      ]
      .map(|(k, v)| (k.into(), v))
    );
    // dbg!(list);

    Ok(())
  }

  #[ignore]
  #[test]
  // #[cfg(not(feature = "std"))]
  fn test_lang_id_en() {
    extern crate std;
    const EN: LangID = lang_id::common::lang_id_en();
    std::dbg!(EN);
  }
}
