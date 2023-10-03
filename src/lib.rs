//! A client library for interacting with the MediathekViewWeb API.
//!
//! # Example
//! ```rust
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> mediathekviewweb::Result<()> {
//! # #[allow(non_snake_case)]
//! # let USER_AGENT = format!(
//! #     "{} Examples ({})",
//! #     env!("CARGO_PKG_NAME"),
//! #     env!("CARGO_PKG_REPOSITORY")
//! # )
//! # .try_into()
//! # .unwrap();
//! let results = mediathekviewweb::Mediathek::new(USER_AGENT)?
//!     .query([mediathekviewweb::models::QueryField::Topic], "tagesschau")
//!     .query(
//!         [mediathekviewweb::models::QueryField::Title],
//!         "tagesschau 20.00 Uhr",
//!     )
//!     .duration_min(std::time::Duration::from_secs(10 * 60))
//!     .duration_max(std::time::Duration::from_secs(30 * 60))
//!     .include_future(false)
//!     .sort_by(mediathekviewweb::models::SortField::Timestamp)
//!     .sort_order(mediathekviewweb::models::SortOrder::Descending)
//!     .size(2)
//!     .offset(3)
//!     .await?;
//!
//! println!("{results:#?}");
//! # Ok(())
//! # }
//! ```
//! <details><summary>Results in something like</summary>
//!
//! ```ignore
#![doc = include_str!("../examples/tagesschau.ron")]
//! ```
//! </details>

use std::{
    future::{Future, IntoFuture},
    pin::Pin,
    time::Duration,
};

use reqwest::header::HeaderMap;
use serde::Serialize;

pub use crate::error::{Error, Result};
use crate::models::{ApiResult, Query, QueryField, QueryResult, SortField, SortOrder};

mod error;
pub mod models;

/// A client for a MediathekViewWeb server.
#[derive(Debug)]
pub struct Mediathek {
    base_url: String,
    http: reqwest::Client,
}
impl Mediathek {
    /// Create a new client for the official server hosted at <https://mediathekviewweb.de>.
    ///
    /// `user_agent` identifies your application so the server administrators
    /// can contact you in case of a problem.
    pub fn new(user_agent: reqwest::header::HeaderValue) -> crate::Result<Self> {
        Self::new_with_url("https://mediathekviewweb.de", user_agent)
    }

    /// Create a new client for a MediathekViewWeb server hosted at a specific
    /// URL.
    ///
    /// `user_agent` identifies your application so the server administrators
    /// can contact you in case of a problem.
    pub fn new_with_url(
        base_url: impl Into<String>,
        user_agent: reqwest::header::HeaderValue,
    ) -> crate::Result<Self> {
        let mut base_url: String = base_url.into();
        if base_url.ends_with('/') {
            base_url.truncate(base_url.len() - 1)
        }

        Ok(Self {
            base_url,
            http: reqwest::Client::builder()
                .default_headers({
                    let mut headers = HeaderMap::new();
                    headers.insert(reqwest::header::USER_AGENT, user_agent);
                    headers
                })
                .build()?,
        })
    }
}
impl Mediathek {
    /// Query the current media database.
    ///
    /// `fields` describes the fields in which should be searched for `query`.
    pub fn query(
        &self,
        fields: impl Into<Vec<QueryField>>,
        query: impl Into<String>,
    ) -> MediathekQueryBuilder<'_> {
        MediathekQueryBuilder::new(self, fields, query)
    }
}

/// Request builder for the `/api/query` endpoint.
#[derive(Debug, Serialize)]
pub struct MediathekQueryBuilder<'client> {
    #[serde(skip)]
    client: &'client Mediathek,
    queries: Vec<Query>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_min: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_max: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    future: Option<bool>,
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    sort_by: Option<SortField>,
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    sort_order: Option<SortOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
}
impl<'client> MediathekQueryBuilder<'client> {
    fn new(
        client: &'client Mediathek,
        fields: impl Into<Vec<QueryField>>,
        query: impl Into<String>,
    ) -> Self {
        MediathekQueryBuilder {
            client,
            queries: vec![Query {
                fields: fields.into(),
                query: query.into(),
            }],
            duration_min: None,
            duration_max: None,
            future: None,
            sort_by: None,
            sort_order: None,
            size: None,
            offset: None,
        }
    }
}
impl MediathekQueryBuilder<'_> {
    /// Add an additional search query.
    ///
    /// Multiple queries are combined using a logical `AND`.
    ///
    /// `fields` describes the fields in which should be searched for `query`.
    pub fn query(mut self, fields: impl Into<Vec<QueryField>>, query: impl Into<String>) -> Self {
        self.queries.push(Query {
            fields: fields.into(),
            query: query.into(),
        });
        self
    }
    /// Filter for a minimum duration.
    pub fn duration_min(mut self, duration_min: impl Into<Duration>) -> Self {
        self.duration_min = Some(duration_min.into().as_secs());
        self
    }
    /// Filter for a maximum duration.
    pub fn duration_max(mut self, duration_max: impl Into<Duration>) -> Self {
        self.duration_max = Some(duration_max.into().as_secs());
        self
    }
    /// Include media with a broadcasting date in the future.
    pub fn include_future(mut self, include_future: bool) -> Self {
        self.future = Some(include_future);
        self
    }
    /// Sort the results by a specific field.
    pub fn sort_by(mut self, sort_by: SortField) -> Self {
        self.sort_by = Some(sort_by);
        self
    }
    /// Set the sort order.
    pub fn sort_order(mut self, sort_order: SortOrder) -> Self {
        self.sort_order = Some(sort_order);
        self
    }
    /// Set the count of results to retrieve.
    ///
    /// Can be used for pagination.
    pub fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }
    /// Skip the specified count of items.
    ///
    /// Can be used for pagination.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}
impl MediathekQueryBuilder<'_> {
    /// Build and send the request to the server.
    ///
    /// This call can be usually omitted since this type implements
    /// [`IntoFuture`].
    pub async fn send(self) -> crate::Result<QueryResult> {
        self.client
            .http
            .post(format!(
                "{base_url}/api/query",
                base_url = self.client.base_url
            ))
            // https://github.com/mediathekview/mediathekviewweb/issues/145#issuecomment-555054562
            .header(reqwest::header::CONTENT_TYPE, "text/plain")
            .json(&self)
            .send()
            .await?
            .error_for_status()?
            .json::<ApiResult<QueryResult>>()
            .await?
            .into()
    }
}
impl<'client> IntoFuture for MediathekQueryBuilder<'client> {
    type Output = crate::Result<QueryResult>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send + 'client>>;

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
