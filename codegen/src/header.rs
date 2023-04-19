use crate::generator::Generator;
use std::io::{self, Write};

impl<'ver, 'p, 'res> Generator<'ver, 'p, 'res> {
    pub(crate) fn defind_rs_file_header<W: Write>(
        &self,
        rs_file: &mut W,
    ) -> io::Result<()> {
        writeln!(rs_file, "// Version: {}", self.get_version())?;

        rs_file.write_all(b"#![allow(dead_code)]\n\n")?;

        rs_file.write_all(b"use super::{lang_id_consts, HashMap, LangID};\n\n")?;

        rs_file.write_all(
            b"pub(crate) type L10nMap = ::phf::Map<&'static str, &'static str>;\n",
        )?;
        rs_file.write_all(
            b"pub(crate) type SubLocaleMap = ::phf::Map<&'static str, fn() -> L10nMap>;\n",
        )?;
        rs_file.write_all(
            b"pub(crate) type LocaleMap = ::phf::Map<&'static str, fn() -> SubLocaleMap>;\n",
        )?;
        rs_file.write_all(
            b"pub(crate) type LocaleHashMap = HashMap<LangID, SubLocaleMap>;\n\n",
        )?;

        Ok(())
    }
}
