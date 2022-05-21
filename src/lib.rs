pub mod routes;

use once_cell::sync::Lazy;
use opentelemetry_prometheus::PrometheusExporter;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Person {
    pub firstname: String,
    pub lastname: String,
}

impl TryFrom<(sled::IVec, sled::IVec)> for Person {
    type Error = anyhow::Error;

    fn try_from(sled: (sled::IVec, sled::IVec)) -> Result<Self, Self::Error> {
        let firstname = String::from_utf8(sled.0.iter().copied().collect())?;
        let lastname = String::from_utf8(sled.1.iter().copied().collect())?;

        Ok(Person {
            firstname,
            lastname,
        })
    }
}

static PROMETHEUS: Lazy<PrometheusExporter> =
    Lazy::new(|| opentelemetry_prometheus::exporter().init());
