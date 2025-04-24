# glossa-shared

[![glossa-shared.crate](https://img.shields.io/crates/v/glossa-shared.svg?logo=rust&logoColor=lightsalmon&label=glossa-shared)](https://crates.io/crates/glossa-shared)

[![Documentation](https://docs.rs/glossa-shared/badge.svg)](https://docs.rs/glossa-shared)
[![Apache-2 licensed](https://img.shields.io/crates/l/glossa-shared.svg?logo=apache)](../License)

<details open>
<summary>
<img alt="Language/语言" src="./svg/language.svg"/>
</summary>

- [zh-Hant: 繁體中文](Readme-zh-Hant.md)
- [en: English](Readme.md)
- [zh: 简体中文](Readme-zh.md)

</details>

<!--  -->
<details open>
<summary>
<img alt="目录" src="./svg/toc/目录.svg"/>
</summary>

- [设计目标](#设计目标)
- [features](#features)
- [数据类型](#数据类型)
  - [PHF](#phf)
  - [Map](#map)
- [解码 bincode](#解码-bincode)
  - [`decode::slice`](#decodeslice)
  - [`decode::file`](#decodefile)

</details>

## 设计目标

- 为 glossa-codegen 生成的代码提供数据类型
- 解码（反序列化） glossa-codegen 生成的 bincode 文件

## features

Q: 我需要启用哪些 features?

A:

这取决于 glossa-codegen 生成的代码的类型。

| Codegen 方法         | Required glossa-shared Feature   |
| -------------------- | -------------------------------- |
| `.output_phf()`      | `["phf"]`                        |
| `.output_bincode()`  | `["std", "decode"]`              |
| `.output_match_fn()` | *您无需导入 glossa-shared crate* |

## 数据类型

### PHF

- PhfL10nOrderedMap
  - `.output_phf()` 生成的函数的返回类型
- PhfL10nAllInOneMap
  - `.output_phf_all_in_one()` 生成的函数的返回类型
- PhfTupleKey
  - 元组结构体 `(MapName, Key)`
  - `.output_phf()` 生成的 map 的 key 类型
- PhfTripleKey
  - 元组结构体 `(Language, MapName, Key)`
  - `.output_phf_all_in_one()` 生成的 map 的 key 类型

### Map

> NonDSL: Regular|Highlight|RegularAndHighlight

- L10nFlattenMap
  - `<(MapName, Key), Value>`
  - `output_bincode(NonDSL)` => `{language}`.bincode => 解码 => L10nFlattenMap
- L10nMaps
  - `<Language, L10nFlattenMap>`
  - `output_bincode_all_in_one(NonDSL)` => all.bincode => 解码 => L10nMaps
- L10nDSLMap
  - `<MapName, Resolver>`
  - `output_bincode(DSL)` => `{language}`.bincode => 解码 => L10nDSLMap
- DSLMaps
  - `<Language, L10nDSLMap>`
  - `output_bincode_all_in_one(DSL)` => all.bincode => 解码 => DSLMaps

## 解码 bincode

- `decode::slice` 不需要启用 "std" feature
- `decode::file` 需要 "std"

### `decode::slice`

- decode_slice()
  - decode_single_data_to_map()
  - `output_bincode(NonDSL)` => `{language}`.bincode => 读取 => `&[u8]` => `decode_single_data_to_map(&[u8])` => `<(MapName, Key), Value>`
  - decode_to_maps()
    - `output_bincode_all_in_one(NonDSL)` => all.bincode => 读取 => `&[u8]` => `decode_to_maps(&[u8])` => `<Language, L10nFlattenMap>`
  - decode_single_data_to_dsl_map()
    - `output_bincode(DSL)` => `{language}`.bincode => 读取 => `&[u8]` => `decode_single_data_to_dsl_map(&[u8])` => `<MapName, Resolver>`
  - decode_to_dsl_maps
    - `output_bincode_all_in_one(DSL)` => all.bincode => 读取 => `&[u8]` => `decode_to_dsl_maps(&[u8])` => `<Language, <MapName, Resolver>>`

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
