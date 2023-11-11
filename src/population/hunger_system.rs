use super::Vitals;
use crate::data::Seed;
use bevy::prelude::*;
use core::fmt::Display;

const MIN_HUNGER: f32 = 0.0;
const MAX_HUNGER: f32 = f32::MAX;

#[derive(Component, Clone)]
pub struct Hunger(f32);

impl Hunger {
    pub fn new() -> Self {
        Self(0.0)
    }

    pub fn increase(&mut self, amount: f32) {
        self.0 = (self.0 + amount).clamp(MIN_HUNGER, MAX_HUNGER);
    }

    pub fn level(&self) -> HungerLevel {
        match self.0 as u32 {
            0..=5 => HungerLevel::Satisfied,
            6..=24 => HungerLevel::Hungry,
            _ => HungerLevel::Starving,
        }
    }
}

impl Display for Hunger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.level() {
            HungerLevel::Starving => write!(f, "Starving"),
            HungerLevel::Hungry => write!(f, "Hungry"),
            HungerLevel::Satisfied => write!(f, "Satisfied"),
        }
    }
}

pub enum HungerLevel {
    Starving,
    Hungry,
    Satisfied,
}

pub fn hunger_advance(time: Res<Time>, seed: Res<Seed>, mut query: Query<(&Vitals, &mut Hunger)>) {
    for (alive, mut hunger) in query.iter_mut() {
        if let Vitals::Alive = alive {
            hunger.increase((seed.time_multiplier / 60.0 / 60.0) * time.delta_seconds());
        }
    }
}

mod test {
    #[test]
    fn test_hunger() {
        use super::*;
        let mut hunger = Hunger::new();
        assert_eq!(hunger.0, 0.0);
        hunger.increase(50.0);
        assert_eq!(hunger.0, 50.0);
    }
}
