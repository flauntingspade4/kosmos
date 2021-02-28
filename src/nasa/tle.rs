use crate::nasa::NasaClient;

use reqwest::Result;
use serde::{Deserialize, Serialize};

/// Two line element data for earth-orbiting objects at a given point in time
///
/// # Examples
///
/// ```
/// # use kosmos::Kosmos;
/// # async fn get_tle() {
///     // Get the name of the satellite with id 28931
///     let tle = Kosmos::new()
///         .nasa()
///         .tle()
///         .get()
///         .lookup_id("28931")
///         .await
///         .unwrap();
///     assert_eq!(tle.name, "ALOS (DAICHI)".to_string());
/// # }
/// ```
pub struct TLEHandler<'k> {
    nasa: &'k NasaClient<'k>,
}

impl<'k> TLEHandler<'k> {
    pub(crate) fn new(nasa: &'k NasaClient) -> Self {
        Self { nasa }
    }

    pub fn get(&'k self) -> TLERequestBuilder<'k> {
        TLERequestBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct TLERequestBuilder<'k> {
    #[serde(skip)]
    handler: &'k TLEHandler<'k>,
}

impl<'k> TLERequestBuilder<'k> {
    pub(crate) fn new(handler: &'k TLEHandler) -> Self {
        Self { handler }
    }

    pub async fn lookup_name(&self, name: &str) -> Result<TLESatelliteName> {
        let url = format!("https://tle.ivanstanojevic.me/api/tle?search={}", name);

        let request = self.handler.nasa.kosmos.client.get(&url).send().await?;

        request.json().await
    }

    pub async fn lookup_id(&self, id: &str) -> Result<Satellite> {
        let url = format!("https://tle.ivanstanojevic.me/api/tle/{}", id);

        let request = self.handler.nasa.kosmos.client.get(&url).send().await?;

        request.json().await
    }
}

#[derive(Deserialize)]
pub struct TLESatelliteName {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub result_type: String,
    #[serde(rename = "totalItems")]
    pub total_items: usize,
    pub member: Vec<Satellite>,
}

#[derive(Deserialize)]
pub struct Satellite {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub result_type: String,
    #[serde(rename = "satelliteId")]
    pub satellite_id: usize,
    pub name: String,
    pub date: String,
    pub line1: String,
    pub line2: String,
}
