use crate::population::{Disorder, Implant};
use crate::transport::TransportNetwork;
use bevy::utils::HashMap;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize};
use std::fs;

#[derive(Deserialize)]
pub struct Names {
    elements: Vec<String>,
    human: Vec<String>,
}

#[derive(Deserialize)]
pub struct Seed {
    pub date: DateTime<Utc>,
    population_count: u32,
    rate_of_time: f32,
    transport_speeds_road: f32,
    fast_rate_of_time: f32,
}

#[derive(Deserialize)]
pub struct Mods {
    implants: HashMap<String, Implant>,
    disorders: HashMap<String, Disorder>,
}

pub fn transport_network() -> TransportNetwork {
    let datafile = read_datafile("transport_network.toml");
    decode_datafile::<TransportNetwork>(datafile)
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
