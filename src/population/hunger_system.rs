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
        hunger.increase(5.0);
        assert_eq!(hunger.0, 5.0);
    }

    #[test]
    fn test_hunger_advance() {
        use super::*;
        use chrono::Utc;
        use std::time::Duration;

        let seed = Seed {
            date: Utc::now(),
            population_count: 0,
            time_multiplier: 3600.0,
        };

        let mut app = App::new();
        let mut time = Time::default();

        time.update();

        app.insert_resource(seed).insert_resource(time);
        app.add_systems(Update, hunger_advance);

        let entity = app.world.spawn((Vitals::Alive, Hunger::new())).id();

        let mut time = app.world.resource_mut::<Time>();

        let last_update = time.last_update().unwrap();
        time.update_with_instant(last_update + Duration::from_secs(10));

        app.update();

        let hunger = app.world.get::<Hunger>(entity).unwrap();
        assert_eq!(hunger.0, 10.0);
    }
}
