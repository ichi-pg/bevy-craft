use crate::input::*;
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIStates {
    None,
    Inventory,
    Storage,
    Craft,
}

fn close_ui(input: Res<Input>, mut next_state: ResMut<NextState<UIStates>>) {
    if !input.escape {
        return;
    }
    next_state.set(UIStates::None);
}

pub struct UIStatusPlugin;

impl Plugin for UIStatusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(UIStates::None);
        app.add_systems(Update, close_ui);
    }
}
