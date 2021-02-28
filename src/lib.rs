//! # Kosmos: A client library for apis of the cosmos.
//! Kosmos is a client library for a whole host of apis dealing with the
//! universe. This includes apis for our earth e.g. Geomagnetism data, sea
//! level data, as well as apis dealing with space. Specific services are exposed under
//! their respective owning organization, e.g. Nasa's Astronomy Photo of the
//! Day requires you to build a `NasaClient`. This is because most organizations
//! share an auth strategy over all their services.
//!
//! The apis are behind feature flags but all are included by default. If you
//! want to cherry-pick apis, use `deafult-features = false`.
//!
//! Organizations (and feature flags) with APIs we support (so far)
//! - [`nasa`] NASA Open APIs
//!
//! [`nasa`]: ./nasa/struct.NasaClient.html
//!
//! #### Lookup an Asteroid
//! ```no_run
//! # use kosmos::Kosmos;
//! # async fn run() -> kosmos::Result<()> {
//! // Get asteroid 3542519 from the NASA NeoWS
//! let asteroid = Kosmos::new().nasa().neo().lookup(3542519).await;
//! # Ok(())
//! # }
//! ```

pub mod error;
#[cfg(feature = "nasa")]
pub mod nasa;

pub use self::error::Error;
use serde::Serialize;
use snafu::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct Kosmos {
    client: reqwest::Client,
}

impl Kosmos {
    pub fn new() -> Self {
        Self {
            client: reqwest::ClientBuilder::new()
                .user_agent("kosmos")
                .build()
                .unwrap(),
        }
    }

    #[cfg(feature = "nasa")]
    pub fn nasa(&self) -> nasa::NasaClient {
        nasa::NasaClient::new(self)
    }

    pub(crate) async fn get<R, A, P>(&self, url: A, parameters: Option<&P>) -> Result<R>
    where
        A: AsRef<str>,
        P: Serialize,
        R: serde::de::DeserializeOwned,
    {
        let url = url::Url::parse(url.as_ref()).context(error::Url)?;
        let response = self._get(url, parameters).await?;

        let text = response.text().await.context(crate::error::Http)?;

        let mut de = serde_json::Deserializer::from_str(&text);
        serde_path_to_error::deserialize(&mut de).context(crate::error::Json)
    }

    pub(crate) async fn _get<P: Serialize>(
        &self,
        url: impl reqwest::IntoUrl,
        parameters: Option<&P>,
    ) -> Result<reqwest::Response> {
        let mut request = self.client.get(url);

        if let Some(parameters) = parameters {
            request = request.query(parameters);
        }

        self.execute(request).await
    }

    pub(crate) async fn execute(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response> {
        request.send().await.context(error::Http)
    }
}
