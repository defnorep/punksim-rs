use super::Alive;
use bevy::prelude::*;
use core::fmt::Display;

const MIN_HUNGER: u32 = 0;
const MAX_HUNGER: u32 = 100;
const HUNGER_INTERVAL_HOURS: u32 = 4; // they should get hungry every 4 hours

#[derive(Component, Clone)]
pub struct Hunger(u32);

impl Hunger {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn increase(&mut self, amount: u32) {
        self.0 = (self.0 + amount).clamp(MIN_HUNGER, MAX_HUNGER);
    }

    pub fn reduce(&mut self, amount: u32) {
        if amount > self.0 {
            self.0 = 0; // can't go below 0 with unsigned integers when subtracting; rust will panic
        } else {
            self.0 = (self.0 - amount).clamp(MIN_HUNGER, MAX_HUNGER);
        }
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl Display for Hunger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}%", self.0)
    }
}

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
    time: Res<Time>,
    mut hunger_timer: ResMut<HungerTimer>,
    mut query: Query<(&Alive, &mut Hunger)>,
) {
    hunger_timer.0.tick(time.delta());

    if hunger_timer.0.finished() {
        for (alive, mut hunger) in query.iter_mut() {
            match alive {
                Alive::Alive => hunger.increase(1),
                _ => {}
            }
        }
    }
}

mod test {
    #[test]
    fn test_hunger() {
        use super::*;
        let mut hunger = Hunger::new();
        assert_eq!(hunger.value(), 0);
        hunger.increase(50);
        assert_eq!(hunger.value(), 50);
        hunger.increase(100);
        assert_eq!(hunger.value(), 100);
        hunger.reduce(50);
        assert_eq!(hunger.value(), 50);
        hunger.reduce(100);
        assert_eq!(hunger.value(), 0);
    }
}
