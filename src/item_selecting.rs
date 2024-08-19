use crate::input::*;
use crate::item_node::*;
use crate::math::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SelectedItem(pub u8);

fn change_wheel(mut selected: ResMut<SelectedItem>, wheel: Res<Wheel>) {
    selected.0 = (selected.0 as i8 - wheel.0.signum()).repeat(0, 9) as u8;
}

fn change_number(mut selected: ResMut<SelectedItem>, key_num: Res<KeyNum>) {
    for (index, pressed) in key_num.iter().enumerate() {
        if *pressed {
            selected.0 = index as u8;
        }
    }
}

fn sync_selected(
    mut query: Query<(&ItemIndex, &mut Visibility), With<Text>>,
    selected: Res<SelectedItem>,
) {
    for (index, mut visibility) in &mut query {
        *visibility = if index.0 == selected.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        }
    }
}

pub struct ItemSelectingPlugin;

impl Plugin for ItemSelectingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedItem(0));
        app.add_systems(Update, (change_wheel, change_number, sync_selected));
    }
}
