use bevy::prelude::*;

#[derive(Component)]
pub struct UI;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIHovered {
    None,
    Hovered,
}

fn change_hovered(
    query: Query<&Interaction, (With<UI>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<UIHovered>>,
) {
    for intersection in &query {
        match intersection {
            Interaction::Pressed => continue,
            Interaction::Hovered => next_state.set(UIHovered::Hovered),
            Interaction::None => next_state.set(UIHovered::None),
        }
    }
}

pub struct UIHoveredPlugin;

impl Plugin for UIHoveredPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(UIHovered::None);
        app.add_systems(PreUpdate, change_hovered);
    }
    // TODO hovered component?
}
