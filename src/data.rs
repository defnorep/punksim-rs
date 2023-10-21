use crate::population::{Disorder, Implant};
use bevy::prelude::Resource;
use bevy::utils::HashMap;
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
    pub rate_of_time: f32,
    pub transport_speeds_road: f32,
    pub fast_rate_of_time: f32,
}

#[derive(Deserialize)]
pub struct Mods {
    implants: HashMap<String, Implant>,
    disorders: HashMap<String, Disorder>,
}

pub fn names() -> Names {
    let datafile = read_datafile("names.toml");
    decode_datafile::<Names>(datafile)
}

pub fn seed() -> Seed {
    let datafile = read_datafile("seed.toml");
    decode_datafile::<Seed>(datafile)
}

pub fn mods() -> Mods {
    let datafile = read_datafile("mods.toml");
    decode_datafile::<Mods>(datafile)
}

fn decode_datafile<T: DeserializeOwned>(datafile: String) -> T {
    toml::from_str::<T>(&datafile).unwrap()
}

fn read_datafile(path: &str) -> String {
    let filename = format!("data/{}", path);
    fs::read_to_string(filename).expect("Unable to read the provided datafile")
}
