use bevy::ecs::component::Component;
use chrono::{DateTime, Utc};
use std::fmt::Display;

#[derive(Component, Clone)]
pub struct Meters(pub f32);

impl Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*}", 2, self.0)
    }
}

#[derive(Component, Clone)]
pub struct Mass(pub f32);

impl Display for Mass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*}", 2, self.0)
    }
}

#[derive(Component, Clone)]
pub struct Years(pub u32);

impl Display for Years {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*}", 2, self.0)
    }
}

#[derive(Component, Clone)]
pub struct Dimensions {
    pub height: Meters,
    pub width: Meters,
    pub depth: Meters,
}

#[derive(Component, Clone)]
pub struct Epoch(pub DateTime<Utc>);

impl Epoch {
    pub fn age(&self, reference: DateTime<Utc>) -> u32 {
        reference.years_since(self.0).unwrap()
    }
}

impl Display for Epoch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
