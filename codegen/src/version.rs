use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

/// Returns Ok(true) if it is the same version
/// For blank strings (spaces, tabs, etc.), it is always a different version because it is automatically trimmed out
pub fn is_same_version<P: AsRef<Path>>(
    path: P,
    version: Option<&str>,
) -> io::Result<bool> {
    let Some(ver) = version else {
        return Ok(false)
    };
    match path.as_ref() {
        p if !p.exists() => Ok(false),
        p => {
            let mut file = BufReader::new(File::open(p)?);
            let mut buf = String::with_capacity(120);

            while let 1.. = file.read_line(&mut buf)? {
                buf.make_ascii_lowercase();
                if buf.contains("version:") {
                    let mut iter = buf.trim().splitn(2, "version:");
                    iter.next();
                    return Ok(matches!(iter.next(), Some(v) if v.trim() == ver));
                }
                buf.clear()
            }
            Ok(false)
        }
    }
}
