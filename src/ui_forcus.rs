use bevy::prelude::*;

#[derive(Component)]
pub struct UI;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIHobered {
    None,
    Hovered,
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

pub struct UIForcusPlugin;

impl Plugin for UIForcusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(UIHobered::None);
        app.add_systems(Update, focus_ui);
    }
}
