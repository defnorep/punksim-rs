use bevy::prelude::Resource;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};
use std::fs;

#[derive(Deserialize)]
pub struct Names {
    pub elements: Vec<String>,
    pub human: Vec<String>,
}

#[derive(Resource, Deserialize)]
pub struct Seed {
    pub date: DateTime<Utc>,
    pub population_count: u32,
    pub time_multiplier: f32,
}

pub fn names() -> Names {
    let datafile = read_datafile("names.toml").unwrap();
    decode_datafile::<Names>(datafile).unwrap()
}

pub fn seed() -> Seed {
    match read_datafile("seed.override.toml") {
        Ok(datafile_override) => {
            return decode_datafile::<Seed>(datafile_override).unwrap();
        }
        Err(_) => {
            let datafile = read_datafile("seed.toml").unwrap();
            return decode_datafile::<Seed>(datafile).unwrap();
        }
    }
}

fn decode_datafile<T: DeserializeOwned>(datafile: String) -> Result<T, toml::de::Error> {
    toml::from_str::<T>(&datafile)
}

fn read_datafile(path: &str) -> Result<String, std::io::Error> {
    let filename = format!("data/{}", path);
    fs::read_to_string(filename)
}
