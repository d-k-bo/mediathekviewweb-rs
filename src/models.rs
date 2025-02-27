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
    pub const ALL: &'static [QueryField] = &[
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
    #[serde(deserialize_with = "empty_string_as_none")]
    pub description: Option<String>,
    pub timestamp: i64,
    #[serde(with = "optional_duration_secs")]
    pub duration: Option<Duration>,
    pub size: Option<usize>,
    pub url_website: String,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub url_subtitle: Option<String>,
    pub url_video: String,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub url_video_low: Option<String>,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub url_video_hd: Option<String>,
    #[serde(with = "timestamp", rename = "filmlisteTimestamp")]
    pub filmliste_timestamp: i64,
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
    #[serde(with = "timestamp")]
    pub filmliste_timestamp: i64,
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

mod optional_duration_secs {
    use std::time::Duration;

    use serde::{Deserializer, Serialize, Serializer};

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration
            .as_ref()
            .map(Duration::as_secs)
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OptionalDurationVisitor;

        impl serde::de::Visitor<'_> for OptionalDurationVisitor {
            type Value = Option<Duration>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "an integer or an empty string")
            }

            fn visit_u64<E: serde::de::Error>(self, n: u64) -> Result<Self::Value, E> {
                Ok(Some(Duration::from_secs(n)))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                if s.is_empty() {
                    Ok(None)
                } else {
                    Err(E::custom("string is not empty"))
                }
            }
        }

        deserializer.deserialize_any(OptionalDurationVisitor)
    }
}

mod timestamp {
    use serde::{Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimestampVisitor;

        impl serde::de::Visitor<'_> for TimestampVisitor {
            type Value = i64;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "an integer or string that can be parsed as an integer"
                )
            }

            fn visit_i64<E: serde::de::Error>(self, n: i64) -> Result<Self::Value, E> {
                Ok(n)
            }

            fn visit_u64<E: serde::de::Error>(self, n: u64) -> Result<Self::Value, E> {
                n.try_into().map_err(serde::de::Error::custom)
            }

            fn visit_str<E: serde::de::Error>(self, s: &str) -> Result<Self::Value, E> {
                s.parse().map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(TimestampVisitor)
    }
}

pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s))
    }
}
