use bevy::prelude::*;

#[derive(Component)]
pub struct Planter;

fn update_plant(mut query: Query<&mut Sprite, With<Planter>>) {
    for sprite in &query {}
}

pub struct PlantPlugin;

impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_plant);
    }
}
