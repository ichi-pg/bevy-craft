use crate::block::*;
use crate::item::*;
use crate::item_id::*;
use bevy::math::I16Vec2;
use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Component)]
pub struct Surface;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct SurfaceSet(pub HashSet<I16Vec2>);

fn update_soil(
    query: Query<(&Children, &Transform, &ItemID), With<Surface>>,
    mut child_query: Query<&mut TextureAtlas, With<BlockSprite>>,
    surface_set: Res<SurfaceSet>,
) {
    for (children, transform, item_id) in &query {
        for child in children.iter() {
            let Ok(mut atlas) = child_query.get_mut(*child) else {
                todo!()
            };
            let point = (transform.translation.xy() * INVERTED_BLOCK_SIZE).as_i16vec2();
            let top_point = point + I16Vec2::Y;
            atlas.index = if surface_set.contains(&top_point) {
                match item_id.0 {
                    SOIL_ITEM_ID => 52,
                    _ => todo!(),
                }
            } else {
                match item_id.0 {
                    SOIL_ITEM_ID => 43,
                    _ => todo!(),
                }
            };
        }
    }
    // FIXME when chunk updated?
    // TODO freeze
    // TODO flower and grass
    // TODO farming
}

pub struct SurfacePlugin;

impl Plugin for SurfacePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SurfaceSet::default());
        app.add_systems(Update, update_soil);
    }
}
