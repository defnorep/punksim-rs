use bevy::ecs::component::Component;
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
pub struct Years(pub f32);

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
