#[cfg(feature = "nasa")]
pub mod nasa;

#[derive(Debug, Clone)]
pub struct Kosmos {
    client: reqwest::Client,
}

/// # APIs available in Kosmos
impl Kosmos {
    pub fn nasa(&self) -> nasa::NasaClient {
        nasa::NasaClient::new(self)
    }
}
