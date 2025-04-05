# glossa-shared

[![glossa-shared.crate](https://img.shields.io/crates/v/glossa-shared.svg?logo=rust&logoColor=lightsalmon&label=glossa-shared)](https://crates.io/crates/glossa-shared)

[![Documentation](https://docs.rs/glossa-shared/badge.svg)](https://docs.rs/glossa-shared)
[![Apache-2 licensed](https://img.shields.io/crates/l/glossa-shared.svg?logo=apache)](./License)

<details open>
<summary>
<img alt="Language/語言" src="./svg/language.svg"/>
</summary>

- [繁體中文](Readme-zh-Hant.md)
- [English](Readme.md)
- [簡體中文](Readme-zh.md)

</details>

<!--  -->
<details open>
<summary>
<img alt="目錄" src="./svg/toc/目錄.svg"/>
</summary>

- [設計目標](#設計目標)
- [features](#features)
- [資料型別](#資料型別)
  - [PHF](#phf)
  - [Map](#map)
- [解碼 bincode](#解碼-bincode)
  - [`decode::slice`](#decodeslice)
  - [`decode::file`](#decodefile)

</details>

## 設計目標

- 為 glossa-codegen 生成的程式碼提供資料型別
- 解碼（反序列化） glossa-codegen 生成的 bincode 檔案

## features

Q: 我需要啟用哪些 features?

A:

這取決於 glossa-codegen 生成的程式碼的型別。

| Codegen 方法         | Required glossa-shared Feature   |
| -------------------- | -------------------------------- |
| `.output_phf()`      | `["phf"]`                        |
| `.output_bincode()`  | `["std", "decode"]`              |
| `.output_match_fn()` | *您無需匯入 glossa-shared crate* |

## 資料型別

### PHF

- PhfL10nOrderedMap
  - `.output_phf()` 生成的函式的返回型別
- PhfL10nAllInOneMap
  - `.output_phf_all_in_one()` 生成的函式的返回型別
- PhfTupleKey
  - 元組結構體 `(MapName, Key)`
  - `.output_phf()` 生成的 map 的 key 型別
- PhfTripleKey
  - 元組結構體 `(Language, MapName, Key)`
  - `.output_phf_all_in_one()` 生成的 map 的 key 型別

### Map

> NonDSL: Regular|Highlight|RegularAndHighlight

- L10nFlattenMap
  - `<(MapName, Key), Value>`
  - `output_bincode(NonDSL)` => `{language}`.bincode => 解碼 => L10nFlattenMap
- L10nMaps
  - `<Language, L10nFlattenMap>`
  - `output_bincode_all_in_one(NonDSL)` => all.bincode => 解碼 => L10nMaps
- L10nDSLMap
  - `<MapName, Resolver>`
  - `output_bincode(DSL)` => `{language}`.bincode => 解碼 => L10nDSLMap
- DSLMaps
  - `<Language, L10nDSLMap>`
  - `output_bincode_all_in_one(DSL)` => all.bincode => 解碼 => DSLMaps

## 解碼 bincode

- `decode::slice` 不需要啟用 "std" feature
- `decode::file` 需要 "std"

### `decode::slice`

- decode_slice()
  - decode_single_data_to_map()
  - `output_bincode(NonDSL)` => `{language}`.bincode => 讀取 => `&[u8]` => `decode_single_data_to_map(&[u8])` => `<(MapName, Key), Value>`
  - decode_to_maps()
    - `output_bincode_all_in_one(NonDSL)` => all.bincode => 讀取 => `&[u8]` => `decode_to_maps(&[u8])` => `<Language, L10nFlattenMap>`
  - decode_single_data_to_dsl_map()
    - `output_bincode(DSL)` => `{language}`.bincode => 讀取 => `&[u8]` => `decode_single_data_to_dsl_map(&[u8])` => `<MapName, Resolver>`
  - decode_to_dsl_maps
    - `output_bincode_all_in_one(DSL)` => all.bincode => 讀取 => `&[u8]` => `decode_to_dsl_maps(&[u8])` => `<Language, <MapName, Resolver>>`

### `decode::file`

- decode_file()
  - decode_single_file_to_flatten_map()
    - `output_bincode(NonDSL)` => `{language}`.bincode => `decode_single_file_to_flatten_map({language}.bincode)` => `<(MapName, Key), Value>`
  - decode_file_to_maps()
    - `output_bincode_all_in_one(NonDSL)` => all.bincode => `decode_file_to_maps("all.bincode")` => `<Language, L10nFlattenMap>`
  - decode_single_file_to_dsl_map()
    - `output_bincode(DSL)` => `{language}`.bincode => `decode_single_file_to_dsl_map({language}.bincode)` => `<MapName, Resolver>`
  - decode_file_to_dsl_maps()
    - `output_bincode_all_in_one(DSL)` => all.bincode => `decode_file_to_dsl_maps("all.bincode")` => `<Language, <MapName, Resolver>>`
