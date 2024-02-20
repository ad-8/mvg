use serde::{Deserialize, Serialize};

const MVG_STATIONS: &str = "https://www.mvg.de/.rest/zdm/stations";

/// Represents a MVG station ("Haltestelle").
///
/// ## Example API Response
/// ```clojure
/// {:abbreviation "KA",
///  :divaId 1,
///  :id "de:09162:1",
///  :latitude 48.13951,
///  :longitude 11.56613,
///  :name "Karlsplatz (Stachus)",
///  :place "München",
///  :products ["UBAHN" "BUS" "TRAM" "SBAHN"],
///  :tariffZones "m"}
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub abbreviation: Option<String>,
    pub diva_id: Option<u32>,
    pub id: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub name: Option<String>,
    pub place: Option<String>,
    pub products: Option<Vec<String>>,
    pub tariff_zones: Option<String>,
}

/// Retrieve a list of all stations.
pub async fn request_stations() -> Result<Vec<Station>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(MVG_STATIONS).await?;
    let stations = resp.json::<Vec<Station>>().await?;

    Ok(stations)
}

#[cfg(test)]
mod tests {
    use super::*;
}
