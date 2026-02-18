use bevy::prelude::*;
use sim_components::{Greed, Health, Name};

// Group systems into a Plugin for cleaner main.rs
pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (survival_system, greed_logging_system));
    }
}

// System: Apply logic to specific components
fn survival_system(mut query: Query<(&Name, &mut Health)>) {
    for (name, mut health) in query.iter_mut() {
        health.current -= 0.1; // Decay health over time
        if health.current <= 0.0 {
            println!("{} has perished!", name.0);
        }
    }
}

// System: Filter only greedy entities
fn greed_logging_system(query: Query<(&Name, &Greed)>) {
    for (name, greed) in query.iter() {
        if greed.0 > 0.8 {
            println!("{} is scheming...", name.0);
        }
    }
}
