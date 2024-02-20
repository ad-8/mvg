use serde::{Deserialize, Serialize};

const MVG_STATIONS: &str = "https://www.mvg.de/.rest/zdm/stations";
const MVG_DEPARTURES: &str = "https://www.mvg.de/api/fib/v2/departure?globalId=";

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

/// Represents information about an upcoming departure.
///
/// ## Example API Response
/// ```clojure
/// {:bannerHash "",
///  :cancelled false,
///  :delayInMinutes 2,
///  :destination "Ebersberg",
///  :divaId "92M06",
///  :label "S6",
///  :messages [],
///  :network "ddb",
///  :occupancy "UNKNOWN",
///  :plannedDepartureTime 1708433340000,
///  :platform 1,
///  :platformChanged false,
///  :realtime true,
///  :realtimeDepartureTime 1708433460000,
///  :sev false,
///  :stopPointGlobalId "",
///  :trainType "",
///  :transportType "SBAHN"}
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartureInfo {
    pub banner_hash: Option<String>,
    pub cancelled: Option<bool>,
    pub delay_in_minutes: Option<i32>, // can be negative
    pub destination: Option<String>,
    pub diva_id: Option<String>,
    pub label: Option<String>,
    pub messages: Option<Vec<String>>,
    pub network: Option<String>,
    pub occupancy: Option<String>,
    pub planned_departure_time: Option<i64>, // Central European Standard Time [ms]
    pub platform: Option<u32>,
    pub platform_changed: Option<bool>,
    pub realtime: Option<bool>,
    pub realtime_departure_time: Option<i64>, // Central European Standard Time [ms]
    pub sev: Option<bool>,
    pub stop_point_global_id: Option<String>,
    pub stop_position_number: Option<u32>,
    pub train_type: Option<String>,
    pub transport_type: Option<String>,
}

/// Retrieve upcoming departures for a station.
pub async fn request_departures<S: Into<String>>(station_id: S) -> Result<Vec<DepartureInfo>, Box<dyn std::error::Error>> {
    let url = format!("{}{}", MVG_DEPARTURES, station_id.into());
    let resp = reqwest::get(url).await?;
    let departures = resp.json::<Vec<DepartureInfo>>().await?;

    Ok(departures)
}


#[cfg(test)]
mod tests {
    use super::*;
}
