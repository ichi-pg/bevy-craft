use crate::camera::*;
use bevy::diagnostic::*;
use bevy::prelude::*;
use iyes_perf_ui::entries::*;
use iyes_perf_ui::*;

#[derive(Resource)]
pub struct CollisionCounter(pub u64);

#[derive(Component)]
struct CollisionText;

fn spawn_profiler(camera_query: Query<Entity, With<PlayerCamera>>, mut commands: Commands) {
    for entity in &camera_query {
        commands.spawn((PerfUiBundle::default(), TargetCamera(entity)));
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
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
                    CollisionText,
                ));
            });
    }
}

fn sync_collision(
    mut query: Query<&mut Text, With<CollisionText>>,
    counter: Res<CollisionCounter>,
) {
    for mut text in &mut query {
        for section in &mut text.sections {
            section.value = format!("{}", counter.0);
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
            app.add_systems(Update, sync_collision);
        }
        app.insert_resource(CollisionCounter(0));
    }
    // TODO performance of each system
}
