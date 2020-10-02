use crate::nasa::NasaClient;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

/// ## apod
/// Astronomy Photo of the Day -  an optional copyright is returned if the
/// image is not in the public domain.
///
/// # Examples
///
/// ```
/// # use kosmos::Kosmos;
/// # use chrono::{Datelike, NaiveDate};
/// # async fn get_apod() {
///     let apod = Kosmos::new()
///         .nasa()
///         .apod()
///         .builder()
///         .hd(true)
///         .date(NaiveDate::from_ymd(2020, 10, 2))
///         .get()
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

    pub fn builder(&self) -> ApodRequestBuilder {
        ApodRequestBuilder::new(self)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApodResponse {
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
}

impl<'k> ApodRequestBuilder<'k> {
    pub(crate) fn new(handler: &'k ApodHandler) -> Self {
        Self {
            handler,
            date: None,
            hd: None,
        }
    }

    pub fn date(mut self, date: impl chrono::Datelike) -> Self {
        self.date = Some(format!("{}-{}-{}", date.year(), date.month(), date.day()));
        self
    }

    pub fn hd(mut self, hd: bool) -> Self {
        self.hd = Some(hd);
        self
    }

    pub async fn get(self) -> Result<ApodResponse, reqwest::Error> {
        let mut req = self
            .handler
            .nasa
            .kosmos
            .client
            .get("https://api.nasa.gov/planetary/apod");

        req = req.query(&self);

        req.send().await?.json::<ApodResponse>().await
    }
}
