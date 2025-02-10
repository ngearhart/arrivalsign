use core::num;
use std::{cmp::Ordering, collections::HashMap, env, error::Error, fmt::Debug, ops::Deref};

use cached::proc_macro::cached;
use chrono::{DateTime, TimeDelta, Utc};
use embedded_graphics::{mono_font::{ascii::FONT_7X14, MonoTextStyle}, pixelcolor::Rgb888, prelude::{DrawTarget, Point, Primitive}, primitives::{PrimitiveStyle, Rectangle}, text::Text, Drawable};
use itertools::{join, Itertools};
use serde::{Deserialize, Serialize};

use crate::firebase::{ArrivalMessage, ArrivalWidget};


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
    group: String,  // Track ID - usually 1 or 2
    #[serde(rename(deserialize = "Line"))]
    line: Line,
    #[serde(rename(deserialize = "LocationCode"))]
    location_code: String,
    #[serde(rename(deserialize = "LocationName"))]
    location_name: String,
    #[serde(rename(deserialize = "Min"))]
    min: String
}

#[derive(Debug, Clone)]
pub struct TrainDisplayEntry {
    line: Line,
    line_color: Rgb888,
    destination: String,
    arrival: String,  // Can be in minutes or ARR, BRD
    arrival_timestamp: DateTime<Utc> 
}

pub trait ArrivalDisplayable {
    fn get_comparison_timestamp(&self) -> DateTime<Utc>;
    fn get_comparison_timestamp_no_sticky(&self) -> DateTime<Utc>;
    fn get_message(&self) -> String;
    fn get_line(&self) -> Line;
    fn get_line_color(&self) -> Rgb888;
    fn pretty_print(&self) -> String;
    fn get_leave(&self) -> String;
    fn is_sticky(&self) -> bool;
    fn get_arrival_time(&self) -> String;
}

impl ArrivalDisplayable for TrainDisplayEntry {
    fn get_message(&self) -> String {
        self.destination.clone()
    }

    fn get_line(&self) -> Line {
        self.line
    }

    fn get_line_color(&self) -> Rgb888 {
        self.line_color
    }

    fn pretty_print(&self) -> String {
        format!("{} {} {}", get_line_string(self.line), self.destination, self.arrival)
    }
    
    fn get_comparison_timestamp(&self) -> DateTime<Utc> {
        match self.arrival.parse::<i64>() {
            Ok(_) => self.arrival_timestamp,  // Use the arrival timestamp by default
            Err(_) => {
                if self.arrival == "ARR" {
                    return Utc::now() - TimeDelta::seconds(60); // If ARR, sort as if it arrived 1 minute ago
                } else if self.arrival == "BRD" {
                    return Utc::now() - TimeDelta::seconds(120); // If BRD, sort as if it arrived 2 minutes ago
                }
                return Utc::now() + TimeDelta::days(1);  // If something else, put it far below
            }
        }
    }

    fn get_comparison_timestamp_no_sticky(&self) -> DateTime<Utc> {
        self.get_comparison_timestamp()
    }

    fn is_sticky(&self) -> bool {
        false
    }

    fn get_leave(&self) -> String {
        match self.arrival.parse::<i64>() {
            Ok(_) => if self.arrival_timestamp - TimeDelta::minutes(15) > Utc::now() { (self.arrival_timestamp - TimeDelta::minutes(15) - Utc::now()).num_minutes().to_string() } else { "- ".to_string() },
            Err(_) => "- ".to_string()
        }
    }

    fn get_arrival_time(&self) -> String {
        self.arrival.clone()
    }
}

impl ArrivalDisplayable for ArrivalMessage {
    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn get_line(&self) -> Line {
        Line::TS
    }

    fn get_line_color(&self) -> Rgb888 {
        get_line_color(Line::TS)
    }

    fn pretty_print(&self) -> String {
        format!("{} {} {}", get_line_string(self.get_line()), self.message, (self.get_comparison_timestamp() - Utc::now()).num_minutes())
    }
    
