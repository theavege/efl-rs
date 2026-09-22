# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `Table` / `TableExt`: grid `pack(col, row, colspan, rowspan)`, padding, homogeneous, `child_at`
- `Scroller` / `ScrollerExt` with `ScrollPolicy` (Auto/On/Off), bounce, and `region_show`
- `Image` / `ImageExt`: load a file, prescale, aspect-fixed, `object_size`

### Fixed
- Example `dialect`: redundant `format!` reference (clippy)

## [0.0.7] - 2026-09-17

### Added
- `WidgetExt::is_set()`, plus `is_set()` on `Timer` and `WidgetItem`, so empty wrappers can be detected without panicking
- Widget types for existing traits: `Calendar`, `Clock`, `ColorSelector`, `FileSelector`, `FileEntry`
- `InputExt<(i32, i32, i32, i32)>` for `ColorSelector`
- `Copy` / `Clone` / `Debug` / `PartialEq` / `Eq` / `Hash` on `Signal`, `Align`, `Cursor`, and `PanelOrient`
- Re-export of `EflError`, `EflResult`, and `CStringExt` from the crate root and prelude
- CI coverage for `alpha/**` branches, bindgen dependencies, and `cargo test --lib`
- `Timer::del()` stops a timer and frees its callback

### Fixed
- `SegmentControl::clear` no longer calls `elm_diskselector_clear` (wrong widget)
- `List` / `Menu` / `SegmentControl` `value()`, `length()`, and `clear()` no longer panic on empty widgets or missing selection
- `FileEntryExt` now reads `HOME` / `USERPROFILE` instead of the invalid `%HOMEPATH%` environment variable
- `Naviframe::promote` is a no-op on an empty wrapper instead of panicking
- README and widget docs: broken links (`elm_triggers.md`, `elm_ranges.md`, `make.ps1`) and copy-pasted examples
- `.gitignore` no longer starts with a stray markdown fence
- Tests compile again (`ProgressBarExt` typo, missing imports, `CStringExt` `?Sized` bound) and pass without a display
- Smart callbacks, list/menu item callbacks, and timer closures are freed when the EFL object is deleted (or when a timer returns `false`)

### Changed
- C-string conversions go through `CStringExt::expect_cstring` for consistent panic messages
- Windows setup docs now match the MSYS2 CI path
- `tooltip`, `cursor`, and `disabled` live on `WidgetExt` (any widget, including `Label`), not only `InputExt`

## [0.0.6] - 2026-09-17

### Added
- `EflError` / `EflResult` and `CStringExt` for safer string conversion
- Unit test suite for widget trait bounds, enums, and error helpers

## [0.0.5] - 2026-07-13

### Fixed
- Typo: `lenght` → `length` throughout the codebase

### Added
- `CONTRIBUTING.md`, `CHANGELOG.md`, and `Makefile`
- Documentation on core traits and widget types

[Unreleased]: https://github.com/theavege/efl-rs/compare/v0.0.7...HEAD
[0.0.7]: https://github.com/theavege/efl-rs/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/theavege/efl-rs/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/theavege/efl-rs/compare/v0.0.4...v0.0.5
