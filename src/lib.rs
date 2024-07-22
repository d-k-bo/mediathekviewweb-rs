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
        MediathekQueryBuilder::new(self).query(fields, query)
    }
    /// Query the current media database by parsing a query string using
    /// [MediathekViewWeb's advanced search syntax](https://github.com/mediathekview/mediathekviewweb/blob/master/README.md#erweiterte-suche).
    pub fn query_string(&self, query: &str, search_everywhere: bool) -> MediathekQueryBuilder<'_> {
        MediathekQueryBuilder {
            client: self,
            query: MediathekQuery::from_search_string(query, search_everywhere),
        }
    }
}

#[derive(Debug, Default, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
struct MediathekQuery {
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

impl MediathekQuery {
    fn from_search_string(s: &str, search_everywhere: bool) -> Self {
        let mut query = Self::default();

        for part in s.split_whitespace() {
            if let Some(channel) = part.strip_prefix('!') {
                query.queries.push(Query {
                    fields: vec![QueryField::Channel],
                    query: channel.replace(',', " "),
                })
            } else if let Some(topic) = part.strip_prefix('#') {
                query.queries.push(Query {
                    fields: vec![QueryField::Topic],
                    query: topic.replace(',', " "),
                })
            } else if let Some(title) = part.strip_prefix('+') {
                query.queries.push(Query {
                    fields: vec![QueryField::Title],
                    query: title.replace(',', " "),
                })
            } else if let Some(description) = part.strip_prefix('*') {
                query.queries.push(Query {
                    fields: vec![QueryField::Description],
                    query: description.replace(',', " "),
                })
            } else if let Some(duration_min) = part.strip_prefix('>').and_then(|s| s.parse().ok()) {
                query.duration_min = Some(duration_min)
            } else if let Some(duration_max) = part.strip_prefix('<').and_then(|s| s.parse().ok()) {
                query.duration_max = Some(duration_max)
            } else {
                let fields = if search_everywhere {
                    vec![
                        QueryField::Channel,
                        QueryField::Topic,
                        QueryField::Title,
                        QueryField::Description,
                    ]
                } else {
                    vec![QueryField::Topic, QueryField::Title]
                };
                query.queries.push(Query {
                    fields,
                    query: s.to_owned(),
                })
            }
        }

        query
    }
}

/// Request builder for the `/api/query` endpoint.
#[derive(Debug)]
pub struct MediathekQueryBuilder<'client> {
    client: &'client Mediathek,
    query: MediathekQuery,
}
impl<'client> MediathekQueryBuilder<'client> {
    fn new(client: &'client Mediathek) -> Self {
        Self {
            client,
            query: MediathekQuery::default(),
        }
    }
}
impl<'client> MediathekQueryBuilder<'client> {
    /// Add an additional search query.
    ///
    /// Multiple queries are combined using a logical `AND`.
    ///
    /// `fields` describes the fields in which should be searched for `query`.
    pub fn query(mut self, fields: impl Into<Vec<QueryField>>, query: impl Into<String>) -> Self {
        self.query.queries.push(Query {
            fields: fields.into(),
            query: query.into(),
        });
        self
    }
    /// Filter for a minimum duration.
    pub fn duration_min(mut self, duration_min: impl Into<Duration>) -> Self {
        self.query.duration_min = Some(duration_min.into().as_secs());
        self
    }
    /// Filter for a maximum duration.
    pub fn duration_max(mut self, duration_max: impl Into<Duration>) -> Self {
        self.query.duration_max = Some(duration_max.into().as_secs());
        self
    }
    /// Include media with a broadcasting date in the future.
    pub fn include_future(mut self, include_future: bool) -> Self {
        self.query.future = Some(include_future);
        self
    }
    /// Sort the results by a specific field.
    pub fn sort_by(mut self, sort_by: SortField) -> Self {
        self.query.sort_by = Some(sort_by);
        self
    }
    /// Set the sort order.
    pub fn sort_order(mut self, sort_order: SortOrder) -> Self {
        self.query.sort_order = Some(sort_order);
        self
    }
    /// Set the count of results to retrieve.
    ///
    /// Can be used for pagination.
    pub fn size(mut self, size: usize) -> Self {
        self.query.size = Some(size);
        self
    }
    /// Skip the specified count of items.
    ///
    /// Can be used for pagination.
    pub fn offset(mut self, offset: usize) -> Self {
        self.query.offset = Some(offset);
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
            .json(&self.query)
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

#[cfg(test)]
mod tests {
    use crate::{
        models::{Query, QueryField},
        Mediathek, MediathekQuery,
    };

    #[test]
    fn test_search_string() {
        assert_eq!(
            MediathekQuery::from_search_string("!ard", false),
            MediathekQuery {
                queries: vec![Query {
                    fields: vec![QueryField::Channel],
                    query: "ard".into()
                }],
                ..Default::default()
            }
        );
        assert_eq!(
            MediathekQuery::from_search_string("+gebärdensprache", false),
            MediathekQuery {
                queries: vec![Query {
                    fields: vec![QueryField::Title],
                    query: "gebärdensprache".into()
                }],
                ..Default::default()
            }
        );
        assert_eq!(
            MediathekQuery::from_search_string("*norwegen", false),
            MediathekQuery {
                queries: vec![Query {
                    fields: vec![QueryField::Description],
                    query: "norwegen".into()
                }],
                ..Default::default()
            }
        );
        assert_eq!(
            MediathekQuery::from_search_string("!ard #wetter", false),
            MediathekQuery {
                queries: vec![
                    Query {
                        fields: vec![QueryField::Channel],
                        query: "ard".into()
                    },
                    Query {
                        fields: vec![QueryField::Topic],
                        query: "wetter".into()
                    }
                ],
                ..Default::default()
            }
        );
        assert_eq!(
            MediathekQuery::from_search_string(">60", false),
            MediathekQuery {
                duration_min: Some(60),
                ..Default::default()
            }
        );
        assert_eq!(
            MediathekQuery::from_search_string("*diane,kruger", false),
            MediathekQuery {
                queries: vec![Query {
                    fields: vec![QueryField::Description],
                    query: "diane kruger".into()
                }],
                ..Default::default()
            }
        );
        assert_eq!(
            MediathekQuery::from_search_string("!ard !ndr #sturm,der,liebe #rote,rosen", false),
            MediathekQuery {
                queries: vec![
                    Query {
                        fields: vec![QueryField::Channel],
                        query: "ard".into()
                    },
                    Query {
                        fields: vec![QueryField::Channel],
                        query: "ndr".into()
                    },
                    Query {
                        fields: vec![QueryField::Topic],
                        query: "sturm der liebe".into()
                    },
                    Query {
                        fields: vec![QueryField::Topic],
                        query: "rote rosen".into()
                    }
                ],
                ..Default::default()
            }
        );
        assert_eq!(
            MediathekQuery::from_search_string("!ard !ndr #sturm,der,liebe #rote,rosen", false),
            MediathekQuery {
                queries: vec![
                    Query {
                        fields: vec![QueryField::Channel],
                        query: "ard".into()
                    },
                    Query {
                        fields: vec![QueryField::Channel],
                        query: "ndr".into()
                    },
                    Query {
                        fields: vec![QueryField::Topic],
                        query: "sturm der liebe".into()
                    },
                    Query {
                        fields: vec![QueryField::Topic],
                        query: "rote rosen".into()
                    }
                ],
                ..Default::default()
            }
        );

        assert_eq!(
            MediathekQuery::from_search_string("test", false),
            MediathekQuery {
                queries: vec![Query {
                    fields: vec![QueryField::Topic, QueryField::Title],
                    query: "test".into()
                },],
                ..Default::default()
            }
        );

        assert_eq!(
            MediathekQuery::from_search_string("test", true),
            MediathekQuery {
                queries: vec![Query {
                    fields: vec![
                        QueryField::Channel,
                        QueryField::Topic,
                        QueryField::Title,
                        QueryField::Description
                    ],
                    query: "test".into()
                },],
                ..Default::default()
            }
        );
    }

    #[tokio::test]
    async fn test_query() -> Result<(), Box<dyn std::error::Error>> {
        let mediathek = Mediathek::new(
            format!(
                "{} Test Suite ({})",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_REPOSITORY")
            )
            .parse()
            .unwrap(),
        )?;

        mediathek.query([QueryField::Topic], "tagesschau").await?;

        // livestreams return `""` as the duration
        mediathek.query([QueryField::Topic], "livestream").await?;

        Ok(())
    }
}
