use getset::{Getters, WithSetters};
use glossa_shared::{
  ToCompactString,
  tap::Pipe,
  type_aliases::ahash::{HashMap, HashMapExt},
};
pub use hlight;
use hlight::HighlightResource;
pub use kstring::KString;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{Generator, MiniStr, resources::L10nMapEntry, to_kstr};

/// `[`(map_name & map_suffix, Highlight resource & syntax name)`]`
pub type HighlightCfgMap<'h> = HashMap<DerivedMapKey, SyntaxHighlightConfig<'h>>;

type KMap = HashMap<KString, MiniStr>;
type HighlightedMaps = HashMap<KString, KMap>;

#[derive(Getters, WithSetters, Debug, Clone)]
#[getset(get = "pub with_prefix", set_with = "pub")]
pub struct SyntaxHighlightConfig<'r> {
  resource: HighlightResource<'r>,
  syntax_name: MiniStr,
  true_color: bool,
}

impl Default for SyntaxHighlightConfig<'_> {
  fn default() -> Self {
    Self {
      resource: Default::default(),
      syntax_name: Default::default(),
      true_color: true,
    }
  }
}

#[derive(
  Debug,
  Clone,
  Hash,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Getters,
  WithSetters,
  Default, //
  Serialize,
  Deserialize,
)]
#[getset(get = "pub with_prefix", set_with = "pub")]
pub struct DerivedMapKey {
  /// map_name
  base_name: KString,
  /// map_suffix
  suffix: KString,
}

impl core::fmt::Display for DerivedMapKey {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let Self { base_name, suffix } = self;
    write!(f, "{base_name}{suffix}")
  }
}

impl DerivedMapKey {
  fn format(&self) -> KString {
    self
      .to_compact_string()
      .pipe(to_kstr)
  }
}

type SmallKeys<'k> = smallvec::SmallVec<&'k KString, 4>;
// type SmallKeys<'k> = Vec<&'k KString>;

/// -> `HashMap<base_name, Vec<suffix>>`
pub(crate) fn collect_derived_keys_to_map<'a>(
  map: &'a HighlightCfgMap,
) -> HashMap<&'a KString, SmallKeys<'a>> {
  map
    .keys()
    .map(|k| (k.get_base_name(), k.get_suffix()))
    .fold(HashMap::with_capacity(32), |mut grouped, (base, suffix)| {
      grouped
        .entry(base)
        .or_default()
        .push(suffix);
      grouped
    })
}

impl<'h> Generator<'h> {
  pub(crate) fn collect_highlight_maps(
    &'h self,
  ) -> Option<Box<[(KString, HighlightedMaps)]>> {
    let highlight_cfg_map = self.get_highlight().as_ref()?;
    let highlight_groups = collect_derived_keys_to_map(highlight_cfg_map);

    self
      .get_l10n_res_map()
      .par_iter()
      .map(|(lang_id, entries)| {
        let map = entries
          .par_iter()
          .filter_map(process_single_entry)
          .filter_map(|(base_map_name, map)| {
            highlight_groups
              .get(&base_map_name)?
              .par_iter()
              .filter_map(move |suffix| {
                generate_derived_map(highlight_cfg_map, &base_map_name, suffix)
              })
              .map(|(key, config)| {
                let new_map = generate_highlight_map(map, config);
                (key.format(), new_map)
              })
              .pipe(Some)
          })
          .flatten()
          .collect::<HashMap<_, _>>();
        (lang_id, map)
      })
      .filter_map(|(id, map)| (!map.is_empty()).then_some((id.clone(), map)))
      .collect::<Box<_>>()
      .into()
  }
}

fn generate_highlight_map<'a>(
  map: &'a KMap,
  config: &'a SyntaxHighlightConfig<'a>,
) -> KMap {
  let highlight_mapper = |(k, v)| config.apply_highlighting(k, v);
  map
    .par_iter()
    .filter_map(highlight_mapper)
    .collect()
}

fn generate_derived_map<'h>(
  highlight_cfg_map: &'h HighlightCfgMap<'h>,
  map_name: &KString,
  suffix: &KString,
) -> Option<(DerivedMapKey, &'h SyntaxHighlightConfig<'h>)> {
  let key = DerivedMapKey::default()
    .with_base_name(map_name.clone())
    .with_suffix(suffix.clone());

  highlight_cfg_map
    .get(&key)
    .map(|config| (key, config))
}