    fn get_comparison_timestamp_no_sticky(&self) -> DateTime<Utc> {
        DateTime::from_timestamp_millis(self.time).expect("Invalid timestamp on custom arrival message")
    }

    fn get_comparison_timestamp(&self) -> DateTime<Utc> {
        match self.sticky {
            true => DateTime::from_timestamp(0, 0).unwrap(),  // Put sticky messages on top
            false => DateTime::from_timestamp_millis(self.time).expect("Invalid timestamp on custom arrival message")
        }
    }

    fn is_sticky(&self) -> bool {
        self.sticky
    }

    fn get_leave(&self) -> String {
        if self.get_comparison_timestamp() - TimeDelta::minutes(15) > Utc::now() {
            (Utc::now() - self.get_comparison_timestamp() - TimeDelta::minutes(15)).num_minutes().to_string()
        } else {
            "- ".to_string() 
        }
    }

    fn get_arrival_time(&self) -> String {
        (DateTime::from_timestamp_millis(self.time).expect("Invalid timestamp on custom arrival message") - Utc::now()).num_minutes().to_string()
    }
}

impl Ord for TrainDisplayEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_comparison_timestamp().cmp(&other.get_comparison_timestamp())
    }
}

impl PartialOrd for TrainDisplayEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_comparison_timestamp().cmp(&other.get_comparison_timestamp()))
    }
}

impl PartialEq for TrainDisplayEntry {
    fn eq(&self, other: &Self) -> bool {
        self.get_comparison_timestamp() == other.get_comparison_timestamp()
    }
}

impl Eq for TrainDisplayEntry {}

impl Ord for ArrivalMessage {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_comparison_timestamp().cmp(&other.get_comparison_timestamp())
    }
}

impl PartialOrd for ArrivalMessage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_comparison_timestamp().cmp(&other.get_comparison_timestamp()))
    }
}

impl PartialEq for ArrivalMessage {
    fn eq(&self, other: &Self) -> bool {
        self.get_comparison_timestamp() == other.get_comparison_timestamp()
    }
}

impl Eq for ArrivalMessage {}

impl Ord for Box<dyn ArrivalDisplayable> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_comparison_timestamp().cmp(&other.get_comparison_timestamp())
    }
}

impl PartialOrd for Box<dyn ArrivalDisplayable> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_comparison_timestamp().cmp(&other.get_comparison_timestamp()))
    }
}

impl PartialEq for Box<dyn ArrivalDisplayable> {
    fn eq(&self, other: &Self) -> bool {
        self.get_comparison_timestamp() == other.get_comparison_timestamp()
    }
}

impl Eq for Box<dyn ArrivalDisplayable> {}


#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Line {
   RD,
   OR,
   SV,
   YL,
   GR,
   BL,
   TS,
}

const API_URL: &str = "https://api.wmata.com/StationPrediction.svc/json/GetPrediction/";
const API_KEY_HEADER: &str = "api_key";
const LINE_HEIGHT: i32 = 10;
const LINE_HEIGHT_WITH_PADDING: i32 = 12;

fn get_line_color(line: Line) -> Rgb888 {
    match line {
        Line::RD => Rgb888::new(255, 0, 0),
        Line::OR => Rgb888::new(255, 85, 0),
        Line::YL => Rgb888::new(255, 255, 0),
        Line::GR => Rgb888::new(0, 255, 0),
        Line::BL => Rgb888::new(0, 0, 255),
        Line::TS => Rgb888::new(0, 51, 160),
        _ => Rgb888::new(170, 170, 170)
    }
}

fn get_line_string(line: Line) -> String {
    match line {
        Line::RD => String::from("RD"),
        Line::OR => String::from("OR"),
        Line::YL => String::from("YL"),
        Line::GR => String::from("GR"),
        Line::BL => String::from("BL"),
        Line::TS => String::from("TS"),
        _ => String::from("SV"),
    }
}

