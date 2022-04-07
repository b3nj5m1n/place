use std::error::Error;
use std::str::FromStr;
use std::io;
use std::process;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize)]
struct PixelPlacement2017 {
    #[serde(rename = "ts")]
    timestamp: String,
    user_hash: String,
    #[serde(rename = "x_coordinate")]
    coordinate_x: u16,
    #[serde(rename = "y_coordinate")]
    coordinate_y: u16,
    color: u8,
}
#[derive(Debug)]
struct Coordinates {
    x: u16,
    y: u16,
}
fn deserialize_coordinates<'de, D>(deserializer: D) -> Result<Coordinates, D::Error>
where D: Deserializer<'de> {
    let buf = String::deserialize(deserializer)?;
    let nums = buf.split(",").collect::<Vec<&str>>();
    if nums.len() != 2 { return Err(serde::de::Error::invalid_length(3, &"2")); }
    return Ok(Coordinates { x: u16::from_str(nums[0]).unwrap(), y: u16::from_str(nums[1]).unwrap() });
}
#[derive(Debug, Deserialize)]
struct PixelPlacement2022 {
    timestamp: String,
    #[serde(rename = "user_id")]
    user_hash: String,
    #[serde(rename = "coordinate")]
    #[serde(deserialize_with = "deserialize_coordinates")]
    coordinates: Coordinates,
    #[serde(rename = "pixel_color")]
    color: String,
}
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum PixelPlacement {
    Place2022(PixelPlacement2022),
    Place2017(PixelPlacement2017),
}

fn read() {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Result<PixelPlacement, csv::Error> = result;
        match record {
            Ok(pixel_placement) => {
                println!("{:?}", pixel_placement);
            }
            Err(error) => {
                println!("Error parsing line: {}", error);
            }
        }
    }
}

fn main() {
    read();
}
