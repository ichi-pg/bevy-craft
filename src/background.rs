use crate::math::Pow2;
use crate::player::*;
use crate::random::*;
use crate::z_sort::*;
use bevy::prelude::*;
use rand::RngCore;
use rand_chacha::ChaCha8Rng;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct BackgroundLayer;

#[derive(Component)]
struct BackgroundCloud;

const SCROLL_SPEED: f32 = 0.03;
const CLOUD_SPEED: f32 = 2.0;

fn build_clouds(parent: &mut ChildBuilder, mut random: ChaCha8Rng, z: f32, y1: u32, y2: u32) {
    parent
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, z),
                ..default()
            },
            BackgroundLayer,
        ))
        .with_children(|parent| {
            for x in -20..20 {
                for y in y1..y2 {
                    if (y + 5).pow2() > random.next_u32() % 500 {
                        let y = y + random.next_u32() % 3;
                        parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::WHITE,
                                    custom_size: Some(Vec2::new((y * 30) as f32, (y * 10) as f32)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(96.0 * x as f32, 54.0 * y as f32, z),
                                ..default()
                            },
                            BackgroundCloud,
                        ));
                    }
                }
            }
        });
    // TODO merged texture
}

fn spawn_background(mut commands: Commands, random: Res<Random>) {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.3, 0.6, 0.9),
                    custom_size: Some(Vec2::new(1920.0, 1080.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, BACKGROUND_Z),
                ..default()
            },
            Background,
        ))
        .with_children(|parent| {
            build_clouds(parent, random.0.clone(), 1.0, 1, 4);
            build_clouds(parent, random.0.clone(), 2.0, 5, 8);
            build_clouds(parent, random.0.clone(), 3.0, 9, 12);
        });
}

fn trace_player(
    mut query: Query<&mut Transform, With<Background>>,
    mut layer_query: Query<&mut Transform, (With<BackgroundLayer>, Without<Background>)>,
    player_query: Query<
        &Transform,
        (
            With<PlayerController>,
            Without<Background>,
            Without<BackgroundLayer>,
            Without<BackgroundCloud>,
            Changed<Transform>,
        ),
    >,
) {
    for player_transform in &player_query {
        for mut transform in &mut query {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
        for mut transform in &mut layer_query {
            transform.translation.x =
                -player_transform.translation.x * transform.translation.z * SCROLL_SPEED;
            transform.translation.y =
                -player_transform.translation.y * transform.translation.z * SCROLL_SPEED;
        }
    }
    // TODO common trace system
    // TODO change texture with environment
}

fn move_clouds(mut query: Query<&mut Transform, With<BackgroundCloud>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.x += transform.translation.z * time.delta_seconds() * CLOUD_SPEED;
    }
    // TODO repeat
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_background);
        app.add_systems(Update, (trace_player, move_clouds));
    }
}
