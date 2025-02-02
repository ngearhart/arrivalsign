use std::{any::TypeId, collections::HashMap, env};

use firebase_rs::Firebase;
use futures::future::{try_join_all, TryJoinAll};
use retry::delay::{jitter, Exponential};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Widget {
    name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alert {
    pub message: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlertWidget {
    name: String,
    pub alerts: Vec<Alert>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArrivalWidget {
    name: String,
    pub station_id: String
}

impl AlertWidget {
    pub fn get_messages(&self) -> Vec<String> {
        self.alerts.iter().cloned().map(|alert| alert.message).collect::<Vec<String>>()
    } 
}

macro_rules! retry {
    ($f:expr, $count:expr, $interval_millis:expr) => {{
        let mut retries = 0;
        let mut time = Exponential::from_millis($interval_millis).map(jitter);
        let result = loop {
            let result = $f;
            if result.is_ok() {
                break result;
            } else if retries > $count {
                break result;
            } else {
                retries += 1;
                tokio::time::sleep(time.next().unwrap()).await;
            }
        };
        result
    }};
    ($f:expr) => {
        retry!($f, 10, 500)
    };
}

pub trait LoadableWidget {
    async fn load() -> Self;
}

impl LoadableWidget for ArrivalWidget {
    async fn load() -> Self {
        let widgets = get_widgets().await;
        let key = widgets.keys().find(|&k| widgets.get(k).unwrap().name == "DCMetroTrainArrivalWidget").unwrap();

        retry! {
            get_firebase().at("widgets").at(key).get::<ArrivalWidget>().await
        }.expect("Could not get ArrivalWidget from firebase")
    }
}

impl LoadableWidget for AlertWidget {
    async fn load() -> Self {
        let widgets = get_widgets().await;
        let key = widgets.keys().find(|&k| widgets.get(k).unwrap().name == "DCMetroAlertsWidget").unwrap();

        retry! {
            get_firebase().at("widgets").at(key).get::<AlertWidget>().await
        }.expect("Could not get AlertWidget from firebase")
    }
}

fn get_firebase() -> Firebase {
    Firebase::auth(
        &env::var("FIREBASE_URL").unwrap(),
        &env::var("FIREBASE_API_KEY").unwrap()
    ).unwrap()
}

async fn get_widgets() -> HashMap<String, Widget> {
    let firebase = get_firebase().at("widgets");

    let widgets = retry! {{
        firebase.get::<HashMap<String, Widget>>().await
    }}.expect("Could not connect to Firebase after retries");

    widgets
}
