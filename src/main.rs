// use avian2d::prelude::*;
use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;
mod avian_level;
mod avian_player;
mod camera;
mod collision;
mod hit_test;
mod input;
mod item;
mod level;
mod player;
mod rapier_level;
mod rapier_player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Craft".into(),
                    resolution: (1920.0, 1080.0).into(),
                    ..default()
                }),
                ..default()
            }),
            // PhysicsPlugins::default(),
            // RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            // RapierDebugRenderPlugin::default(),
            input::InputPlugin,
            collision::CollisionPlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            level::LevelPlugin,
        ))
        .run();

    // TODO gimmicks
    // gear, water, wind, spike, fall floor, moving floor, seesaw, spring, tarzan, warp,
    // switch, door, ladder, rope, bomb, barrel, raft, magnetic, torch, ...

    // TODO tame, mount
    // TODO weapon, tools, potion
    // TODO hotbar, inventory, chest
    // TODO craft, enchant, skill
    // TODO level generate
    // TODO farmming, industry
    // TODO rogue dungeon
    // TODO enemy

    // TODO master data
    // TODO save and load
    // TODO multiplayer
    // TODO sound

    // TODO visual making
    // water, lighting, post effect, background layers, ...
}
