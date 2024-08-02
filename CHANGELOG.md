## [1.5.2](https://github.com/hasezoey/new_string_template/compare/v1.5.1...v1.5.2) (2024-08-02)


### Style

* **template::DEFAULT_TEMPLATE:** add some more documentation on how the default template works ([eaae228](https://github.com/hasezoey/new_string_template/commit/eaae2288458e372d30fd786c341a6265f5b863ad))
* **template::new:** fix example using default template ([fabe605](https://github.com/hasezoey/new_string_template/commit/fabe60522b18560755c48481702efeaf38232607))


### Reverts

* Revert "fix(releaserc): try option "no_dirty"" ([75092e0](https://github.com/hasezoey/new_string_template/commit/75092e0009e26f282abc4ef79774907b1994d2bd))

## [1.5.1](https://github.com/hasezoey/new_string_template/compare/v1.5.0...v1.5.1) (2024-04-06)


### Fixes

* **releaserc:** try option "no_dirty" ([8a15c7e](https://github.com/hasezoey/new_string_template/commit/8a15c7eb7d76b71d9972888227198f8ff2ba5281))

## [1.5.0](https://github.com/hasezoey/new_string_template/compare/v1.4.0...v1.5.0) (2024-04-06)


### Features

* mention MSRV being 1.40 ([f99c846](https://github.com/hasezoey/new_string_template/commit/f99c846d593fb2b367480ed5db71c7d1dde33bd5))


### Style

* convert tabs to spaces ([9751443](https://github.com/hasezoey/new_string_template/commit/975144371ac91461330446f091e776f68270ef5b))


### Dependencies

* specify full semver ([e352325](https://github.com/hasezoey/new_string_template/commit/e35232557c0611836a218cd03848a1b613e68b8c))

## [1.4.0](https://github.com/hasezoey/new_string_template/compare/v1.3.1...v1.4.0) (2022-08-30)


### Features

* add full Eq to TemplateErrorKind ([5751b8f0](https://github.com/hasezoey/new_string_template/commit/5751b8f0b884a8d87a095867680057c9450d619a))
* make MatchEntry::new() consts ([9c9d5d2c](https://github.com/hasezoey/new_string_template/commit/9c9d5d2c61880b4ff74e8d41a4254a8516d3aa82))
* make TemplateError:kind() consts ([7b1f8b07](https://github.com/hasezoey/new_string_template/commit/7b1f8b07f2232a2dfcc0d3995e61a6215a633bad))


### Refactor

* **error.rs, template.rs:** :recycle: Nightly clippy controlled cleanup ([b999d8c](https://github.com/hasezoey/new_string_template/commit/b999d8c1753da54805280003bbeb796cf13b0e17))
* **templates.rs:** change some match statments into if let statments ([5d85b7e](https://github.com/hasezoey/new_string_template/commit/5d85b7e7eb456cc61190d8e554d755d7befcd560))


### Style

* fix a typo "an" -> "a" ([fb1db5a](https://github.com/hasezoey/new_string_template/commit/fb1db5a3cee8f0d9cc250327b4207ff8cd5c8414))

### [1.3.1](https://github.com/hasezoey/new_string_template/compare/v1.3.0...v1.3.1) (2022-05-26)


### Fixes

* **template:** change default regex to not be greedy ([b67be7c](https://github.com/hasezoey/new_string_template/commit/b67be7c9833c0191be461d82c9ab1ed54e9e2bbe)), closes [#4](https://github.com/hasezoey/new_string_template/issues/4)


### Refactor

* **template::MatchEntry:** rename fields to be more intuitive ([d3fe11c](https://github.com/hasezoey/new_string_template/commit/d3fe11cad04b67cb2133f6127777c5e7f826ed9c))

## [1.3.0](https://github.com/hasezoey/new_string_template/compare/v1.2.0...v1.3.0) (2022-03-05)


### Features

* **template:** derive all possible traits where possible ([3557591](https://github.com/hasezoey/new_string_template/commit/35575914e7c275aaf53b2183e4acb47950488208)), closes [#3](https://github.com/hasezoey/new_string_template/issues/3)


### Style

* add some missing rustdoc ([d0d9f39](https://github.com/hasezoey/new_string_template/commit/d0d9f3967b60e6c7a8538de5716b94540ab54239))

## [1.2.0](https://github.com/hasezoey/new_string_template/compare/v1.1.0...v1.2.0) (2022-01-24)


### Features

* **error::TemplateError:** implement "Display" and "Error" ([d651d7d](https://github.com/hasezoey/new_string_template/commit/d651d7da559ff0469482b4a4e483c0f8bed0882e))
* **template::render_nofail*:** replace HashMap value with generic to match other functions ([3932c9a](https://github.com/hasezoey/new_string_template/commit/3932c9a27f31a90aca3a745b493d68b542b5ab7b))
* **template::Template:** add function "render_nofail_string" ([0065ac2](https://github.com/hasezoey/new_string_template/commit/0065ac25046df52c0c91b0c6ae3a2a202e4bc4c6))

## [1.1.0](https://github.com/hasezoey/new_string_template/compare/v1.0.0...v1.1.0) (2021-11-12)


### Features

* **template:** add "_string" functions to allow HashMap "String" keys ([d992861](https://github.com/hasezoey/new_string_template/commit/d992861076eeda54b952e002de53c3fe2087ee44))
* **template:** change HashMap data type to be "AsRef<str>" to allow using "String" as data type ([d8eb8a2](https://github.com/hasezoey/new_string_template/commit/d8eb8a2035ffc6039696448d9457aff84fb0c277))


### Style

* **template:** apply clippy suggestions ([8898bfe](https://github.com/hasezoey/new_string_template/commit/8898bfe0d06dc7ae978c5e8bf16dfe5252972bf8))

## 1.0.0 (2021-05-28)


### Features

* add "lib.rs" file again ([70962fa](https://github.com/hasezoey/new_string_template/commit/70962faf129531968f999d7fe75a4a2a5d28ff35))
* rename file "lib.rs" to "template.rs" ([aba4e79](https://github.com/hasezoey/new_string_template/commit/aba4e7940df18909cfd67f36909240c2254f63bb))
* **errors:** add errors ([4cdbc97](https://github.com/hasezoey/new_string_template/commit/4cdbc9756648adea771cd6b8888c0790b00ebbe4))
* **lib:** add initial code ([0d98bcb](https://github.com/hasezoey/new_string_template/commit/0d98bcb58f9b7a49a1cb8c334267b68b8aa21e24))
* **lib::Template:** implement render functions ([2178d13](https://github.com/hasezoey/new_string_template/commit/2178d13e9c581cd1a83a1c69f1de70d1cc45bfae))


### Fixes

* **template::DEFAULT_TEMPLATE:** make it public ([76774a1](https://github.com/hasezoey/new_string_template/commit/76774a192d1b7c72efb45750e9045d0da08157a1))


### Style

* add missing "rustfmt.toml" ([d0991c4](https://github.com/hasezoey/new_string_template/commit/d0991c490a3aeee10ad299f29d4bccca54b31a2d))
* **lib:** add index documentation ([754e991](https://github.com/hasezoey/new_string_template/commit/754e991534229b8dd2ca6bf8f1f1178549337f27))


### Dependencies

* **lazy_static:** be more lazy on the version ([4211f63](https://github.com/hasezoey/new_string_template/commit/4211f635658b2bfadc629218436bc5e14d002990))
