# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.5] - 2026-06-02

### Added

- `TryFrom<UNative> for bool` and `TryFrom<INative> for bool`

## [0.2.4] - 2026-06-02

### Added

- `const` comparison, ordering, and bitwise methods on `UNative` and `INative`,
  for use in `const` contexts where the corresponding operators and trait
  methods are not available: `const_eq`, `const_cmp`, `const_lt`, `const_le`,
  `const_gt`, `const_ge`, `const_min`, `const_max`, `const_clamp`,
  `const_bitand`, `const_bitor`, `const_bitxor`, and `const_not`.

## [0.2.3] - 2026-06-01

### Added

- `const` conversion methods on `UNative` and `INative`. `UNative` gains
  conversions to and from the unsigned primitives and `INative` to and from the
  signed primitives.

## [0.2.2] - 2026-05-27

### Changed

- Enabled all optional features and the `docsrs` cfg when building
  documentation on docs.rs, so the optional `serde`, `num-traits`, and `rand`
  integrations are documented.

## [0.2.1] - 2026-05-27

### Added

- Optional `serde` feature implementing `Serialize`/`Deserialize` for `UNative`
  and `INative`.
- Optional `num-traits` feature implementing the `num-traits` integer traits for
  `UNative` and `INative`.
- Optional `rand` feature for sampling `UNative` and `INative`.
- `keywords` package metadata.

### Changed

- Documented that `UNative` and `INative` are at most 64 bits wide.

## [0.2.0] - 2026-05-22

Initial release.

### Added

- Platform-native integer types `UNative` (unsigned) and `INative` (signed),
  always the same width, chosen at compile time as the widest size with
  efficient hardware arithmetic on the target.
- The full inherent API delegating to the underlying primitive: constants, bit
  operations, and checked/overflowing/saturating/wrapping/strict/unchecked/
  unbounded arithmetic, byte conversions, and `from_str_radix`.
- Operator, `From`/`TryFrom`, `Sum`/`Product`, `Display`, and related trait
  implementations.
- Minimum supported Rust version of 1.95 (edition 2024), enforced in CI.

[Unreleased]: https://github.com/tczajka/unative/compare/v0.2.5...HEAD
[0.2.5]: https://github.com/tczajka/unative/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/tczajka/unative/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/tczajka/unative/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/tczajka/unative/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/tczajka/unative/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/tczajka/unative/releases/tag/v0.2.0
