//! # Kosmos: A client library for apis of the cosmos.
//! Kosmos is a client library for a whole host of apis dealing with the
//! universe. This includes apis for our earth e.g. Geomagnetism data, sea
//! level data, as well as apis dealing with space. Specific services are exposed under
//! their respective owning organization, e.g. Nasa's Astronomy Photo of the
//! Day requires you to build a Nasa client. See examples below.
//!
//! The apis are behind feature flags but all are included by default. If you
//! want to cherry-pick apis, use `deafult-features = false`.
//!
//! Organizations with APIs we support (so far)
//! - [`nasa`] NASA Open APIs
//!
//! [`nasa`]: ./nasa/struct.NasaClient.html

#[cfg(feature = "nasa")]
pub mod nasa;

#[derive(Debug, Clone)]
pub struct Kosmos {
    client: reqwest::Client,
}

impl Kosmos {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    #[cfg(feature = "nasa")]
    pub fn nasa(&self) -> nasa::NasaClient {
        nasa::NasaClient::new(self)
    }
}
