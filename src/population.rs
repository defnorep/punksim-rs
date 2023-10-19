pub(crate) mod population_seeding_system;

use crate::{
    data,
    global::{Dimensions, Mass, Meters, Years},
};
use bevy::prelude::{Bundle, Component};
use chrono::{DateTime, Utc};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Bundle)]
pub struct CitizenBundle {
    pub civic_identity: CivicIdentity,
    pub dimensions: Dimensions,
    pub epoch: Epoch,
    pub gender: Gender,
    pub mass: Mass,
    pub species: Species,
    pub location: Location,
}

impl CitizenBundle {
    pub fn random(reference: DateTime<Utc>, max_age: f32) -> CitizenBundle {
        let age = rand::thread_rng().gen_range(0.0..=max_age);
        let epoch = reference - chrono::Duration::days(age as i64 * 365);

        CitizenBundle {
            civic_identity: rand::random(),
            dimensions: Dimensions {
                height: Meters(rand::thread_rng().gen_range(1.5..=1.9)),
                width: Meters(0.4), // default humanoid wideness
                depth: Meters(0.2), // default humanoid... depth?
            },
            epoch: Epoch(epoch),
            gender: rand::random(),
            mass: Mass(rand::thread_rng().gen_range(70.0..=120.0)),
            species: rand::random(),
            location: Location("Residence-1".into()),
        }
    }
}

#[derive(Component, Clone)]
pub struct Location(pub String);

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component, Clone)]
pub struct CivicIdentity {
    pub name: String,
    pub status: Status,
    pub surname: String,
}

impl Distribution<CivicIdentity> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CivicIdentity {
        let names = data::names();
        let length = names.human.len() as f32;
        let [roll1, roll2] = rng.gen::<[f32; 2]>().map(|r| (r * length).floor() as usize);

        CivicIdentity {
            name: names.human.get(roll1).unwrap().into(),
            surname: names.human.get(roll2).unwrap().into(),
            status: Status::Living,
        }
    }
}

#[derive(Component, Clone)]
pub struct Epoch(pub DateTime<Utc>);

impl Display for Epoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Component, Clone)]
pub enum Species {
    Human,
    Android,
}

impl Distribution<Species> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Species {
        match rng.gen_range(0..=1) {
            0 => Species::Android,
            _ => Species::Human,
        }
    }
}

impl Display for Species {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Species::Human => write!(f, "Human"),
            Species::Android => write!(f, "Android"),
        }
    }
}

#[derive(Component, Clone)]
pub enum Status {
    Living,
    Deceased,
    Missing,
    Unknown,
}

impl Distribution<Status> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Status {
        match rng.gen_range(0..=3) {
            0 => Status::Living,
            1 => Status::Deceased,
            2 => Status::Missing,
            _ => Status::Unknown,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Living => write!(f, "Living"),
            Status::Deceased => write!(f, "Deceased"),
            Status::Missing => write!(f, "Missing"),
            Status::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Component, Clone)]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    None,
}

impl Distribution<Gender> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gender {
        match rng.gen_range(0..=3) {
            0 => Gender::Male,
            1 => Gender::Female,
            2 => Gender::NonBinary,
            _ => Gender::None,
        }
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
            Gender::NonBinary => write!(f, "Non-Binary"),
            Gender::None => write!(f, "None"),
        }
    }
}

// this might make more sense to have as a resource than a component, we'll see.
pub struct Census {
    total: u32,
    human: u32,
    android: u32,
    male: u32,
    female: u32,
    non_binary: u32,
    ungendered: u32,
    living: u32,
    dead: u32,
    missing: u32,
}

#[derive(Deserialize)]
pub struct Disorder {
    pub name: String,
    pub description: String,
    pub adjective: String,
    pub variant: DisorderVariant,
}

#[derive(Deserialize)]
pub enum DisorderVariant {
    Mental,
    Physical,
}

#[derive(Deserialize)]
pub struct Implant {
    pub name: String,
    pub description: String,
}
