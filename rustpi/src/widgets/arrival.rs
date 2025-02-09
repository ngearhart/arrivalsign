// struct TrainState {

// }

use std::{collections::HashMap, env, error::Error, fmt::Debug};

use serde::{Deserialize, Serialize};


// These structs are a mess to account for what likely is .NET naming convention.

#[derive(Serialize, Deserialize, Debug)]
struct PredictionApiReturn {
    #[serde(rename(deserialize = "Trains"))]
    trains: Vec<Train>
}

#[derive(Serialize, Deserialize, Debug)]
struct Train {
    #[serde(rename(deserialize = "Car"))]
    car: String,
    #[serde(rename(deserialize = "Destination"))]
    destination: String,
    #[serde(rename(deserialize = "DestinationCode"))]
    destination_code: String,
    #[serde(rename(deserialize = "Group"))]
    group: String,
    #[serde(rename(deserialize = "Line"))]
    line: String,
    #[serde(rename(deserialize = "LocationCode"))]
    location_code: String,
    #[serde(rename(deserialize = "LocationName"))]
    location_name: String,
    #[serde(rename(deserialize = "Min"))]
    min: String
}

const API_URL: &str = "https://api.wmata.com/StationPrediction.svc/json/GetPrediction/";
const API_KEY_HEADER: &str = "api_key";

pub async fn get_latest_state(station_code: &str) -> Result<(), Box<dyn Error>> {

    let mut url = API_URL.to_owned();
    url.push_str(station_code);

    let client = reqwest::Client::new();

    match client.get(url)
        .header(API_KEY_HEADER, env::var("WMATA_API_KEY").unwrap())
        .send()
        .await {
            Ok(resp) => {
                let text= resp.json::<PredictionApiReturn>().await.expect("Should be deserializable");
                println!("{text:#?}");
                
                Ok(())
            }
            Err(err) => {
                println!("Reqwest Error: {}", err);
                Err(Box::new(err))
            }
        }
}

// pub fn render<D>(widget: ArrivalWidget, canvas: D) {
    
// }
