# mediathekviewweb-rs

[![Build Status](https://github.com/d-k-bo/mediathekviewweb-rs/workflows/CI/badge.svg)](https://github.com/d-k-bo/mediathekviewweb-rs/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/mediathekviewweb)](https://lib.rs/crates/mediathekviewweb)
[![Documentation](https://img.shields.io/docsrs/mediathekviewweb)](https://docs.rs/mediathekviewweb)
[![License: MIT](https://img.shields.io/crates/l/mediathekviewweb)](LICENSE)

<!-- cargo-rdme start -->

A client library for interacting with the MediathekViewWeb API.

## Example
```rust
let results = mediathekviewweb::Mediathek::new(USER_AGENT)?
    .query([mediathekviewweb::models::QueryField::Topic], "tagesschau")
    .query(
        [mediathekviewweb::models::QueryField::Title],
        "tagesschau 20.00 Uhr",
    )
    .duration_min(std::time::Duration::from_secs(10 * 60))
    .duration_max(std::time::Duration::from_secs(30 * 60))
    .include_future(false)
    .sort_by(mediathekviewweb::models::SortField::Timestamp)
    .sort_order(mediathekviewweb::models::SortOrder::Descending)
    .size(2)
    .offset(3)
    .await?;

println!("{results:#?}");
```
<details><summary>Results in something like</summary>

```rust
```
</details>

<!-- cargo-rdme end -->

## License

This project is licensed under the MIT License.

See [LICENSE](LICENSE) for more information.
