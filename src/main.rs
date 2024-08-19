use bevy::prelude::*;
mod block;
mod camera;
mod click_shape;
mod collision;
mod craft;
mod craft_recipe;
mod equipment;
mod framerate;
mod gravity;
mod hit_test;
mod hotbar;
mod input;
mod inventory;
mod item;
mod item_container;
mod item_details;
mod item_dragging;
mod item_putting;
mod item_selecting;
mod item_sorting;
mod item_stats;
mod math;
mod player;
mod profiler;
mod random;
mod storage;
mod ui_hovered;
mod ui_parts;
mod ui_states;
mod velocity;
mod window;
mod workbench;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Craft".into(),
                resolution: (1920.0, 1080.0).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                resizable: false,
                #[cfg(target_arch = "wasm32")]
                canvas: Some("#bevy-canvas".into()),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            input::InputPlugin,
            random::RandomPlugin,
            window::WindowPlugin,
            profiler::ProfilerPlugin,
            framerate::FrameratePlugin,
            ui_hovered::UIHoveredPlugin,
            ui_states::UIStatusPlugin,
        ))
        .add_plugins((
            collision::CollisionPlugin,
            velocity::VelocityPlugin,
            gravity::GravityPlugin,
            click_shape::ClickShapePlugin,
        ))
        .add_plugins((
            item_container::ItemContainerPlugin,
            item_putting::ItemPuttingPlugin,
            item_dragging::ItemDraggingPlugin,
            item_selecting::ItemSelectingPlugin,
            item_details::ItemDetailsPlugin,
            item_sorting::ItemSortingPlugin,
            item_stats::ItemStatsPlugin,
        ))
        .add_plugins((
            hotbar::HotbarPlugin,
            inventory::InventoryPlugin,
            storage::StoragePlugin,
            craft::CraftPlugin,
            craft_recipe::CraftRecipePlugin,
            equipment::EquipmentPlugin,
            workbench::WorkbenchPlugin,
        ))
        .add_plugins((
            camera::CameraPlugin,
            player::PlatformerPlayerPlugin,
            block::BlockPlugin,
            item::ItemPlugin,
        ))
        .run();

    // TODO gimmicks
    // gear, water, wind, spike, fall floor, moving floor, seesaw, spring, tarzan, warp,
    // switch, door, ladder, rope, bomb, barrel, raft, magnetic, torch, ...

    // TODO tame, mount
    // TODO tools, potion
    // TODO enchant(job building), skill(combo building)
    // TODO housing, farmming, industry
    // TODO rogue dungeon
    // TODO enemy
    // TODO minimap, fast travel

    // TODO durability with free repair
    // TODO weapon
    // sword, arrow, drone, ...

    // TODO level generate
    // forest and ruins, submerged city,
    // magic fantasy or cyber punk or post apocalypse

    // TODO master data
    // TODO save and load
    // TODO multiplayer
    // TODO sound
    // TODO config (framerate, resolution, key bind, ...)

    // TODO visual making
    // water, lighting, post effect, background layers, ...
    // ui animation, character animation, atlas, tilemap, ...

    // TODO search macro
}
