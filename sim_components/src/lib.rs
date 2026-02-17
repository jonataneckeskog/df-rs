use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Name(pub String);

#[derive(Component, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component, Debug)]
pub struct Greed(pub f32);

#[derive(Component)]
pub struct Actor;
