use crate::Kosmos;

pub mod apod;

pub enum NasaAuth {
    Demo(String),
    ApiKey(String),
}

impl Default for NasaAuth {
    fn default() -> Self {
        Self::Demo(String::from("DEMO_KEY"))
    }
}

pub struct NasaClient<'k> {
    kosmos: &'k Kosmos,
    api_key: String,
}

impl<'k> NasaClient<'k> {
    pub(crate) fn new(kosmos: &'k Kosmos) -> Self {
        Self {
            kosmos,
            api_key: std::env::var("NASA_API_KEY").unwrap_or(String::from("DEMO_KEY")),
        }
    }

    pub fn apod(&self) -> apod::ApodHandler {
        apod::ApodHandler::new(self)
    }
}