pub async fn get_latest_state(arrival_state: ArrivalWidget) -> Result<Vec<Box<dyn ArrivalDisplayable>>, Box<dyn Error>> {

    let mut url = API_URL.to_owned();
    url.push_str(&arrival_state.station_id);

    let client = reqwest::Client::new();

    match client.get(url)
        .header(API_KEY_HEADER, env::var("WMATA_API_KEY").unwrap())
        .send()
        .await {
            Ok(resp) => {
                let api_return= resp.json::<PredictionApiReturn>().await.expect("Should be deserializable");
                Ok(convert_api_return_to_display(api_return, arrival_state.messages))
            }
            Err(err) => {
                println!("Reqwest Error: {}", err);
                Err(Box::new(err))
            }
        }
}

fn convert_api_return_to_display(response: PredictionApiReturn, extra_messages: Option<Vec<ArrivalMessage>>) -> Vec<Box<dyn ArrivalDisplayable>> {
    let extra_msg: Vec<Box<dyn ArrivalDisplayable>> = extra_messages.unwrap_or(Vec::new())
        .iter()
        .filter(|v | (v.get_comparison_timestamp_no_sticky() - Utc::now()).num_seconds() >= 0) // Filter out custom messages that have expired
        .map(|v| Box::new(v.clone()) as _)
        .collect();
    response.trains
        .iter()
        .map(|train| {
            let arrival_as_number=  train.min.parse::<i64>();

            Box::new(TrainDisplayEntry {
                arrival: train.min.clone(),
                arrival_timestamp: Utc::now() + TimeDelta::minutes(if arrival_as_number.is_ok() {arrival_as_number.unwrap()} else {0}),
                destination: if train.destination == "No Passenger" || train.destination == "NoPssenger" || train.destination == "ssenger" { "No Psngr".to_string() } else { train.destination.clone() },
                line: train.line,
                line_color: get_line_color(train.line)
            }) as _
        })
        .chain(extra_msg)
        .sorted()
        .collect()
}

pub fn render_arrival_display<D>(state: Vec<Box<dyn ArrivalDisplayable>>, canvas: &mut D) where D: DrawTarget<Color = Rgb888>, <D as DrawTarget>::Error: Debug {
    let white_text_style = MonoTextStyle::new(&FONT_7X14, Rgb888::new(255, 255, 255));
    // Header
    let header_text_style = MonoTextStyle::new(&FONT_7X14, Rgb888::new(120, 120, 120));
    Text::new( "LN  DEST    LV MIN", Point::new(1, LINE_HEIGHT), header_text_style)
        .draw(canvas)
        .unwrap();

    for (index, message) in state.iter().enumerate() {
        // Draw left rectangle
        Rectangle::with_corners(
            Point::new(1, LINE_HEIGHT_WITH_PADDING * (index as i32 + 2)),
            Point::new(2, LINE_HEIGHT_WITH_PADDING * (index as i32 + 1) + (LINE_HEIGHT_WITH_PADDING - LINE_HEIGHT))
        )
            .into_styled(PrimitiveStyle::with_fill(message.get_line_color()))
            .draw(canvas).unwrap();

        // Draw line text
        Text::new( 
            &get_line_string(message.get_line()),
            Point::new(5, LINE_HEIGHT_WITH_PADDING * (index as i32 + 2)),
            MonoTextStyle::new(&FONT_7X14, message.get_line_color())
        )
            .draw(canvas)
            .unwrap();

        // Draw LEAVE - Custom for this sign to indicate when to leave the office to catch this train (15 minutes before)
        Text::new( 
            &message.get_leave(),
            Point::new(84, LINE_HEIGHT_WITH_PADDING * (index as i32 + 2)),
            white_text_style
        )
            .draw(canvas)
            .unwrap();

        // Draw minutes
        Text::new( 
            &message.get_arrival_time(),
            Point::new(106, LINE_HEIGHT_WITH_PADDING * (index as i32 + 2)),
            white_text_style
        )
            .draw(canvas)
            .unwrap();
    }
}

