pub mod fatigue;
pub mod hunger;
pub(crate) mod seeding;
use self::{fatigue::Fatigue, hunger::Hunger};
use crate::{
    data,
    global::{Dimensions, Epoch, Mass, Meters},
};
use bevy::prelude::{Bundle, Component};
use chrono::{DateTime, Utc};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt::Display;

#[derive(Bundle)]
pub struct CitizenBundle {
    pub vitals: Vitals,
    pub attributes: Attributes,
    pub civic_identity: CivicIdentity,
    pub dimensions: Dimensions,
    pub epoch: Epoch,
    pub fatigue: Fatigue,
    pub gender: Gender,
    pub hunger: Hunger,
    pub mass: Mass,
    pub physiology: Physiology,
}

impl CitizenBundle {
    pub fn random(reference: DateTime<Utc>, max_age: f32) -> CitizenBundle {
        let age = rand::thread_rng().gen_range(0.0..=max_age);
        let epoch = reference - chrono::Duration::days(age as i64 * 365);

        CitizenBundle {
            vitals: Vitals::Alive,
            attributes: rand::random(),
            civic_identity: rand::random(),
            dimensions: Dimensions {
                height: Meters(rand::thread_rng().gen_range(1.5..=1.9)),
                width: Meters(0.4), // default humanoid wideness
                depth: Meters(0.2), // default humanoid... depth?
            },
            epoch: Epoch(epoch),
            fatigue: Fatigue::new(),
            gender: rand::random(),
            hunger: Hunger::new(),
            mass: Mass(rand::thread_rng().gen_range(70.0..=120.0)),
            physiology: rand::random(),
        }
    }
}

#[derive(Component, Clone)]
pub struct Attributes {
    pub charisma: u16, // how well they can communicate, how well they can lead, manipulate, etc.
    pub determination: u16, // how likely they are to stick to a task or give up
    pub ethics: u16, // how likely they are to do the right thing, how likely they are to do the wrong thing
    pub intelligence: u16, // how quickly they learn, maximum capacity for knowledge
    pub speed: u16,  // movement speed, reaction time, how quickly they can process information
    pub strength: u16, // physical strength, how much they can lift, how hard they can hit
}

impl Distribution<Attributes> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Attributes {
        Attributes {
            charisma: rng.gen_range(10..=15),
            determination: rng.gen_range(10..=15),
            ethics: rng.gen_range(10..=15),
            intelligence: rng.gen_range(10..=15),
            speed: rng.gen_range(10..=15),
            strength: rng.gen_range(10..=15),
        }
    }
}

#[derive(Component, Clone)]
pub struct CivicIdentity {
    pub name: String,
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
        }
    }
}

#[derive(Component, Clone)]
pub enum Physiology {
    Human,
    Android,
}

impl Distribution<Physiology> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Physiology {
        match rng.gen_range(0..=1) {
            0 => Physiology::Android,
            _ => Physiology::Human,
        }
    }
}

impl Display for Physiology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Physiology::Human => write!(f, "Human"),
            Physiology::Android => write!(f, "Android"),
        }
    }
}

#[derive(Component, Clone)]
pub enum Vitals {
    Alive,
    Deceased,
    Unknown,
}

impl Distribution<Vitals> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vitals {
        match rng.gen_range(0..=2) {
            0 => Vitals::Alive,
            1 => Vitals::Deceased,
            _ => Vitals::Unknown,
        }
    }
}

impl Display for Vitals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vitals::Alive => write!(f, "Alive"),
            Vitals::Deceased => write!(f, "Deceased"),
            Vitals::Unknown => write!(f, "Unknown"),
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

pub struct Census {
    pub total: u32,
    pub human: u32,
    pub android: u32,
    pub male: u32,
    pub female: u32,
    pub non_binary: u32,
    pub ungendered: u32,
    pub living: u32,
    pub deceased: u32,
    pub unknown: u32,
    pub infants: u32,
    pub children: u32,
    pub adults: u32,
    pub adolescents: u32,
    pub elderly: u32,
}

impl Census {
    pub fn empty() -> Census {
        Census {
            total: 0,
            human: 0,
            android: 0,
            male: 0,
            female: 0,
            non_binary: 0,
            ungendered: 0,
            living: 0,
            deceased: 0,
            unknown: 0,
            infants: 0,
            children: 0,
            adolescents: 0,
            adults: 0,
            elderly: 0,
        }
    }
}
