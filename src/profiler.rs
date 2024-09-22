use crate::block::*;
use crate::camera::*;
use crate::chunk::*;
use crate::math::*;
use crate::player::*;
use crate::velocity::*;
use bevy::diagnostic::*;
use bevy::prelude::*;
use iyes_perf_ui::entries::*;
use iyes_perf_ui::*;

#[derive(Resource)]
pub struct CollisionCounter(pub u64);

#[derive(Component)]
struct CollisionInfo;

#[derive(Component)]
struct PlayerInfo;

fn spawn_profiler(camera_query: Query<Entity, With<PlayerCamera>>, mut commands: Commands) {
    for entity in &camera_query {
        commands.spawn((PerfUiBundle::default(), TargetCamera(entity)));
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::Start,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                },
                TargetCamera(entity),
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle {
                        text: Text::from_section("", TextStyle::default()),
                        ..default()
                    },
                    CollisionInfo,
                ));
                parent.spawn((
                    TextBundle {
                        text: Text::from_section("", TextStyle::default()),
                        ..default()
                    },
                    PlayerInfo,
                ));
            });
    }
}

fn sync_collision(
    mut query: Query<&mut Text, With<CollisionInfo>>,
    mut counter: ResMut<CollisionCounter>,
) {
    for mut text in &mut query {
        for section in &mut text.sections {
            section.value = format!("Collision {}", counter.0);
        }
    }
    counter.0 = 0;
}

fn sync_player(
    mut query: Query<&mut Text, With<PlayerInfo>>,
    player_query: Query<(&Transform, &Velocity2), With<PlayerController>>,
    chunk_point: Res<ChunkPoint>,
) {
    for (transform, velocity) in &player_query {
        for mut text in &mut query {
            for section in &mut text.sections {
                section.value = format!(
                    "Position:{} Velocity:{} Chunk:{}",
                    (transform.translation * INVERTED_BLOCK_SIZE).to_i32vec2(),
                    velocity.to_i32vec2(),
                    chunk_point.to_i32vec2(),
                );
            }
        }
    }
}

pub struct ProfilerPlugin;

impl Plugin for ProfilerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(any(debug_assertions, target_arch = "wasm32"))]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin,
                EntityCountDiagnosticsPlugin,
                SystemInformationDiagnosticsPlugin,
                PerfUiPlugin,
            ));
            app.add_systems(Startup, spawn_profiler);
            app.add_systems(Update, (sync_collision, sync_player));
        }
        app.insert_resource(CollisionCounter(0));
    }
    // TODO performance of each system
}
