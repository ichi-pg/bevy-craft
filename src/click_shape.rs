use crate::hit_test::*;
use crate::input::*;
use crate::item_dragging::*;
use crate::ui_hovered::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct LeftClicked;

#[derive(Component)]
pub struct RightClicked;

#[derive(Event)]
pub struct EmptyClicked {
    pub pos: Vec2,
}

fn left_click(
    query: Query<(Entity, &Transform, &Shape)>,
    left_click: Res<LeftClick>,
    world_cursor: Res<WorldCursor>,
    mut commands: Commands,
) {
    if !left_click.pressed {
        return;
    }
    for (entity, transform, shape) in &query {
        if point_test(world_cursor.0, transform.translation, *shape) {
            commands.entity(entity).insert(LeftClicked);
            break;
        }
    }
    // TODO chunk or sweep or tree
}

fn right_click(
    query: Query<(Entity, &Transform, &Shape)>,
    right_click: Res<RightClick>,
    world_cursor: Res<WorldCursor>,
    mut commands: Commands,
    mut event_writer: EventWriter<EmptyClicked>,
) {
    if !right_click.just_pressed {
        return;
    }
    let mut found = false;
    for (entity, transform, shape) in &query {
        if point_test(world_cursor.0, transform.translation, *shape) {
            commands.entity(entity).insert(RightClicked);
            found = true;
            break;
        }
    }
    if found {
        return;
    }
    event_writer.send(EmptyClicked {
        pos: world_cursor.0,
    });
}

pub struct ClickShapePlugin;

impl Plugin for ClickShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EmptyClicked>();
        app.add_systems(
            Update,
            (left_click, right_click)
                .run_if(in_state(ItemDragged::None))
                .run_if(in_state(UIHovered::None)),
        );
    }
}
