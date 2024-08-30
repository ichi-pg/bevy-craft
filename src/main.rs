use bevy::prelude::*;
mod block;
mod camera;
mod click_shape;
mod collision;
mod craft;
mod craft_recipe;
mod enemy;
mod equipment;
mod framerate;
mod gravity;
mod hit_test;
mod hotbar;
mod input;
mod inventory;
mod item;
mod item_details;
mod item_dragging;
mod item_node;
mod item_putting;
mod item_selecting;
mod item_sorting;
mod item_stats;
mod math;
mod minimap;
mod mob_chase;
mod mob_jump_attack;
mod mob_patrol;
mod mob_stroll;
mod mob_walk;
mod player;
mod player_melee;
mod player_stats;
mod profiler;
mod random;
mod stats;
mod storage;
mod ui_hovered;
mod ui_parts;
mod ui_states;
mod velocity;
mod window;
mod workbench;
mod z_sort;

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
            camera::CameraPlugin,
            input::InputPlugin,
            random::RandomPlugin,
            window::WindowPlugin,
            profiler::ProfilerPlugin,
            framerate::FrameratePlugin,
            ui_hovered::UIHoveredPlugin,
            ui_states::UIStatusPlugin,
            collision::CollisionPlugin,
            velocity::VelocityPlugin,
            gravity::GravityPlugin,
            click_shape::ClickShapePlugin,
        ))
        .add_plugins((
            item_node::ItemNodePlugin,
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
            minimap::MinimapPlugin,
            player_stats::PlayerStatsPlugin,
        ))
        .add_plugins((
            player::PlayerPlugin,
            player_melee::PlayerMeleePlugin,
            block::BlockPlugin,
            item::ItemPlugin,
            enemy::EnemyPlugin,
            mob_walk::MobWalkPlugin,
            mob_stroll::MobStrollPlugin,
            mob_patrol::MobPatrolPlugin,
            mob_chase::MobChasePlugin,
            mob_jump_attack::MobJumpAttackPlugin,
        ))
        .run();

    // TODO gimmicks
    // gear, water, wind, spike, fall floor, moving floor, seesaw, spring, tarzan, warp,
    // switch, door, ladder, rope, bomb, barrel, raft, magnetic, torch, ...

    // TODO tame, mount
    // TODO potion
    // TODO enchant(job building), skill(combo building)
    // TODO housing, farmming, industry, fishing, pastoralism
    // TODO rogue dungeon
    // TODO boss

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
