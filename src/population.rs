use crate::global::{Dimensions, Kilograms, Years};
use bevy::prelude::Component;
use chrono::{DateTime, Utc};

#[derive(Component)]
pub struct CivicIdentity {
    civic_id: String,
    name: String,
    surname: String,
    status: Status,
}

#[derive(Component)]
pub struct Epoch {
    age: Years,
    epoch: DateTime<Utc>,
}

#[derive(Component)]
pub struct LifeformClassification {
    species: Species,
}

#[derive(Component)]
pub struct Physical {
    dimensions: Dimensions,
    mass: Kilograms,
}

#[derive(Component)]
pub struct SexualIdentity {
    gender: Gender,
}

pub enum Species {
    Human,
    Android,
}

pub enum Status {
    Alive,
    Dead,
    Missing,
    Unknown,
}

pub enum Gender {
    Male,
    Female,
    NonBinary,
    None,
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
