# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- BREAKING: use `i64`for timestamps
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

[Unreleased]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.1.2...HEAD
[0.1.2]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/d-k-bo/mediathekviewweb-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/d-k-bo/mediathekviewweb-rs/releases/tag/v0.1.0
