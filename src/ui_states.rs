use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIStates {
    None,
    Inventory,
    Storage,
    Craft,
}

pub struct UIConstrollerPlugin;

impl Plugin for UIConstrollerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(UIStates::None);
    }
}
