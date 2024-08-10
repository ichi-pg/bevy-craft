use bevy::prelude::*;
mod block;
mod camera;
mod chest;
mod click_shape;
mod collision;
mod gravity;
mod hit_test;
mod hotbar;
mod input;
mod inventory;
mod item;
mod item_container;
mod item_dragging;
mod item_selecting;
mod player;
mod profiler;
mod random;
mod ui_forcus;
mod velocity;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Craft".into(),
                resolution: (1920.0, 1080.0).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            input::InputPlugin,
            random::RandomPlugin,
            // profiler::ProfilerPlugin,
        ))
        .add_plugins((
            collision::CollisionPlugin,
            velocity::VelocityPlugin,
            gravity::GravityPlugin,
            click_shape::ClickShapePlugin,
        ))
        .add_plugins((
            ui_forcus::UIForcusPlugin,
            item_container::ItemContainerPlugin,
            item_dragging::ItemDraggingPlugin,
            item_selecting::ItemSelectingPlugin,
            hotbar::HotbarPlugin,
            inventory::InventoryPlugin,
            chest::ChestPlugin,
        ))
        .add_plugins((
            camera::CameraPlugin,
            player::PlatformerPlayerPlugin,
            // player::TopDownPlayerPlugin,
            block::BlockPlugin,
            item::ItemPlugin,
        ))
        .run();

    // TODO gimmicks
    // gear, water, wind, spike, fall floor, moving floor, seesaw, spring, tarzan, warp,
    // switch, door, ladder, rope, bomb, barrel, raft, magnetic, torch, ...

    // TODO tame, mount
    // TODO weapon, tools, potion, equipment
    // TODO craft, enchant(job building), skill(combo building)
    // TODO farmming, industry
    // TODO rogue dungeon
    // TODO enemy
    // TODO minimap, fast travel

    // TODO level generate
    // forest and ruins, submerged city,
    // magic fantasy or cyber punk or post apocalypse

    // TODO master data
    // TODO save and load
    // TODO multiplayer
    // TODO sound
    // TODO config (fps, resolution, key config, ...)

    // TODO visual making
    // water, lighting, post effect, background layers, ...
    // ui animation, character animation, atlas, tilemap, ...
}
