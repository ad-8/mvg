use serde::{Deserialize, Serialize};

const MVG_LOCATION: &str = "https://www.mvg.de/api/fib/v2/location";
const MVG_STATION_NEARBY: &str = "https://www.mvg.de/api/fib/v2/station/nearby";
const MVG_DEPARTURE: &str = "https://www.mvg.de/api/fib/v2/departure";
const MVG_STATIONS: &str = "https://www.mvg.de/.rest/zdm/stations";
const MVG_STATION_GLOBAL_IDS: &str = "https://www.mvg.de/.rest/zdm/mvgStationGlobalIds";
const MVG_LINES: &str = "https://www.mvg.de/.rest/zdm/lines";

/// Represents a MVG station ("Haltestelle").
///
/// ## Example API Response
/// The first element of the response when querying the `/stations` endpoint:
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

/// Represents a MVG global station id.
///
/// Examples of valid ids are "de:09162:1" and "de:09162:9029".
type StationGlobalId = String;

/// Retrieve a list of all station global ids.
pub async fn request_station_global_ids() -> Result<Vec<StationGlobalId>, Box<dyn std::error::Error>>
{
    let resp = reqwest::get(MVG_STATION_GLOBAL_IDS).await?;
    let ids = resp.json::<Vec<StationGlobalId>>().await?;

    Ok(ids)
}

/// Represents a MVG line.
///
/// # Example API Response
/// Part of the response when querying the `/lines` endpoint:
/// ```clojure
/// ({:lineNumber -1, :name "N19", :product "TRAM"}
///  {:lineNumber -1, :name "N20", :product "TRAM"}
///  {:lineNumber -1, :name "N27", :product "TRAM"}
///  {:lineNumber 2012, :name "12", :product "TRAM"}
///  {:lineNumber 2016, :name "16", :product "TRAM"}
///  {:lineNumber 2017, :name "17", :product "TRAM"})
/// ```
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    pub line_number: Option<i32>,
    pub name: Option<String>,
    pub product: Option<String>,
}

/// Retrieve a list of all lines.
pub async fn request_lines() -> Result<Vec<Line>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(MVG_LINES).await?;
    let lines = resp.json::<Vec<Line>>().await?;

    Ok(lines)
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
pub async fn request_departures<S: Into<String>>(
    global_id: S,
) -> Result<Vec<DepartureInfo>, Box<dyn std::error::Error>> {
    let url = format!("{}?globalId={}", MVG_DEPARTURE, global_id.into());
    let resp = reqwest::get(url).await?;
    let departures = resp.json::<Vec<DepartureInfo>>().await?;

    Ok(departures)
}

/// Represents information about a location.
///
/// ## Example API Response
/// ```clojure
/// {:aliases "Stachus Bf. Bahnhof Muenchen Munchen KA",
///  :divaId 1,
///  :globalId "de:09162:1",
///  :hasZoomData true,
///  :latitude 48.13951,
///  :longitude 11.56613,
///  :name "Karlsplatz (Stachus)",
///  :place "München",
///  :surroundingPlanLink "KA",
///  :tariffZones "m",
///  :transportTypes ["UBAHN" "BUS" "TRAM" "SBAHN"],
///  :type "STATION"}
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub aliases: Option<String>,
    pub distance_in_meters: Option<i32>,
    pub diva_id: Option<u32>,
    pub global_id: Option<String>,
    pub has_zoom_data: Option<bool>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub name: Option<String>,
    pub place: Option<String>,
    pub surrounding_plan_link: Option<String>,
    pub tariff_zones: Option<String>,
    pub transport_types: Option<Vec<String>>,
    pub r#type: Option<String>,
}

/// Find a location using a query string.
///
/// Returns a list of locations, where the first element is the best match.
pub async fn find_location<S: Into<String>>(
    query: S,
) -> Result<Vec<Location>, Box<dyn std::error::Error>> {
    let url = format!("{}?query={}", MVG_LOCATION, query.into());
    let resp = reqwest::get(url).await?;
    let locations = resp.json::<Vec<Location>>().await?;

    Ok(locations)
}

/// Find a nearby location via latitude and longitude.
///
/// Returns a list of locations, where the first element is the best match.
pub async fn find_nearby_location(
    latitude: f32, longitude: f32
) -> Result<Vec<Location>, Box<dyn std::error::Error>> {
    let url = format!("{}?latitude={}&longitude={}", MVG_STATION_NEARBY, latitude, longitude);
    let resp = reqwest::get(url).await?;
    let locations = resp.json::<Vec<Location>>().await?;

    Ok(locations)
}



#[cfg(test)]
mod tests {
    use super::*;
}
