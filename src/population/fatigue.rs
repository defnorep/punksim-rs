use super::Vitals;
use crate::data::Seed;
use bevy::prelude::*;
use core::fmt::Display;

const MIN_FATIGUE: f32 = 0.0;
const MAX_FATIGUE: f32 = f32::MAX;

#[derive(Component, Clone)]
pub struct Fatigue(f32);

impl Fatigue {
    pub fn new() -> Self {
        Self(0.0)
    }

    pub fn increase(&mut self, amount: f32) {
        self.0 = (self.0 + amount).clamp(MIN_FATIGUE, MAX_FATIGUE);
    }

    pub fn level(&self) -> FatigueLevel {
        match self.0 as u32 {
            0..=5 => FatigueLevel::Alert,
            16..=24 => FatigueLevel::Tired,
            _ => FatigueLevel::Exhausted,
        }
    }
}

impl Display for Fatigue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.level() {
            FatigueLevel::Alert => write!(f, "Alert"),
            FatigueLevel::Tired => write!(f, "Tired"),
            FatigueLevel::Exhausted => write!(f, "Exhausted"),
        }
    }
}

pub enum FatigueLevel {
    Alert,
    Tired,
    Exhausted,
}

pub fn fatigue_advance(
    time: Res<Time>,
    seed: Res<Seed>,
    mut query: Query<(&Vitals, &mut Fatigue)>,
) {
    for (alive, mut fatigue) in query.iter_mut() {
        if let Vitals::Alive = alive {
            fatigue.increase((seed.time_multiplier / 60.0 / 60.0) * time.delta_seconds());
        }
    }
}

mod test {
    #[test]
    fn test_fatigue_advance() {
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
        app.add_systems(Update, fatigue_advance);

        let entity = app.world.spawn((Vitals::Alive, Fatigue::new())).id();

        let mut time = app.world.resource_mut::<Time>();

        let last_update = time.last_update().unwrap();
        time.update_with_instant(last_update + Duration::from_secs(10));

        app.update();

        let fatigue = app.world.get::<Fatigue>(entity).unwrap();
        assert_eq!(fatigue.0, 10.0);
    }
}
