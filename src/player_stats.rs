use crate::camera::*;
use crate::item_stats::{Health, MaxHealth};
use crate::player::PlayerController;
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

fn sync_health(
    player_query: Query<(&Health, &MaxHealth), (With<PlayerController>, Changed<Health>)>,
    mut query: Query<&mut Style, With<PlayerHealth>>,
) {
    for (health, max_health) in &player_query {
        for mut style in &mut query {
            style.width = Val::Percent(health.0 / max_health.0 * 100.0);
        }
    }
}

pub struct PlayerStatsPlugin;

impl Plugin for PlayerStatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stats);
        app.add_systems(Update, sync_health);
    }
    // TODO multi players stats
}
