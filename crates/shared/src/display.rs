use core::fmt::Debug;

/// => `println!("{msg}");`
pub fn puts<T: core::fmt::Display>(msg: &T) {
  println!("{msg}")
}

/// => `println!("{debug_msg:?}");`
///
/// > To print debug information to stderr, use [`eputs_dbg`] rather than
/// > `puts_dbg`.
pub fn puts_dbg<T: Debug>(debug_msg: &T) {
  println!("{debug_msg:?}")
}

/// => `eprintln!("{msg}");`
pub fn eputs<T: core::fmt::Display>(msg: &T) {
  eprintln!("{msg}")
}

/// => `eprintln!("{debug_msg:?}");`
pub fn eputs_dbg<T: Debug>(debug_msg: &T) {
  eprintln!("{debug_msg:?}")
}
