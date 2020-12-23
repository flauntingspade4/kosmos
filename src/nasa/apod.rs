use crate::nasa::NasaClient;
use crate::Result;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

/// Astronomy Photo of the Day -  an optional copyright is returned if the
/// image is not in the public domain.
///
/// # Examples
///
/// ```
/// # use kosmos::Kosmos;
/// # use chrono::{Datelike, NaiveDate};
/// # async fn get_apod() {
///     // Get the photo information from October 2nd, 2020
///     let apod = Kosmos::new()
///         .nasa()
///         .apod()
///         .get()
///         .hd(true)
///         .date(NaiveDate::from_ymd(2020, 10, 2))
///         .send()
///         .await
///         .unwrap();
///     assert_eq!(apod.date.day(), 2);
/// # }
/// ```
///
pub struct ApodHandler<'k> {
    nasa: &'k NasaClient<'k>,
}

impl<'k> ApodHandler<'k> {
    pub(crate) fn new(nasa: &'k NasaClient) -> Self {
        Self { nasa }
    }

    pub fn get(&self) -> ApodRequestBuilder {
        ApodRequestBuilder::new(self)
    }
}

#[derive(Deserialize)]
pub struct Apod {
    pub copyright: Option<String>,
    pub date: NaiveDate,
    pub explanation: String,
    pub hdurl: Url,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: Url,
}

#[derive(Serialize)]
pub struct ApodRequestBuilder<'k> {
    #[serde(skip)]
    handler: &'k ApodHandler<'k>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hd: Option<bool>,
    api_key: &'k str,
}

impl<'k> ApodRequestBuilder<'k> {
    pub(crate) fn new(handler: &'k ApodHandler) -> Self {
        Self {
            handler,
            date: None,
            hd: None,
            api_key: &handler.nasa.api_key,
        }
    }

    pub fn date(mut self, date: impl chrono::Datelike) -> Self {
        self.date = Some(self.handler.nasa.format_date(date));
        self
    }

    pub fn hd(mut self, hd: bool) -> Self {
        self.hd = Some(hd);
        self
    }

    pub async fn send(self) -> Result<Apod> {
        self.handler
            .nasa
            .kosmos
            .get("https://api.nasa.gov/planetary/apod", Some(&self))
            .await
    }
}
