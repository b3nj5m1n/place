use std::fs::File;
use std::io;
use std::str::FromStr;

use chrono::prelude::*;
use clap::{Arg, Command};
use iter_progress::ProgressableIter;
use num_format::{Locale, ToFormattedString};
use serde::de::Error as SerdeError;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use sqlx::sqlite::{SqliteConnectOptions, SqliteConnection, SqliteJournalMode, SqliteSynchronous};
use sqlx::{Connection, Executor};

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
impl From<PlacementYear> for u16 {
    fn from(year: PlacementYear) -> Self {
        match year {
            PlacementYear::_2017 => {
                return 2017;
            }
            PlacementYear::_2022 => {
                return 2022;
            }
            _ => {
                panic!("Trying to write unkown year to database.");
            }
        }
    }
}

impl<'de> Deserialize<'de> for PixelPlacement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(PixelPlacementVisotor {})
    }
}

async fn read<R: io::Read>(reader: R, config: &mut Config) {
    let mut rdr = csv::Reader::from_reader(reader);
    for (state, result) in rdr.deserialize().progress() {
        state.do_every_n_sec(5., |state| {
            println!(
                "Inserted {:>11} records at a rate of {} per sec.",
                state.num_done().to_formatted_string(&Locale::en),
                (state.rate() as i64).to_formatted_string(&Locale::en)
            );
        });
        let record: Result<PixelPlacement, csv::Error> = result;
        write(record, config).await;
    }
}

struct Config {
    db: Option<SqliteConnection>,
    stout: bool,
}

async fn write(placement: Result<PixelPlacement, csv::Error>, config: &mut Config) {
    match placement {
        Ok(pixel_placement) => {
            if config.stout {
                println!("{:?}", pixel_placement);
            }
            if let Some(conn) = &mut config.db {
                match pixel_placement.coordinates {
                    PlacementCoordinates::Tile(coordinates) => {
                        let query = sqlx::query(
                            "INSERT INTO placements (ts, user_hash, coordinate_x, coordinate_y, color, year)
                             VALUES (?, ?, ?, ?, ?, ?)",
                        )
                        .bind(pixel_placement.timestamp)
                        .bind(pixel_placement.user_hash)
                        .bind(coordinates.x)
                        .bind(coordinates.y)
                        .bind(pixel_placement.color)
                        .bind(u16::from(pixel_placement.year));
                        conn.execute(query).await.unwrap();
                    }
                    PlacementCoordinates::Rect(coordinates_1, coordinates_2) => {
                        let query = sqlx::query(
                            "INSERT INTO placements_moderation (ts, user_hash, coordinate_x_1, coordinate_y_1, coordinate_x_2, coordinate_y_2, color, year)
                             VALUES (?, ?, ?, ?, ?, ?)",
                        )
                        .bind(pixel_placement.timestamp)
                        .bind(pixel_placement.user_hash)
                        .bind(coordinates_1.x)
                        .bind(coordinates_1.y)
                        .bind(coordinates_2.x)
                        .bind(coordinates_2.y)
                        .bind(pixel_placement.color)
                        .bind(u16::from(pixel_placement.year));
                        conn.execute(query).await.unwrap();
                    }
                }
            }
        }
        Err(error) => {
            if config.stout {
                println!("Error parsing line: {}", error);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let m = Command::new("r/place dataset parser")
        .about("Parses 2017 & 2022 datasets from r/place")
        .arg(Arg::new("files").min_values(1))
        .arg(Arg::new("database").short('d').takes_value(true))
        .arg(Arg::new("stdout").short('s').takes_value(true))
        .get_matches();

    let mut config = Config {
        db: None,
        stout: false,
    };

    if let Some(filename) = m.value_of("database") {
        let database_url = format!("sqlite://{}", filename);
        let connection_options = SqliteConnectOptions::from_str(&database_url)
            .unwrap()
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);
        let conn = SqliteConnection::connect_with(&connection_options).await;
        match conn {
            Ok(conn) => {
                config.db = Some(conn);
                config.stout = false;
            }
            Err(_error) => {
                panic!(
                    "Error trying to establish database connection to {}",
                    filename
                );
            }
        }
    }

    if let Some(conn) = &mut config.db {
        conn.execute(
            r#"
                    DROP TABLE IF EXISTS placements;
                    CREATE TABLE IF NOT EXISTS placements (
                      ts INTEGER,
                      user_hash TEXT,
                      coordinate_x INTEGER,
                      coordinate_y INTEGER,
                      color TEXT,
                      year INTERGER,
                      CHECK (year = 2017 or year = 2022),
                      CHECK (
                        ( coordinate_x >= 0 AND coordinate_y >= 0)
                        AND
                        (
                          ( year = 2022 AND coordinate_x < 2000 AND coordinate_y < 2000)
                          OR
                          ( year = 2017 AND coordinate_x <= 1000 AND coordinate_y <= 1000)
                        )
                      )
                    );
                    /* CREATE INDEX indx_placements_user ON placements (user_hash);
                    CREATE INDEX indx_placements_coordinate ON placements (coordinate_x, coordinate_y);
                    CREATE INDEX indx_placements_color ON placements (color); */
                    DROP TABLE IF EXISTS placements_moderation;
                    CREATE TABLE IF NOT EXISTS placements_moderation (
                      ts INTEGER,
                      user_hash TEXT,
                      coordinate_x_1 INTEGER,
                      coordinate_y_1 INTEGER,
                      coordinate_x_2 INTEGER,
                      coordinate_y_2 INTEGER,
                      color TEXT,
                      year INTERGER,
                      CHECK (year = 2022),
                      CHECK (
                        ( coordinate_x_1 >= 0 AND coordinate_y_1 >= 0)
                        AND
                        ( coordinate_x_1 < 2000 AND coordinate_y_1 < 2000)
                        AND
                        ( coordinate_x_2 >= 0 AND coordinate_y_2 >= 0)
                        AND
                        ( coordinate_x_2 < 2000 AND coordinate_y_2 < 2000)
                      )
                    );
        "#,
        )
        .await
        .unwrap();
    }

    match m.values_of("files") {
        Some(values) => {
            let mut files: Vec<File> = Vec::new();
            for filename in values {
                match File::open(filename) {
                    Ok(file) => {
                        files.push(file);
                    }
                    Err(_error) => {
                        panic!("Error trying to read file {}", filename);
                    }
                }
            }
            for file in files {
                read(file, &mut config).await;
            }
        }
        None => {
            read(io::stdin(), &mut config).await;
        }
    }
}
