use crate::nasa::NasaClient;
use crate::Result;
use chrono::{serde::ts_milliseconds, DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize)]
pub struct EstimatedKilometers {
    pub estimated_diameter_min: f64,
    pub estimated_diameter_max: f64,
}

#[derive(Debug, Deserialize)]
pub struct EstimatedMeters {
    pub estimated_diameter_min: f64,
    pub estimated_diameter_max: f64,
}

#[derive(Debug, Deserialize)]
pub struct EstimatedMiles {
    pub estimated_diameter_min: f64,
    pub estimated_diameter_max: f64,
}

#[derive(Debug, Deserialize)]
pub struct EstimatedFeet {
    pub estimated_diameter_min: f64,
    pub estimated_diameter_max: f64,
}

#[derive(Debug, Deserialize)]
pub struct EstimatedDiameter {
    pub kilometers: EstimatedKilometers,
    pub meters: EstimatedMeters,
    pub miles: EstimatedMiles,
    pub feet: EstimatedFeet,
}

#[derive(Debug, Deserialize)]
pub enum OrbitingBody {
    Merc,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

#[derive(Debug, Deserialize)]
pub struct MissDistance {
    pub astronomical: f64,
    pub lunar: f64,
    pub kilometers: f64,
    pub miles: f64,
}

#[derive(Debug, Deserialize)]
pub struct RelativeVelocity {
    pub kilometers_per_second: f64,
    pub kilometers_per_hour: f64,
    pub miles_per_hour: f64,
}

#[derive(Debug, Deserialize)]
pub struct CloseApproachData {
    pub close_approach_date: NaiveDate,
    pub close_approach_date_full: String,
    #[serde(with = "ts_milliseconds")]
    pub epoch_date_close_approach: DateTime<Utc>,
    pub relative_velocity: RelativeVelocity,
    pub miss_distance: MissDistance,
    pub orbiting_body: OrbitingBody,
}

#[derive(Debug, Deserialize)]
pub struct OrbitClass {
    pub orbit_class_type: String,
    pub orbit_class_description: String,
    pub orbit_class_range: String,
}

#[derive(Debug, Deserialize)]
pub struct OrbitalData {
    pub orbit_id: u64,
    pub orbit_determination_date: String,
    pub first_observation_date: NaiveDate,
    pub last_observation_date: NaiveDate,
    pub data_arc_in_days: u64,
    pub observations_used: u64,
    pub orbit_uncertainty: f64, // Guessing, only examples I found were "0"
    pub minimum_orbit_intersection: f64,
    pub jupiter_tisserand_invariant: f64,
    pub epoch_osculation: f64,
    pub eccentricity: f64,
    pub semi_major_axis: f64,
    pub inclination: f64,
    pub ascending_node_longitude: f64,
    pub orbital_period: f64,
    pub perihelion_distance: f64,
    pub perihelion_argument: f64,
    pub aphelion_distance: f64,
    pub perihelion_time: f64,
    pub mean_anomaly: f64,
    pub mean_motion: f64,
    pub equinox: String,
    pub orbit_class: OrbitClass,
}

#[derive(Debug, Deserialize)]
pub struct NearEarthObject {
    pub id: String,
    pub neo_reference_id: String,
    pub name: String,
    pub designation: String,
    pub nasa_jpl_url: Url,
    pub absolute_magnitude_h: f64,
    pub estimated_diameter: EstimatedDiameter,
    pub is_potentially_hazardous_asteroid: bool,
    pub close_approach_data: Vec<CloseApproachData>,
    pub orbital_date: Option<OrbitClass>,
    pub is_sentry_object: bool,
}

#[derive(Serialize)]
pub struct NeoHandler<'k> {
    #[serde(skip)]
    nasa: &'k NasaClient<'k>,
    api_key: &'k str,
}

impl<'k> NeoHandler<'k> {
    pub(crate) fn new(nasa: &'k NasaClient) -> Self {
        Self {
            nasa,
            api_key: &nasa.api_key,
        }
    }

    /// Look up an asteroid by its [NASA JPL small body (SPK-ID) ID](http://ssd.jpl.nasa.gov/sbdb_query.cgi)
    ///
    /// # Example
    ///
    /// ```
    /// # use kosmos::Kosmos;
    /// # async fn get_asteroid() {
    /// let asteroid = Kosmos::new()
    ///     .nasa()
    ///     .neo()
    ///     .lookup(3542519)
    ///     .await
    ///     .unwrap();
    /// assert_eq!(asteroid.name, String::from("(2010 PK9)"));
    /// # }
    /// ```
    pub async fn lookup(&self, asteroid_id: u64) -> Result<NearEarthObject> {
        self.nasa
            .kosmos
            .get(
                format!("https://api.nasa.gov/neo/rest/v1/neo/{}", asteroid_id),
                Some(self),
            )
            .await
    }
}
