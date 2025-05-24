# glossa

[![glossa.crate](https://img.shields.io/crates/v/glossa.svg?logo=rust&logoColor=lightsalmon&label=glossa)](https://crates.io/crates/glossa)

[![Documentation](https://docs.rs/glossa/badge.svg)](https://docs.rs/glossa)
[![Apache-2 licensed](https://img.shields.io/crates/l/glossa.svg?logo=apache)](../License)

<!-- Language -->
<details open>
<summary>
<img alt="Language/語言" src="./svg/language.svg" />
</summary>

- [zh-Hant: 繁體中文](Readme-zh-Hant.md)
- [en: English](Readme.md)
- [zh: 簡體中文](Readme-zh.md)

</details>

<!-- TOC -->
<details open>
<summary>
<img alt="目錄" src="./svg/toc/目錄.svg"/>
</summary>

- [Locale Fallback 鏈](#locale-fallback-鏈)
  - [案例：zh-Hans-HK](#案例zh-hans-hk)
  - [案例：en-AU](#案例en-au)
  - [例子: gsw-LI](#例子-gsw-li)
- [實戰](#實戰)
  - [codegen](#codegen)
  - [LocaleContext](#localecontext)
  - [Trait 例子](#trait-例子)
  - [雙語](#雙語)

</details>

<!--  -->
## Locale Fallback 鏈

glossa crate 的核心功能：

- 根據當前語言與所有語言的**相似性**，生成一個數組。
  - （理論上）與當前語言的相似性越高，排名越前。

Q: 為什麼需要 fallback?

A:
因為當 current locale 的本地化文字缺失時，fallback 到您更為熟悉的語言（e.g., 當前語言的其他變體）可以確保良好的使用者體驗。

> 一個人可以掌握不同的語言（或者是同一門語言的不同變體）

假設當前 locale 為 pt-PT(português, Portugal), 所有本地化資源的 locales 為 pt-PT, pt(português, Brasil), es-419(español, Latinoamérica), en。

此時 i18n 庫應根據 `[pt-PT, pt, en]` 的順序來獲取本地化文字，而非 `[pt-PT, en]`。

若忽略了語言的相似性，直接 fallback 到 en，則不僅降低了本地化(L10n)覆蓋率，還可能會增加使用者的認知負擔。

### 案例：zh-Hans-HK

假設當前 locale 為 zh-Hans-HK, 所有本地化資源的 locales 為 zh-Hant-MO, zh-SG, ru, zh-Hant, fr, zh, ar, zh-HK, en-001, lzh。

呼叫 `try_init_chain()` 後，自動生成的 locale 鏈為: `["zh", "zh-SG", "zh-HK", "zh-Hant-MO", "zh-Hant"]`

當 log level 為 debug 或 trace 時，我們能看到 `[... DEBUG glossa::fallback] ...<(id, score)>`:

```rust
[
  ("zh", 37), // zh-Hans-CN
  ("zh-SG", 36), // zh-Hans-SG
  ("zh-HK", 35), // zh-Hant-HK
  ("zh-Hant-MO", 31)
  ("zh-Hant", 28) // zh-Hant-TW
]
```

> 分數越高，優先順序越高

- 完全相同，得滿分（50分）
- 部分相同
  - 相同語言：+20分
    - 由於當前語言為 zh (中文)，且內建規則不包含其他語言，因此語言鏈裡只有 ^zh。
    - 從理論上來說， lzh（文言）與現代漢語之間也有一定的相似性，只不過內建的 zh-Hans-HK 的 fallback 規則不包含 lzh。
  - 相同 script：+15分
    - 當前 script 為 Hans (簡體)，Hans 的分數比 Hant 更高
      - zh-HK 本質上是 zh-Hant-HK。
        - 由於 Hans 的分數高於 Hant，且當前存在 ^zh-Hans 的本地化資源，因此 zh-HK 的分數並不是最高的。
  - 符合內建 fallback 規則的語言，享受加分
    - 完全符合：+3+6 => +9分
    - 只符合 lang+script：+6分
  - 相同region：+4分
    - 將 zh-Hant (zh-Hant-TW), zh-Hant-MO, zh-HK (zh-Hant-HK) 進行對比
      - zh-HK 與當前 locale (zh-Hans-HK) 是相同的區域(HK)。
        - zh-HK: +4分
      - 由於 zh-Hant 與 zh-Hant-MO 的區域與當前區域（HK） 不同，故無法享受 +4 分的優待。
  - 相近地域，享受加分
    - 共同位於同一大洲的子區域（比如同時位於東亞地區）: +2分
    - 共同位於同一大洲 (比如同時位於亞洲): +1分
    - 將 zh(zh-Hans-CN) 與 zh-SG (zh-Hans-SG) 進行對比。
      - 當前 locale (zh-Hans-HK) 的區域為 HK, HK(中國香港)作為中國的一部分，與CN(中國內地)共同位於東亞地區；而 SG(新加坡) 位於東南亞，與 HK 共同位於亞洲。
        - zh: +2分
        - zh-SG: +1分

### 案例：en-AU

假設當前 locale 為 en-AU，不同地區的本地化資源非常齊全（包括幾乎無人的小島嶼）。

從語言相似性的角度來說，en-NZ (New Zealand English) 與 en-AU (Australian English) 的關係，會比 en-GB(British English) 更為密切。

遺憾的是， glossa 生成的 chain 不能保證 100% 的準確度。

```rust
// <(id, score)>:
[
  ("en-AU", 50), ("en-GB", 44), ("en-CC", 43), ("en-CX", 43), ("en-NF", 43),
  ("en-NZ", 43), ("en-UM", 42), ("en-CK", 42), ("en-DG", 42), ("en-FJ", 42),
  ("en-FM", 42), ("en-KI", 42), ("en-NR", 42), ("en-NU", 42), ("en-PG", 42),
  ("en-PN", 42), ("en-PW", 42), ("en-SB", 42), ("en-TK", 42), ("en-TO", 42),
  ("en-TV", 42), ("en-VU", 42), ("en-WS", 42), ("en-AS", 42), ("en-GU", 42),
  ("en-MH", 42), ("en-MP", 42), ("en-US", 22), ...
]
```

### 例子: gsw-LI

> gsw 是瑞士德語(Schwiizertüütsch)，de 是德國德語(Deutsch)。

```rust
use glossa::{
  error::GlossaError, fallback::conv_to_str_chain,
  try_init_chain_from_slice,
};

let chain = try_init_chain_from_slice(
  // current:
  "gsw-LI",

  // all_locales:
  &[
     "en", "es", "pt", "zh", "gsw", "gsw-FR", "gsw-LI", "de", "de-AT", "de-BE", "de-CH", "de-IT",
    "de-LI", "de-LU",
  ],
)?;
// <(id, score)>:
// [ ("gsw-LI", 50), ("gsw", 37), ("gsw-FR", 37), ("de-LI", 27), ("de", 26),
//   ("de-AT", 23), ("de-BE", 23), ("de-CH", 23), ("de-LU", 23), ("de-IT", 22) ]

let v = conv_to_str_chain(&chain);

assert_eq!(
  v.as_ref(),
  [
    "gsw-LI", "gsw", "gsw-FR", "de-LI", "de", "de-AT", "de-BE", "de-CH",
    "de-LU", "de-IT",
  ]
);
```

## 實戰

> 我們需要根據 glossa-codegen 生成的**本地化資源(L10n Map)**的型別，來實現相應的邏輯。

### codegen

```rust
use glossa_codegen::{Generator, L10nResources, Visibility, generator::MapType};

let generator = Generator::default()
  .with_resources(L10nResources::new("locales").with_include_map_names(["yes-no"]))
  .with_visibility(Visibility::Pub);
```

Generator 支援輸出多種不同的型別。
若我們呼叫了 `generator.output_match_fn_all_in_one_by_language_and_key(MapType::Regular)?`，則其輸出的內容如下所示。

```rust
pub const fn map(language: &[u8], key: &[u8]) -> &'static str {
  match (language, key) {
    (b"cs", b"cancel") => r#####"Zrušit"#####,
    (b"cs", b"no") => r#####"Ne"#####,
    (b"cs", b"yes") => r#####"Ano"#####,
    (b"de", b"cancel") => r#####"Abbrechen"#####,
    (b"de", b"no") => r#####"Nein"#####,
    (b"de", b"yes") => r#####"Ja"#####,
    (b"en", b"cancel") => r#####"Cancel"#####,
    (b"en", b"no") => r#####"No"#####,
    (b"en", b"ok") => r#####"OK"#####,
    (b"en", b"yes") => r#####"Yes"#####,
    (b"es", b"cancel") => r#####"Cancelar"#####,
    (b"es", b"ok") => r#####"Aceptar"#####,
    (b"es", b"yes") => r#####"Sí"#####,
    (b"fr", b"cancel") => r#####"Annuler"#####,
    (b"fr", b"no") => r#####"Non"#####,
    (b"fr", b"yes") => r#####"Oui"#####,
    (b"ja", b"cancel") => r#####"取消"#####,
    (b"ja", b"no") => r#####"いいえ"#####,
    (b"ja", b"ok") => r#####"瞭解"#####,
    (b"ja", b"yes") => r#####"はい"#####,
    (b"ko", b"cancel") => r#####"취소"#####,
    (b"ko", b"no") => r#####"아니오"#####,
    (b"ko", b"ok") => r#####"확인"#####,
    (b"ko", b"yes") => r#####"예"#####,
    (b"ru", b"no") => r#####"Нет"#####,
    (b"ru", b"yes") => r#####"Да"#####,
    (b"zh-Hant", b"cancel") => r#####"取消"#####,
    (b"zh-Hant", b"no") => r#####"否"#####,
    (b"zh-Hant", b"ok") => r#####"確定"#####,
    (b"zh-Hant", b"yes") => r#####"是"#####,
    (b"zh-Latn-CN", b"cancel") => r#####"QuXiao"#####,
    (b"zh-Latn-CN", b"no") => r#####"Fou"#####,
    (b"zh-Latn-CN", b"ok") => r#####"QueDing"#####,
    (b"zh-Latn-CN", b"yes") => r#####"Shi"#####,
    _ => "",
  }
}
```

呼叫 `generator.output_locales_fn(MapType::Regular, true)?` 後, 我們將得到如下函式。

```rust
// super: use glossa_shared::lang_id;

pub const fn all_locales() -> [super::lang_id::LangID; 10] {
  #[allow(unused_imports)]
  use super::lang_id::RawID;
  use super::lang_id::consts::*;
  [
    lang_id_cs(),
    lang_id_de(),
    lang_id_en(),
    lang_id_es(),
    lang_id_fr(),
    lang_id_ja(),
    lang_id_ko(),
    lang_id_ru(),
    lang_id_zh_hant(),
    lang_id_zh_pinyin(),
  ]
}
```

### LocaleContext

接下來，我們需要根據 codegen 生成的程式碼/資料的型別，來實現查詢本地化文字的邏輯。
由上文可知，codegen 生成了 `match_fn`。

根據函式的定義： `const fn map(language: &[u8], key: &[u8]) -> &'static str`

我們編寫了如下的查詢邏輯:

```rust
let lookup = |(language, key)| match map(language, key) {
  "" => None,
  s => Some(s),
};
```

如果 codegen 生成的函式的定義為： `const fn map(language: &[u8], map_name: &[u8], key: &[u8]) -> &'static str`，則查詢邏輯也不一樣。

我們可以這樣子寫：

```rust
let lookup = |(language, map_name, key)| match map(language, map_name, key) {
  "" => None,
  s => Some(s),
};
```

若 codegen 生成了 bincode file，則將其反序列化後，會得到普通的 HashMap 或 BTreeMap。

我們可以使用 `map.get(&language)?.get(&(map_name, key))` 進行查詢。

```rust
let map = glossa_shared::decode::file::decode_file_to_maps(path)?;
let lookup = |language, tuple_key| {
  map
    .get(language)?
    .get(&tuple_key)
};
```

### Trait 例子

```rust
use glossa::{LocaleContext, traits::ChainProvider};

trait GetL10nText: ChainProvider {
  fn try_get_by_key<'t>(&self, key: &[u8]) -> Option<&'t str> {
    let lookup = |(language, key)| match map(language, key) {
      "" => None,
      s => Some(s),
    };

    self
      .provide_chain()?
      .iter()
      .map(|id| (id.as_bytes(), key))
      .find_map(lookup)
  }
}

impl GetL10nText for LocaleContext {}

#[test]
pub(crate) fn print_l10n_text() {
  let new_ctx = || LocaleContext::default().with_all_locales(all_locales());

  // #[cfg(any(target_os = "macos", target_os = "linux"))]
  let set_env_lang = |value| unsafe { std::env::set_var("LANG", value) };

  let display = |ctx: &LocaleContext, key: &str| {
    let text = ctx
      .try_get_by_key(key.as_bytes())
      .unwrap_or_else(|| panic!("{}", glossa::Error::new_text_not_found(key)));
    println!("{key}: {text}")
  };

  {
    // set_env_lang("gsw_CH.UTF-8");
    //

    let ctx = new_ctx()
      .with_current_locale(Some(glossa_shared::lang_id::consts::lang_id_gsw()));
    // [("de", 26)]

    for key in ["yes", "no", "ok", "cancel"] {
      display(&ctx, key)
    }
  }
  // Output:
  //   yes: Ja
  //   no: Nein
  //   ok: OK
  //   cancel: Abbrechen

  {
    set_env_lang("zh_MO.UTF-8");
    // new_ctx();                           // current_locale =>  get_static_locale()
    let ctx = new_ctx().with_current_locale(None);

    log::debug!("\n---\n--- current locale => zh-MO");
    // [("zh-Hant", 43), ("zh-Latn-CN", 22)]

    for key in ["yes", "no", "ok", "cancel", "confirm"] {
      display(&ctx, key)
    }
  }
  // Output:
  //   yes: 是
  //   no: 否
  //   ok: 確定
  //   cancel: 取消
  //   confirm: Confirm
}
```

### 雙語

**場景1**：

在某些資源受限環境中，漢字可能無法正常顯示。
這時候，我們可以將本地化語言切換為漢語拼音。

由於 漢語-普通話 中存在 同音多義字，因此在只能用拼音不能用漢字的情況下，可能會產生歧義。
此時，就是“雙語功能”閃亮登場✨的時刻了！

> 我們需要手動實現 “雙語功能”。

---

```rust
#[ignore]
#[test]
// en-GB, zh-pinyin
fn test_bilingual() {
  use glossa_shared::lang_id::consts::{lang_id_en_gb, lang_id_zh_pinyin};

  let new_ctx = |id| {
    LocaleContext::default()
      .with_current_locale(Some(id))
      .with_all_locales(all_locales())
  };
  let zh_pinyin_ctx = new_ctx(lang_id_zh_pinyin());
  let en_gb_ctx = new_ctx(lang_id_en_gb());

  fn get_text<'a>(ctx: &LocaleContext, key: &str) -> Option<&'a str> {
    let key_bytes = key.as_bytes();
    let lookup = |language| match map(language, key_bytes) {
      "" => None,
      x => Some(x),
    };

    ctx
      .get_or_try_init_chain()?
      .iter()
      .map(|id| id.as_bytes())
      .find_map(lookup)
  }

  let get_cancel_text = |ctx| get_text(ctx, "cancel").unwrap_or_default();

  let zh_pinyin_text = get_cancel_text(&zh_pinyin_ctx);
  let en_gb_text = get_cancel_text(&en_gb_ctx);

  let text = match zh_pinyin_text == en_gb_text {
    true => zh_pinyin_text.into(),
    _ => glossa_shared::fmt_compact!("{en_gb_text}; {zh_pinyin_text}"),
  };

  assert_eq!(text, "Cancel; QuXiao")
}
```
