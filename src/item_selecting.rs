use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectedItem(pub u8);

pub struct ItemSelectingPlugin;

impl Plugin for ItemSelectingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedItem(0));
    }
}
