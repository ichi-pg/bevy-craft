use crate::hit_test::*;
use crate::input::*;
use crate::item_container::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Clicked;

#[derive(Event)]
pub struct EmptyClicked {
    pub pos: Vec2,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum UIHobered {
    None,
    Hovered,
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

fn focus_ui(
    query: Query<&Interaction, (With<UI>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<UIHobered>>,
) {
    for intersection in &query {
        match intersection {
            Interaction::Pressed => continue,
            Interaction::Hovered => next_state.set(UIHobered::Hovered),
            Interaction::None => next_state.set(UIHobered::None),
        }
    }
}

pub struct ClickShapePlugin;

impl Plugin for ClickShapePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(UIHobered::None);
        app.add_event::<EmptyClicked>();
        app.add_systems(
            Update,
            (click_shape.run_if(in_state(UIHobered::None)), focus_ui),
        );
    }
}
