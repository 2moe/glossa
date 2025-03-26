use std::collections::BTreeMap;

use ahash::{HashMap, HashMapExt};
use kstring::KString;
use lang_id::{LangID, matches};
use tap::Tap;
use tmpl_resolver::resolver::OrderedAST;

use crate::{Generator, MiniStr};

pub(crate) type L10nBTreeMap = BTreeMap<(KString, KString), MiniStr>;
// [(lang, <(map_name, key), value>)]
pub(crate) type L10nMaps = Box<[(LangID, L10nBTreeMap)]>;

pub(crate) type L10nTemplateBTreeMap = BTreeMap<KString, OrderedAST>;

// [(lang, <map_name, map>)]
pub(crate) type L10nTemplateMaps = Box<[(LangID, L10nTemplateBTreeMap)]>;

impl<'h> Generator<'_, 'h> {
  #[cfg(not(feature = "highlight"))]
  pub(crate) fn flatten_highlight_maps(&'h self) -> Option<L10nMaps> {
    None
  }

  #[cfg(feature = "highlight")]
  pub(crate) fn flatten_highlight_maps(&'h self) -> Option<L10nMaps> {
    let flatten_entries = |entries: HashMap<KString, HashMap<_, _>>| {
      entries
        .into_iter()
        .flat_map(|(map_name, data)| {
          data
            .into_iter()
            .map(move |(k, v)| ((map_name.clone(), k), v))
        })
        .collect()
    };

    self
      .collect_highlight_maps()?
      .into_iter()
      .map(|(lang, entries)| {
        let lang = parse_language_id(&lang);
        let map = flatten_entries(entries);
        (lang, map)
      })
      .collect::<Box<_>>()
      .tap_mut(|x| x.sort_unstable())
      .into()
  }

  /// See also: [Self::get_or_init_merged_maps()]
  pub(super) fn merge_l10n_and_highlight_maps(&'h self) -> L10nMaps {
    let l10n_maps = || self.get_or_init_maps();

    let highlight_maps = self
      .get_or_init_highlight_maps()
      .expect("Failed to get highlight maps");

    let capacity = l10n_maps().len();

    l10n_maps()
      .iter()
      .chain(highlight_maps)
      .fold(
        HashMap::with_capacity(capacity),
        |mut acc, (map_name, map)| {
          acc
            .entry(map_name.clone())
            .and_modify(|existing_map: &mut L10nBTreeMap| {
              existing_map.append(&mut map.clone());
            })
            .or_insert_with(|| map.clone());
          acc
        },
      )
      .into_iter()
      .collect::<Box<_>>()
      .tap_mut(|x| x.sort_unstable())
  }
}

impl Generator<'_, '_> {
  pub(crate) fn flatten_l10n_maps(&self) -> L10nMaps {
    self
      .get_l10n_res_map()
      .iter()
      .map(|(lang, entries)| {
        let map = entries
          .iter()
          .filter_map(|entry| {
            let map_name = entry.map_name_to_kstring();
            entry
              .get_data()
              .as_ref()
              .map(|data| (map_name, data))
          })
          .flat_map(|(map_name, data)| {
            data
              .iter()
              .map(move |(k, v)| ((map_name.clone(), k.clone()), v.clone()))
          })
          .collect();
        (parse_language_id(lang), map)
      })
      .collect::<Box<_>>()
      .tap_mut(|x| x.sort_unstable())
  }

  pub(crate) fn flatten_template_maps(&self) -> L10nTemplateMaps {
    self
      .get_l10n_res_map()
      .iter()
      .map(|(lang, entries)| {
        let map = entries
          .iter()
          .filter_map(|entry| {
            let map_name = entry.map_name_to_kstring();

            entry
              .get_tmpl_data()
              .as_ref()
              .map(|tmpl| (map_name, tmpl.to_owned().into_btree_map()))
          })
          .collect();
        (parse_language_id(lang), map)
      })
      .collect::<Box<_>>()
      .tap_mut(|x| x.sort_unstable_by_key(|(id, _)| id.clone()))
  }
}

fn parse_language_id(lang: &str) -> LangID {
  const UND: LangID = lang_id::consts::lang_id_und();

  match matches::match_id(lang.as_bytes()) {
    UND => lang
      .parse()
      .unwrap_or_else(|e| {
        eprintln!("[WARN] Invalid Language ID. {e}");
        UND
      }),
    x => x,
  }
}

#[cfg(test)]
mod tests {
  type L10nHashMap = ahash::HashMap<(KString, KString), MiniStr>;

  use anyhow::Result as AnyResult;
  use testutils::dbg;

  use super::*;
  use crate::generator::dbg_generator::en_generator;

  #[ignore]
  #[test]
  fn test_collect_tmpl_maps() -> AnyResult<()> {
    let all_maps = en_generator().flatten_template_maps();

    let (_lang, map) = &all_maps[0];
    let toml = toml::to_string_pretty(&map)?;
    println!("{toml}");
    Ok(())
  }

  #[ignore]
  #[test]
  fn test_flatten_l10n_maps() -> AnyResult<()> {
    let all_maps = en_generator().flatten_l10n_maps();
    let (_lang, map) = &all_maps[0];
    dbg!(map);

    let cfg = bincode::config::standard();
    let bin = bincode::serde::encode_to_vec(&map, cfg)?;
    dbg!(bin.len(), bin.capacity());

    let (data, _size) =
      bincode::serde::decode_from_slice::<L10nHashMap, _>(&bin, cfg)?;
    dbg!(&data);
    let hello = get_l10n_map_value(&data, &["test", "👋🌐"]);
    dbg!(&hello);

    Ok(())
  }

  fn get_l10n_map_value<'a>(
    map: &'a L10nHashMap,
    key: &[&str; 2],
  ) -> Option<&'a MiniStr> {
    let [k1, k2] = key.map(KString::from_ref);
    map.get(&(k1, k2))
  }
}
