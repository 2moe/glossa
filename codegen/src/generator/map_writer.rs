use lang_id::LangID;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    io::{self, Write},
};

pub(crate) type DataMap = phf_codegen::Map<String>;
type TupStrMap<'a> = HashMap<(&'a str, String), (String, String)>;

pub(crate) struct MapWriter<'v, W: Write> {
    pub(crate) rs_file: W,
    pub(crate) visibility: &'v str,
    pub(crate) gen_doc: bool,
}

impl<'v, W: Write> MapWriter<'v, W> {
    pub(crate) fn build_data_map(
        &mut self,
        fn_name: &str,
        map: &mut DataMap,
    ) -> io::Result<()> {
        writeln!(
            self.rs_file,
            "{} const fn {}() -> L10nMap {{\n    {}\n}}\n",
            self.visibility,
            fn_name,
            map.build()
        )
    }

    pub(crate) fn build_data_doc(&mut self, map: &TupStrMap) -> io::Result<()> {
        for ((loc, k1), (v1, v2)) in map.iter().take(1) {
            writeln!(
                self.rs_file,
                "/// Language ID: {};\n/// Map name: {};",
                loc, k1
            )?;

            if let Some(desc) = lang_id::map::description::desc_map().get(loc) {
                writeln!(self.rs_file, "/// Description: {};", desc)?;
            }
            writeln!(self.rs_file, "///")?;

            writeln!(
                self.rs_file,
                r#"/// # Example
///
/// ```no_run
/// use glossa::{{GetText, MapLoader}};
/// 
/// let loader = MapLoader::new(locale_hashmap());
/// let msg = loader.get_or_default("{}", "{}");
///"#,
                k1, v1
            )?;

            writeln!(
                self.rs_file,
                "/// assert_eq!(msg, r#\"{}\"#);
/// ```",
                v2
            )?;
        }

        Ok(())
    }

    fn gen_sub_locale_doc(&mut self, locale: &str) -> io::Result<()> {
        match lang_id::map::description::desc_map().get(locale) {
            Some(desc) => {
                writeln!(self.rs_file, r#"/// {}: {}"#, locale, desc)?;
            }
            _ => {
                if let Ok(mut s) = locale.parse::<LangID>() {
                    s.maximize();
                    if s != locale {
                        writeln!(self.rs_file, r#"/// {}: {}"#, locale, s)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn build_sub_locale_map(
        &mut self,
        lc_map_fn: &str,
        locale: &str,
        map: &mut DataMap,
    ) -> io::Result<()> {
        if self.gen_doc {
            self.gen_sub_locale_doc(locale)?;
        }
        writeln!(
            self.rs_file,
            "{} const fn {}() -> SubLocaleMap {{\n    {}\n}}\n",
            self.visibility,
            lc_map_fn,
            map.build()
        )
    }

    fn gen_locale_phf_doc(&mut self) -> io::Result<()> {
        writeln!(
            self.rs_file,
            r##"/// # Example
///
/// ```no_run
/// let map = locale_map();
///
/// for k in map.keys() {{
///     println!("{{k}}")
/// }}
///
/// map.get("en").map(|v| dbg!(v()));
/// ```"##
        )
    }

    pub(crate) fn build_locale_phf_map(
        &mut self,
        map: phf_codegen::Map<&String>,
    ) -> Result<(), io::Error> {
        if self.gen_doc {
            self.gen_locale_phf_doc()?;
        }

        writeln!(
            self.rs_file,
            "{} const fn locale_map() -> LocaleMap {{\n    {}\n}}\n",
            self.visibility,
            map.build()
        )
    }

    fn gen_lc_doc(&mut self) -> io::Result<()> {
        writeln!(
            self.rs_file,
            r#"/// # Example
///
/// ```no_run
/// let loader = glossa::MapLoader::new(locale_hashmap());
///
/// dbg!(&loader);
/// ```"#
        )
    }
    pub(crate) fn build_lc_hashmap(
        &mut self,
        // map: HashMap<&str, String>,
        map: BTreeMap<&str, String>, // rs_file: &mut W,
    ) -> io::Result<()> {
        let mut sets = HashSet::new();
        if self.gen_doc {
            self.gen_lc_doc()?;
        }
        self.rs_file
            .write_all(self.visibility.as_bytes())?;
        self.rs_file.write_all(
            br#" fn locale_hashmap() -> LocaleHashMap {
    use lang_id_consts::*;

    HashMap::from_iter(["#,
        )?;

        const SPACES: &str = "        ";
        for (k, v) in map {
            match k.parse::<LangID>() {
                Ok(lang) => {
                    if locale_collision_checker(&mut sets, lang) {
                        continue;
                    }
                }
                _ => continue,
            };

            match lang_id::get::get_fn_name(k.as_bytes()) {
                n if n.is_empty() => {
                    write!(
                        self.rs_file,
                        r#"
    {SPACES}(("{k}").parse::<LangID>().expect("Invalid Language ID"), {v}()),"#,
                    )?;
                }
                name => {
                    write!(self.rs_file, "\n{SPACES}(unsafe {{ {name} }}, {v}()),",)?;
                }
            }
        }

        self.rs_file.write_all(
            b"
    ])
}\n",
        )?;

        self.rs_file.flush()
        // Ok(())
    }
}

fn locale_collision_checker(sets: &mut HashSet<LangID>, current: LangID) -> bool {
    match sets.contains(&current) {
        y if y => y,
        no => {
            sets.insert(current);
            no
        }
    }
}
