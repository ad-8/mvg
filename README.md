## About
This crate provides a Rust API to communicate with the unofficial [MVG (Münchner Verkehrsgesellschaft)](https://www.mvg.de/) API.

See the example section below, which shows how to query timetable information 
and more for public transport in and around Munich.

Since no information about an official  API could be found on the [MVG developer website](https://www.mvv-muenchen.de/fahrplanauskunft/fuer-entwickler/index.html), 
credit and thanks go to [this Python project](https://github.com/mondbaron/mvg/),
which documents some of the available endpoints.

Right now, this is very much a work in progress, as only a "happy path" solution is implemented,
which basically only documents the API endpoints.

## Disclaimer
This is not an official project. Please acknowledge the [imprint on the official MVG website](https://www.mvg.de/impressum.html)
and respect the copyright section, which states:
> [...] 
> Our systems are used for direct customer interaction.
> The processing of our content or data by third parties requires our express consent. 
> **For private, non-commercial purposes, moderate use is tolerated without our express consent.**
>  Any form of data mining does not constitute moderate use. [...]



## Examples
```rust
#[tokio::main]
async fn main() {
    // prints
    // "Starnberg Nord"
    // "P+R Starnberg Nord"
    // "B+R Starnberg-Nord 02 (Hans-Zellner-Weg)"
    let locations = mvg::locations("Starnberg Nord").await.unwrap();
    for l in locations.iter().take(3) {
        println!("{:?}", l.name.as_ref().unwrap());
    }

    // prints e.g.
    // SBAHN S6 Tutzing - 20:51:00
    // SBAHN S6 Ostbahnhof - 21:01:00
    // SBAHN S6 Tutzing - 21:04:00
    let starnberg_nord_gid = locations.first().unwrap().global_id.as_ref().unwrap();
    mvg::departures(starnberg_nord_gid)
        .await
        .unwrap()
        .iter()
        .take(3)
        .for_each(|d| {
            println!(
                "{} {} {} - {}",
                d.transport_type.as_ref().unwrap(),
                d.label.as_ref().unwrap(),
                d.destination.as_ref().unwrap(),
                ts_to_hms(d.realtime_departure_time.unwrap() / 1000)
            )
        });

    // prints
    // Marienplatz (Theatinerstraße) - 142 meters
    // Marienplatz - 189 meters
    // Marienplatz (Rindermarkt) - 338 meters
    let frauenkirche = (48.138611, 11.573889);
    mvg::nearby_locations(frauenkirche.0, frauenkirche.1)
        .await
        .unwrap()
        .iter()
        .take(3)
        .for_each(|l| {
            println!(
                "{} - {} meters",
                l.name.as_ref().unwrap(),
                l.distance_in_meters.unwrap(),
            )
        });

    // prints
    // Karlsplatz (Stachus) - (48.13951, 11.56613)
    // Marienplatz - (48.13725, 11.57542)
    // Isartor - (48.13364, 11.58303)
    let stations = mvg::stations().await.unwrap();
    stations.iter().take(3).for_each(|s| {
        println!(
            "{} - ({}, {})",
            s.name.as_ref().unwrap(),
            s.latitude.unwrap(),
            s.longitude.unwrap()
        )
    });

    // prints
    // de:09162:1
    // de:09162:2
    // de:09162:3
    let station_global_ids = mvg::station_global_ids().await.unwrap();
    station_global_ids
        .iter()
        .take(3)
        .for_each(|id| println!("{id}"));

    // prints
    // S1 - SBAHN
    // S2 - SBAHN
    // S3 - SBAHN
    let lines = mvg::lines().await.unwrap();
    lines.iter().take(3).for_each(|l| {
        println!(
            "{} - {}",
            l.name.as_ref().unwrap(),
            l.product.as_ref().unwrap()
        )
    });
}
```

