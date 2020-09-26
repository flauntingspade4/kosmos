use crate::nasa::NasaClient;

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

#[derive(serde::Serialize)]
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

    pub async fn get(self) -> Result<reqwest::Response, reqwest::Error> {
        let mut req = self
            .handler
            .nasa
            .kosmos
            .client
            .get("https://api.nasa.gov/planetary/apod");

        req = req.query(&self);

        req.send().await
    }
}
