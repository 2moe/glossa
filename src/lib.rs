//! # Glossa
//!
//! Glossa is a language localisation library for software.
//!
//! ## Get started
//!
//! Firstly, we need to add some dependencies:
//!
//! ```sh
//! cargo add glossa
//! cargo add lang-id
//! cargo add once_cell
//! ```
//!
//! If you don't need a custom language ID and would rather use automatic detection, you can skip the `lang-id` dependency.
//!
//! If you don't require an `ArcLoader` with a global variable or static lifetime, you can omit adding `once_cell`.
//!
//! Although the following example may appear lengthy, it actually consists of simple components.
//!
//! ```rust
//! use glossa::{
//!     resource::loader::{new_arc_loader, ArcLoader},
//!     LangRes,
//! };
//! use once_cell::sync::Lazy;
//! use std::{
//!     fs::{self, File},
//!     io::{self, Write},
//!     path::{Path, PathBuf},
//! };
//!
//! /// This function is used to create a directory and write the contents to the relevant file.
//! //
//! // Note: In practice, you don't actually need to create such a function.
//! // It's only written here for tutorial purposes.
//! fn create_l10n_text<P: AsRef<Path>>(
//!     path: P,
//!     append: bool,
//!     contents: &str,
//! ) -> io::Result<()> {
//!     let path = path.as_ref();
//!
//!     fs::create_dir_all(
//!             path.parent()
//!                 .expect(r#"Please bring your parent to meet me😅.
//!                 Just joke, this directory doesn't seem to contain the previous level, please double check.
//!                 For example: for `main.ftl`, you need to store it in "en/main.ftl" or "en-GB/main.ftl".
//!                 (The lang-id can be changed at will)"#),
//!         )?;
//!
//!     let mut file = File::options()
//!         .create(true)
//!         .append(append)
//!         .write(true)
//!         .open(path)?;
//!
//!     writeln!(file, "{contents}")
//! }
//!
//! fn main() {
//!     // Specify the fluent file for en(en-Latn-US)
//!     // On Windows, it's "locales\en\test.ftl".
//!     // On Unix, it's "locales/en/test.ftl".
//!     let mut arr = ["locales", "en", "test.ftl"];
//!     let mut file = PathBuf::from_iter(&arr);
//!     const IO_MSG: &str = "I/O Error, failed to create/write the file";
//!
//!     // We've created a file with only one line of data, where the key is "welcome" and the value is "Welcome to glossa🥳".
//!     create_l10n_text(file, false, "welcome = Welcome to glossa🥳").expect(IO_MSG);
//!
//!     // We change the second element from "en" to "de" to start our German localisation.
//!     if let Some(p) = arr.iter_mut().nth(1) {
//!         *p = "de"
//!     }
//!     file = PathBuf::from_iter(&arr);
//!
//!     // This is the localised content in German([de] de-Latn-DE: Deutsch, Lateinisch, Deutschland) that we'll use for testing later on.
//!     create_l10n_text(file, false, "welcome = Willkommen bei glossa😚")
//!         .expect(IO_MSG);
//!
//!     // We change the second element to "zh"
//!     if let Some(p) = arr.iter_mut().nth(1) {
//!         *p = "zh"
//!     }
//!     file = PathBuf::from_iter(arr);
//!
//!     // This is the localised content in Simplified Chinese.([zh] zh-Hans-CN: 简体中文, 中国)
//!     create_l10n_text(file, false, "welcome = 欢迎使用 glossa🥰").expect(IO_MSG);
//!
//!     const ERR_MSG: &str = "Failed to create arc loader";
//!
//!     // We've created a loader that reads localised resources at runtime.
//!     static LOADER: Lazy<ArcLoader> =
//!         Lazy::new(|| new_arc_loader(Path::new("locales"), None).expect(ERR_MSG));
//!
//!     // Although there is only Loader, not Lang, `from()` will automatically set your system language to the language of `LangRes`.
//!     let res = LangRes::from(&LOADER);
//!
//!     // You can think of `.find()` as the equivalent of `.get()` for a HashMap, but the difference is that it returns a `Result` instead of an `Option`.
//!     let text = res.find("welcome").expect(
//!         r#"Failed to get the value of "greeting" from locales/[lang-id]/test.ftl."#,
//!     );
//!
//!     // Since I'm not sure what language your system is in, I'm using match to determine the language, and then `assert_eq`.
//!     // In fact, this step is not needed at all.
//!     // When you call `find()`, the text will already be the localised text you want.
//!     // If it can't be found, then it's probably not what you want, but it will automatically use fallback. e.g. zh-Hant-HK -> zh-Hant -> zh -> en
//!     match res.language.as_str() {
//!         "zh" => assert_eq!(text, "欢迎使用 glossa🥰"),
//!         "de" => assert_eq!(text, "Willkommen bei glossa😚"),
//!         _ => assert_eq!(text, "Welcome to glossa🥳"),
//!     }
//! }
//! ```
//!
//! After reading this, you might think: why not just use a configuration file instead of all this?
//!
//! In reality, fluent has more advanced syntax than just "k = v".
//!
//! Don't worry, let's take it step by step.
//!
//! Let's assume the following file exists: **locales/en/test2.ftl**
//!
//! > Although we could use `create_l10n_text()` above to write the text to a file, for simplicity's sake, we'll just paste the content here!
//!
//! ```fluent
//! time-period = { $period ->
//!         *[morning] Good morning
//!         [evening] Good evening
//! }
//!
//! gender = { $attr ->
//!         [male] Mr.
//!         *[female] Ms.
//! }
//! # [walmart-bag]
//!
//! greetings = { time-period }! { gender }{ $name }
//! ```
//!
//! Since `greetings` has multiple parameters, we cannot use `find()`.
//! We need to use `find_with_kv()`.
//!
//! ```rust
//!     let text = res
//!         .find_with_kv(
//!             "greetings",
//!             [
//!                 ("period", "evening"),
//!                 ("name", "Alice"),
//!                 ("attr", "unknown"),
//!             ],
//!         )
//!         .expect(r#"Failed to get "greetings"! "#);
//! ```
//!
//! Let's take a guess at what the content of `text` is.
//!
//! Is it "Good evening! unknown Alice"?
//!
//! No, because `*` is the default option. If you pass in an unknown value, it automatically becomes the default value.
//!
//! Let's test it using `assert_eq`!
//!
//! ```rust
//! assert_eq!(text, "Good evening! Ms.Alice");
//! ```
//!
//! Let's give another example, this time in Japanese (ja-Jpan-JP).
//!
//! Since using `from()` will automatically detect our system language, we'll use `with_arc()` here to specify the language and ArcLoader.
//!
//! ```rust
//! let res = LangRes::with_arc(unsafe { lang_id::consts::get_ja() }, &LOADER);
//! ```
//!
//! Create a new file： **locales/ja/test.ftl**
//!
//! ```fluent
//! time-period = { $period ->
//!         [morning] おはようございます
//!         *[evening] こんばんは
//! }
//!
//! appellation = { $attr ->
//!         *[male] さん
//!         [young-male] 君
//!         [old-male] お爺さん
//!         [noble-man] 様
//!         [noble-young-male] お坊ちゃま
//!         [female] さん
//!         [young-female] ちゃん
//!         [old-female] お婆さん
//!         [noble-young-female] お嬢様
//! }
//!
//! greetings = { time-period }、{ $name }{ appellation }
//! ```
//!
//! Then we can use the `find_with_kv()` function.
//!
//! ```rust
//!     let text = res
//!         .find_with_kv(
//!             "greetings",
//!             [
//!                 ("period", "morning"),
//!                 ("name", "Alice"),
//!                 ("attr", "noble-young-female"),
//!             ],
//!         )
//!         .expect(r#"Failed to get "greetings"! "#);
//! ```
//!
//! Using `println!("{text}")`, we can see that the output is `おはようございます、Aliceお嬢様`
//!
//! After reading the content above, I believe you have a clear understanding of it now.
//!
//! However, this is only the beginning. If you want to learn more about fluent syntax, you can visit [Project Fluent](https://projectfluent.org/).
//! We won't go into further detail here.
//!

pub mod error;
pub mod resource;

pub type Result<T> = ::core::result::Result<T, error::GlossaError>;
pub use crate::resource::LangResource as LangRes;
