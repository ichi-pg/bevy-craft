use crate::hit_test::*;
use crate::input::*;
use crate::item_dragging::*;
use crate::ui_forcus::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Clicked;

#[derive(Event)]
pub struct EmptyClicked {
    pub pos: Vec2,
}

fn click_shape(
    query: Query<(Entity, &Transform, &Shape)>,
    input: Res<Input>,
    mut commands: Commands,
    mut event_writer: EventWriter<EmptyClicked>,
) {
    if !input.left_click {
        return;
    }
    let mut found = false;
    for (entity, transform, shape) in &query {
        if point_test(input.world_cursor, transform.translation, *shape) {
            commands.entity(entity).insert(Clicked);
            found = true;
        }
    }
    if found {
        return;
    }
    event_writer.send(EmptyClicked {
        pos: input.world_cursor,
    });
    // TODO chunk or sweep or tree
}

pub struct ClickShapePlugin;

impl Plugin for ClickShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EmptyClicked>();
        app.add_systems(
            Update,
            click_shape
                .run_if(in_state(ItemDragged::None))
                .run_if(in_state(UIHobered::None)),
        );
    }
}
