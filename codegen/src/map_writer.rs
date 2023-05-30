use getset::{Getters, MutGetters};
use lang_id::LangID;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs::File,
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

pub(crate) type DataMap = phf_codegen::Map<String>;
pub(crate) type TupStrMap<'a> = HashMap<(&'a str, String), (String, String)>;

#[derive(Getters, MutGetters, Debug)]
#[getset(get = "pub with_prefix", get_mut = "pub with_prefix")]
pub struct MapWriter<'v> {
    pub(crate) rs_path: &'v Path,
    pub(crate) tmp_path: &'v Path,
    tmp_file: BufWriter<File>,
    pub(crate) visibility: &'v str,
    pub(crate) gen_doc: bool,
}

pub fn get_shmem_path<P: AsRef<Path>>(rs_path: P) -> io::Result<PathBuf> {
    let p = rs_path.as_ref();
    let tmp = || p.with_extension("tmp");

    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::PermissionsExt;
        let shmem = Path::new("/dev/shm");
        let path = match shmem
            .metadata()?
            .permissions()
            .mode()
            & 0o200
        {
            0 => tmp(),
            _ if !shmem.is_dir() => tmp(),
            _ => shmem.join(
                p.file_name()
                    .expect("l10n.rs.tmp"),
            ),
        };

        Ok(path)
    }

    #[cfg(not(target_os = "linux"))]
    Ok(tmp())
}

impl<'v> MapWriter<'v> {
    pub fn new(tmp_path: &'v Path, rs_path: &'v Path) -> Self {
        Self {
            rs_path,
            tmp_path,
            tmp_file: BufWriter::new(
                File::create(tmp_path).expect("Failed to create tmp_file"),
            ),
            visibility: "pub(crate)",
            gen_doc: true,
        }
    }

    pub(crate) fn build_data_map(
        &mut self,
        fn_name: &str,
        map: &mut DataMap,
    ) -> io::Result<()> {
        writeln!(
            self.tmp_file,
            "{} const fn {}() -> L10nMap {{\n    {}\n}}\n",
            self.visibility,
            fn_name,
            map.build()
        )
    }

    pub(crate) fn build_data_doc(
        &mut self,
        map: &TupStrMap,
        lite_doc: bool,
    ) -> io::Result<()> {
        for ((loc, k1), (v1, v2)) in map.iter().take(1) {
            writeln!(
                self.tmp_file,
                "/// Language ID: {};\n/// Map name: {:?};",
                loc, k1
            )?;

            if let Some(desc) = lang_id::map::description::desc_map().get(loc) {
                writeln!(self.tmp_file, "/// Description: {};", desc)?;
            }

            if lite_doc {
                return Ok(());
            }

            writeln!(self.tmp_file, "///")?;

            writeln!(
                self.tmp_file,
                r#"/// # Example
///
/// ```no_run
/// let msg = loader.get_or_default({:?}, {:?});
///"#,
                k1, v1
            )?;

            writeln!(
                self.tmp_file,
                r##"/// assert_eq!(msg, {:?});
/// ```"##,
                v2
            )?;
        }

        Ok(())
    }

    fn gen_sub_locale_doc(&mut self, locale: &str) -> io::Result<()> {
        match lang_id::map::description::desc_map().get(locale) {
            Some(desc) => {
                writeln!(self.tmp_file, r#"/// {}: {}"#, locale, desc)?;
            }
            _ => {
                if let Ok(mut s) = locale.parse::<LangID>() {
                    s.maximize();
                    if s != locale {
                        writeln!(self.tmp_file, r#"/// {}: {}"#, locale, s)?;
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
            self.tmp_file,
            "{} const fn {}() -> SubLocaleMap {{\n    {}\n}}\n",
            self.visibility,
            lc_map_fn,
            map.build()
        )
    }

    fn gen_locale_phf_doc(&mut self) -> io::Result<()> {
        writeln!(
            self.tmp_file,
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
            self.tmp_file,
            "{} const fn locale_map() -> LocaleMap {{\n    {}\n}}\n",
            self.visibility,
            map.build()
        )
    }

    fn gen_lc_doc(&mut self) -> io::Result<()> {
        writeln!(
            self.tmp_file,
            r#"/// # Example
///
/// ```no_run
/// let loader = glossa::MapLoader::new(locale_hashmap());
///
/// dbg!(&loader);
/// ```"#
        )
    }
    pub(crate) fn build_lc_treemap(
        &mut self,
        // map: HashMap<&str, String>,
        map: BTreeMap<&str, String>, // rs_file: &mut W,
    ) -> io::Result<()> {
        let mut sets = HashSet::new();
        if self.gen_doc {
            self.gen_lc_doc()?;
        }
        self.tmp_file
            .write_all(self.visibility.as_bytes())?;
        self.tmp_file.write_all(
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
                        self.tmp_file,
                        r#"
    {SPACES}(("{k}").parse::<LangID>().expect("Invalid Language ID"), {v}()),"#,
                    )?;
                }
                name => {
                    write!(
                        self.tmp_file,
                        "\n{SPACES}(unsafe {{ {name} }}, {v}()),",
                    )?;
                }
            }
        }

        self.tmp_file.write_all(
            b"
    ])
}\n",
        )?;

        self.tmp_file.flush()
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