fn process_single_entry(entry: &L10nMapEntry) -> Option<(KString, &KMap)> {
  let map_name = entry.map_name_to_kstring();

  entry
    .get_data()
    .as_ref()
    .map(|data| (map_name, data))
}

impl<'a> SyntaxHighlightConfig<'a> {
  fn apply_highlighting(
    &'a self,
    k: &KString,
    v: &'a MiniStr,
  ) -> Option<(KString, MiniStr)> {
    let mut buffer = Vec::with_capacity(256);
    hlight::Highlighter::default()
      .with_resource(Some(&self.resource))
      .with_syntax_name(&self.syntax_name)
      .with_true_color(self.true_color)
      .with_content(v.as_str())
      .with_writer(Some(&mut buffer))
      .run()
      .ok()
      .map(|_| (k.clone(), MiniStr::from_utf8_lossy(&buffer)))
  }
}

#[cfg(test)]
pub(crate) mod dbg_shared {
  use super::*;

  pub(crate) fn new_highlight_map<'a>() -> HighlightCfgMap<'a> {
    let mut hmap = HighlightCfgMap::default();
    hmap.insert(
      DerivedMapKey::default()
        .with_base_name("md".into())
        .with_suffix("_md".into()),
      SyntaxHighlightConfig::default()
        .with_syntax_name("md".into())
        .with_true_color(false),
    );
    hmap.insert(
      DerivedMapKey::default()
        .with_base_name("md".into())
        .with_suffix("_md_ayu_dark".into()),
      SyntaxHighlightConfig::default()
        .with_resource(
          HighlightResource::default()
            .with_theme_name("ayu-light".into())
            .with_background(false),
        )
        .with_syntax_name("md".into()),
    );
    hmap.insert(
      DerivedMapKey::default()
        .with_base_name("t".into())
        .with_suffix("_toml".into()),
      SyntaxHighlightConfig::default().with_syntax_name("toml".into()),
    );
    hmap
  }
}

#[cfg(test)]
mod tests {
  use std::io;

  use glossa_shared::display::{puts, puts_dbg};
  use testutils::dbg;

  use super::*;
  use crate::{
    AnyResult,
    generator::{MapType, dbg_generator::en_generator},
    highlight::dbg_shared::new_highlight_map,
  };

  #[ignore]
  #[test]
  fn test_derived_map_key() {
    let map_key = DerivedMapKey::default()
      .with_base_name("island".into())
      .with_suffix("_md".into());
    let mut map = HashMap::new();
    map.insert(map_key, "apple");
    dbg!(map);
  }

  #[ignore]
  #[test]
  fn test_collect_map_keys() {
    let map = new_highlight_map();
    let v = collect_derived_keys_to_map(&map);
    /*
    v = {
        "unread": [ "_toml" ],
        "hi": [ "_md_one_dark", "_md" ],
    }
    */
    dbg!(v);
  }

  #[ignore]
  #[test]
  fn test_toml_highlight() -> io::Result<()> {
    use hlight::{HighlightResource, Highlighter};

    let s: &str = r#"
[main]
enabled = false
"😎" = "🍥"
float = nan
"#;

    let res = HighlightResource::default()
      .with_background(false)
      // .with_theme_name(ayu_dark())
      ;

    let mut writer = Vec::with_capacity(64);

    Highlighter::default()
      .with_syntax_name("toml")
      .with_content(s)
      .with_resource(Some(&res))
      .with_writer(Some(&mut writer))
      .run()?;

    let highlighted_text = String::from_utf8_lossy(&writer);
    puts(&highlighted_text);

    Ok(())
  }

  #[ignore]
  #[test]
  fn test_highlight() -> io::Result<()> {
    let hmap = new_highlight_map();
    let generator = en_generator().with_highlight(hmap);

    let maps = generator
      .collect_highlight_maps()
      .unwrap();

    for (map_name, map) in maps
      .iter()
      .flat_map(|(_, map)| map.iter())
    {
      puts(map_name);
      puts_dbg(map);
    }

    Ok(())
  }

  #[ignore]
  #[test]
  fn test_build_maps() -> AnyResult<()> {
    let hmap = new_highlight_map();
    let generator = en_generator().with_highlight(hmap);

    if let Some(boxed) = generator.get_or_init_highlight_maps() {
      for (idx, (lang, map)) in boxed.iter().enumerate() {
        println!("---- {idx}");
        std::dbg!(lang, map);
      }
    }

    generator.output_match_fn(MapType::Highlight)?;
    // generator.output_phf(MapType::Highlight)?;

    Ok(())
  }
}
