use bevy::prelude::*;
use bevy_craft::*;

pub trait Stats {
    fn get(&self) -> f32;
    fn set(&mut self, stats: f32);
}

#[derive(Component, Stats)]
pub struct Health(pub f32);

#[derive(Component, Stats)]
pub struct MaxHealth(pub f32);

#[derive(Component, Stats)]
pub struct PickaxePower(pub f32);

#[derive(Component, Stats)]
pub struct AttackPower(pub f32);

#[derive(Component)]
pub struct AttackSpeed(pub f32);

#[derive(Component)]
pub struct MoveSpeed(pub f32);

#[derive(Component)]
pub struct JumpPower(pub f32);
