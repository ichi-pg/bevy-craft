// use avian2d::prelude::*;
use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;
mod avian_level;
mod avian_player;
mod block;
mod camera;
mod click_shape;
mod collision;
mod grounded;
mod hit_test;
mod hotbar;
mod input;
mod inventory;
mod item;
mod item_container;
mod player;
mod profiler;
mod random;
mod rapier_level;
mod rapier_player;
mod rigid_body;

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
            player::PlayerPlugin,
            camera::CameraPlugin,
            block::BlockPlugin,
            rigid_body::RigitBodyPlugin,
            item::ItemPlugin,
            grounded::GroundedPlugin,
            hotbar::HotbarPlugin,
            random::RandomPlugin,
            click_shape::ClickShapePlugin,
            item_container::ItemContainerPlugin,
            inventory::InventoryPlugin,
            // profiler::ProfilerPlugin,
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
