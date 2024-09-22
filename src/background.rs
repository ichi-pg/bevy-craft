use crate::block::*;
use crate::player::*;
use crate::random::*;
use crate::world_generator::*;
use crate::z_sort::*;
use bevy::prelude::*;
use bevy::render::texture::*;
use image::RgbaImage;
use noise::*;
use rand::RngCore;
use std::path::Path;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct BackgroundLayer;

#[derive(Component)]
struct BackgroundCloud;

const SCROLL_SPEED: f32 = 0.03;
const CLOUD_SPEED: f32 = 2.0;

const CLOUD_WIDTH: i16 = WORLD_WIDTH * 2;
const CLOUD_HEIGHT: i16 = ABOVE_GROUND_HEIGHT * 2;

const INVERTED_CLOUD_HEIGHT: f64 = 1.0 / CLOUD_HEIGHT as f64;

fn spawn_background(
    asset_server: Res<AssetServer>,
    mut random: ResMut<Random>,
    mut commands: Commands,
) {
    let seed = random.next_u32();
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
            for z in 1..=3 {
                let depth = 1.0 - (z - 1) as f32 * 0.1;
                let fbm = Fbm::<Perlin>::new(seed + z).set_frequency(0.01 * z as f64);
                let mut imgbuf = RgbaImage::new(CLOUD_WIDTH as u32, CLOUD_HEIGHT as u32);
                for y in 0..CLOUD_HEIGHT {
                    let altitude = y as f64 * INVERTED_CLOUD_HEIGHT;
                    for x in 0..CLOUD_WIDTH {
                        let noise = fbm.get([x as f64, y as f64]);
                        if noise < altitude - 0.1 * z as f64 + 0.2 {
                            continue;
                        }
                        let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
                        *pixel = image::Rgba([255, 255, 255, (255 as f32 * depth) as u8]);
                    }
                }
                let texture_name = &format!("cloud_{z}.png");
                if let Err(_) = imgbuf.save(Path::new("assets").join(texture_name)) {
                    todo!()
                }
                let texture = asset_server.load_with_settings(texture_name, |s: &mut _| {
                    *s = ImageLoaderSettings {
                        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                            mipmap_filter: ImageFilterMode::Linear,
                            ..default()
                        }),
                        ..default()
                    }
                });
                parent
                    .spawn((
                        SpatialBundle {
                            transform: Transform::from_xyz(0.0, 0.0, z as f32),
                            ..default()
                        },
                        BackgroundLayer,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(
                                        WORLD_WIDTH as f32 * BLOCK_SIZE * 0.5,
                                        ABOVE_GROUND_HEIGHT as f32 * BLOCK_SIZE * 0.5,
                                    )),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    0.0,
                                    ABOVE_GROUND_HEIGHT as f32 * 0.5,
                                    z as f32,
                                ),
                                texture,
                                ..default()
                            },
                            BackgroundCloud,
                        ));
                    });
            }
        });
    // TODO adjust cloud noise
    // TODO flex background width
}

fn scroll_layers(
    mut query: Query<&mut Transform, With<BackgroundLayer>>,
    player_query: Query<
        &Transform,
        (
            With<PlayerController>,
            Without<BackgroundLayer>,
            Changed<Transform>,
        ),
    >,
) {
    for player_transform in &player_query {
        for mut transform in &mut query {
            transform.translation.x =
                -player_transform.translation.x * transform.translation.z * SCROLL_SPEED;
            transform.translation.y =
                -player_transform.translation.y * transform.translation.z * SCROLL_SPEED;
        }
    }
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
        app.add_systems(
            Update,
            (trace_player::<Background>(1.0), scroll_layers, move_clouds),
        );
    }
}
