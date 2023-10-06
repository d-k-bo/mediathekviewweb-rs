//! Data types that are returned by the API or used as request parameters.

use std::{
    fmt::{Debug, Display},
    time::Duration,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Query {
    pub fields: Vec<QueryField>,
    pub query: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum QueryField {
    Topic,
    Title,
    Description,
    Channel,
}
impl QueryField {
    pub const ALL: &[QueryField] = &[
        QueryField::Topic,
        QueryField::Title,
        QueryField::Description,
        QueryField::Channel,
    ];
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    Channel,
    Timestamp,
    Duration,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub channel: String,
    pub topic: String,
    pub title: String,
    pub description: String,
    pub timestamp: u64,
    #[serde(with = "duration_secs")]
    pub duration: Duration,
    pub size: Option<usize>,
    pub url_website: String,
    pub url_subtitle: String,
    pub url_video: String,
    pub url_video_low: String,
    pub url_video_hd: String,
    #[serde(with = "tostring_fromstr", rename = "filmlisteTimestamp")]
    pub filmliste_timestamp: u64,
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    pub query_info: QueryInfo,
    pub results: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryInfo {
    pub filmliste_timestamp: u64,
    pub result_count: usize,
    #[serde(with = "duration_millisecs")]
    pub search_engine_time: Duration,
    pub total_results: u64,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ApiResult<T> {
    err: Option<ApiError>,
    result: Option<T>,
}
impl<T> From<ApiResult<T>> for crate::Result<T> {
    fn from(result: ApiResult<T>) -> crate::Result<T> {
        match result {
            ApiResult { err: Some(e), .. } => Err(crate::Error::Response(e)),
            ApiResult {
                err: None,
                result: Some(result),
            } => Ok(result),
            ApiResult {
                err: None,
                result: None,
            } => Err(crate::Error::EmptyResponse),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ApiError(pub Box<[String]>);
impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}
impl std::error::Error for ApiError {}

mod duration_millisecs {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{:.2}", duration.as_secs_f32()).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        <&str>::deserialize(deserializer).and_then(|s| {
            s.parse::<f32>()
                .map(Duration::from_secs_f32)
                .map_err(serde::de::Error::custom)
        })
    }
}

mod duration_secs {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        <u64>::deserialize(deserializer).map(Duration::from_secs)
    }
}

mod tostring_fromstr {
    use std::{fmt::Display, str::FromStr};

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: ToString,
        S: Serializer,
    {
        value.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        <&str>::deserialize(deserializer).and_then(|s| s.parse().map_err(serde::de::Error::custom))
    }
}
