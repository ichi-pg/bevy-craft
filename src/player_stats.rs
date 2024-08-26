use crate::camera::*;
use crate::player::*;
use crate::stats::*;
use crate::ui_parts::*;
use bevy::prelude::*;

#[derive(Component, Default)]
struct PlayerHealth;

fn spawn_stats(camera_query: Query<Entity, With<PlayerCamera>>, mut commands: Commands) {
    for entity in &camera_query {
        commands.build_screen(
            entity,
            0,
            0,
            JustifyContent::Start,
            AlignItems::Start,
            |parent| {
                build_progress_bar::<PlayerHealth>(parent, Color::srgb(0.9, 0.3, 0.3));
            },
        );
    }
}

pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stats);
        app.add_systems(
            Update,
            sync_progress_bar::<Health, MaxHealth, PlayerController, PlayerHealth>,
        );
    }
    // TODO multi players stats
}
