use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use std::time::Duration;

use sim_behaviors::SimulationPlugin;
use sim_components::{Actor, Greed, Health, Name};

fn main() {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(500))))
        .add_plugins(SimulationPlugin)
        .add_systems(Startup, setup_world)
        .run();
}

fn setup_world(mut commands: Commands) {
    // Spawn the Merchant
    commands.spawn((
        Actor,
        Name("Urist McMerchant".to_string()),
        Health {
            current: 100.0,
            max: 100.0,
        },
        Greed(0.95),
    ));

    // Spawn a Rock (has Name, but no Health or Greed)
    commands.spawn((
        Name("Granite Boulder".to_string()),
        // No health component = indestructible
    ));
}
