use crate::data::Seed;

use super::Vitals;
use bevy::prelude::*;
use core::fmt::Display;

const MIN_HUNGER: u32 = 0;
const MAX_HUNGER: u32 = 100;
const HUNGER_INTERVAL_HOURS: u32 = 8; // they should get hungry every 8 hours

#[derive(Component, Clone)]
pub struct Hunger(u32);

impl Hunger {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn increase(&mut self, amount: u32) {
        self.0 = (self.0 + amount).clamp(MIN_HUNGER, MAX_HUNGER);
    }

    pub fn level(&self) -> HungerLevel {
        match self.0 {
            80..=100 => HungerLevel::Starving,
            50..=79 => HungerLevel::Hungry,
            _ => HungerLevel::Satisfied,
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

#[derive(Component)]
pub struct Starving;

#[derive(Resource)]
pub struct HungerTimer(Timer);

pub fn hunger_setup(mut commands: Commands) {
    let interval = (HUNGER_INTERVAL_HOURS * 60 * 60) / MAX_HUNGER;
    commands.insert_resource(HungerTimer(Timer::from_seconds(
        interval as f32,
        TimerMode::Repeating,
    )));
}

pub fn hunger_advance(
    mut commands: Commands,
    time: Res<Time>,
    seed: Res<Seed>,
    mut hunger_timer: ResMut<HungerTimer>,
    mut query: Query<(Entity, &Vitals, &mut Hunger)>,
) {
    hunger_timer
        .0
        .tick(time.delta().mul_f32(seed.time_multiplier));

    if hunger_timer.0.finished() {
        for (entity, alive, mut hunger) in query.iter_mut() {
            if let Vitals::Alive = alive {
                hunger.increase(1);

                if let HungerLevel::Starving = hunger.level() {
                    commands.entity(entity).insert(Starving);
                }
            }
        }
    }
}

mod test {
    #[test]
    fn test_hunger() {
        use super::*;
        let mut hunger = Hunger::new();
        assert_eq!(hunger.0, 0);
        hunger.increase(50);
        assert_eq!(hunger.0, 50);
        hunger.increase(100);
        assert_eq!(hunger.0, 100);
    }
}
