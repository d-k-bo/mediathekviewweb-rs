# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2026-02-07

### Changed

- BREAKING: Update to `reqwest` 0.13

## [0.4.1] - 2025-02-21

### Fixed

- Parse min and max duration as minutes. They were incorrectly intepreted as seconds before ([#2](https://github.com/d-k-bo/mediathekviewweb-rs/issues/2))

## [0.4.0] - 2024-07-22

### Changed

- BREAKING: change `duration` from `Duration` to `Option<Duration>` because some entries (e.g. livestreams) don't have a duration 

## [0.3.1] - 2024-05-23

### Added

- Support [MediathekViewWeb's advanced search syntax](https://github.com/mediathekview/mediathekviewweb/blob/master/README.md#erweiterte-suche)

## [0.3.0] - 2024-03-27

### Changed

- BREAKING: Update `reqwest` to v0.12.x which is based on `hyper`, `http` and `http-body` v1

## [0.2.0] - 2023-10-14

### Changed

- BREAKING: use `i64` for timestamps
- Make timestamp parsing more resilient by accepting both strings and integers
- BREAKING: Deserialize maybe-empty strings as `Option<String>`

## [0.1.2] - 2023-10-06

### Fixed

- Replace `Error::cause` with `Error::source`
- Change `QueryInfo` deserialization to adapt to upstream changes

## [0.1.1] - 2023-10-04

### Fixed

- Make QueryResult fields public

## [0.1.0] - 2023-10-03

### Added

- Initial release

[Unreleased]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/d-k-bo/mediathekviewweb-rs/releases/tag/v0.1.0
