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
struct BackgroundObject;

const SCROLL_SPEED: f32 = 0.03;

fn build_clouds(parent: &mut ChildBuilder, mut random: ChaCha8Rng, z: f32, y1: u32, y2: u32) {
    parent
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, z),
                ..default()
            },
            BackgroundObject,
        ))
        .with_children(|parent| {
            for x in -20..20 {
                for y in y1..y2 {
                    if (y + 5).pow2() > random.next_u32() % 500 {
                        let y = y + random.next_u32() % 3;
                        parent.spawn((SpriteBundle {
                            sprite: Sprite {
                                color: Color::WHITE,
                                custom_size: Some(Vec2::new((y * 30) as f32, (y * 10) as f32)),
                                ..default()
                            },
                            transform: Transform::from_xyz(96.0 * x as f32, 54.0 * y as f32, 0.0),
                            ..default()
                        },));
                    }
                }
            }
        });
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

fn move_background(
    mut background_query: Query<&mut Transform, With<Background>>,
    mut query: Query<&mut Transform, (With<BackgroundObject>, Without<Background>)>,
    player_query: Query<
        &Transform,
        (
            With<PlayerController>,
            Without<Background>,
            Without<BackgroundObject>,
            Changed<Transform>,
        ),
    >,
) {
    for player_transform in &player_query {
        for mut transform in &mut background_query {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        }
        for mut transform in &mut query {
            transform.translation.x =
                -player_transform.translation.x * transform.translation.z * SCROLL_SPEED;
            transform.translation.y =
                -player_transform.translation.y * transform.translation.z * SCROLL_SPEED;
        }
    }
    // TODO common trace system
    // TODO change texture with environment
}

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_background);
        app.add_systems(Update, move_background);
    }
}
