use std::error::Error;
use std::io;
use std::process;
use std::str::FromStr;

use chrono::prelude::*;
use serde::de::Error as SerdeError;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize};

struct PixelPlacementVisotor {}

impl<'de> Visitor<'de> for PixelPlacementVisotor {
    type Value = PixelPlacement;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Could not deserialize pixel placement")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut timestamp: i64 = i64::MAX;
        let mut user_hash: String = String::from("");
        let mut coordinates_1: Coordinates = Coordinates {
            x: u16::MAX,
            y: u16::MAX,
        };
        let mut coordinates_2: Coordinates = Coordinates {
            x: u16::MAX,
            y: u16::MAX,
        };
        let mut is_rect: bool = false;
        let mut color: String = String::from("");
        let mut year: PlacementYear = PlacementYear::_UNKOWN;

        while let Some((key, value)) = map.next_entry::<String, String>()? {
            match key.as_str() {
                "timestamp" | "ts" => {
                    timestamp = value.parse::<DateTime<Utc>>().unwrap().timestamp();
                }
                "user_id" | "user_hash" => {
                    user_hash = value;
                }
                "pixel_color" => {
                    color = value;
                    year = PlacementYear::_2022;
                }
                "color" => {
                    match u16::from_str(&value) {
                        Ok(value) => match value {
                            0 => {
                                color = String::from("#FFFFFF");
                            }
                            1 => {
                                color = String::from("#E4E4E4");
                            }
                            2 => {
                                color = String::from("#888888");
                            }
                            3 => {
                                color = String::from("#222222");
                            }
                            4 => {
                                color = String::from("#FFA7D1");
                            }
                            5 => {
                                color = String::from("#E50000");
                            }
                            6 => {
                                color = String::from("#E59500");
                            }
                            7 => {
                                color = String::from("#A06A42");
                            }
                            8 => {
                                color = String::from("#E5D900");
                            }
                            9 => {
                                color = String::from("#94E044");
                            }
                            10 => {
                                color = String::from("#02BE01");
                            }
                            11 => {
                                color = String::from("#00E5F0");
                            }
                            12 => {
                                color = String::from("#0083C7");
                            }
                            13 => {
                                color = String::from("#0000EA");
                            }
                            14 => {
                                color = String::from("#E04AFF");
                            }
                            15 => {
                                color = String::from("#820080");
                            }
                            _ => {
                                return Err(A::Error::invalid_value(
                                    serde::de::Unexpected::Unsigned(value.into()),
                                    &"index between 0 and 15",
                                ));
                            }
                        },
                        Err(error) => {
                            return Err(A::Error::custom(error));
                        }
                    }
                    year = PlacementYear::_2017;
                }
                "coordinate" => {
                    let nums = value.split(",").collect::<Vec<&str>>();
                    match nums.len() {
                        2 => {}
                        4 => {
                            is_rect = true;
                        }
                        _ => {
                            return Err(serde::de::Error::invalid_length(nums.len(), &"2"));
                        }
                    }
                    match u16::from_str(nums[0]) {
                        Ok(value) => {
                            coordinates_1.x = value;
                        }
                        Err(error) => {
                            return Err(A::Error::custom(error));
                        }
                    }
                    match u16::from_str(nums[1]) {
                        Ok(value) => {
                            coordinates_1.y = value;
                        }
                        Err(error) => {
                            return Err(A::Error::custom(error));
                        }
                    }
                    if is_rect {
                        match u16::from_str(nums[2]) {
                            Ok(value) => {
                                coordinates_2.x = value;
                            }
                            Err(error) => {
                                return Err(A::Error::custom(error));
                            }
                        }
                        match u16::from_str(nums[3]) {
                            Ok(value) => {
                                coordinates_2.y = value;
                            }
                            Err(error) => {
                                return Err(A::Error::custom(error));
                            }
                        }
                    }
                    year = PlacementYear::_2022;
                }
                "x_coordinate" => {
                    match u16::from_str(&value) {
                        Ok(value) => {
                            coordinates_1.x = value;
                        }
                        Err(error) => {
                            return Err(A::Error::custom(error));
                        }
                    }
                    year = PlacementYear::_2017;
                }
                "y_coordinate" => {
                    match u16::from_str(&value) {
                        Ok(value) => {
                            coordinates_1.y = value;
                        }
                        Err(error) => {
                            return Err(A::Error::custom(error));
                        }
                    }
                    year = PlacementYear::_2017;
                }
                _ => {}
            }
        }

        if is_rect { println!("Rect from {:?} to {:?}.", coordinates_1, coordinates_2); }

        Ok(PixelPlacement {
            timestamp,
            user_hash,
            coordinates: if !is_rect {
                PlacementCoordinates::Tile(coordinates_1)
            } else {
                PlacementCoordinates::Rect(coordinates_1, coordinates_2)
            },
            color,
            year,
        })
    }
}

#[derive(Debug)]
struct PixelPlacement {
    timestamp: i64,
    user_hash: String,
    coordinates: PlacementCoordinates,
    color: String,
    year: PlacementYear,
}

#[derive(Debug)]
enum PlacementCoordinates {
    Tile(Coordinates),
    Rect(Coordinates, Coordinates),
}

#[derive(Debug)]
struct Coordinates {
    x: u16,
    y: u16,
}

#[derive(Debug)]
enum PlacementYear {
    _UNKOWN,
    _2017,
    _2022,
}

impl<'de> Deserialize<'de> for PixelPlacement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(PixelPlacementVisotor {})
    }
}

fn read() {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Result<PixelPlacement, csv::Error> = result;
        match record {
            Ok(pixel_placement) => {
                // println!("{:?}", pixel_placement);
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
