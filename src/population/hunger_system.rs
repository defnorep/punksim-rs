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
    let interval = (HUNGER_INTERVAL_HOURS * 60 * 60) / 100;
    commands.insert_resource(HungerTimer(Timer::from_seconds(
        interval as f32,
        TimerMode::Repeating,
    )));
}

pub fn hunger_advance(
    time: Res<Time>,
    mut hunger_timer: ResMut<HungerTimer>,
    mut query: Query<&mut Hunger>,
) {
    hunger_timer.0.tick(time.delta());

    if hunger_timer.0.finished() {
        for mut hunger in query.iter_mut() {
            hunger.increase(1);
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
    }
}
