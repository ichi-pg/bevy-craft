use bevy::prelude::*;
use crate::player::*;
use crate::level::*;

#[derive(Component)]
pub struct Grounded;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Positioned(Vec3);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Hits(Vec<Vec3>);

fn hit_test(
    mut players: Query<(&Transform, &mut Positioned, &mut Hits), With<Player>>,
    blocks: Query<&Transform, (With<Block>, Without<Player>)>,
) {
    for (player, mut positioned, mut hits) in &mut players {
        hits.clear();
        if player.translation.x == positioned.x && player.translation.y == positioned.y {
            continue;
        }
        for block in &blocks {
            let w = 128.0;
            let h = 128.0;
            let px = player.translation.x;
            let py = player.translation.y;
            let bx = block.translation.x;
            let by = block.translation.y;
            let x1 = bx - w;
            let x2 = bx + w;
            let y1 = by - h;
            let y2 = by + h;
            if px < x1 || x2 < px || py < y1 || y2 < py {
                continue;
            }
            hits.push(block.translation);
        }
        positioned.x = player.translation.x;
        positioned.y = player.translation.y;
    }
    // TODO chunk
}

fn slide_player(
    mut players: Query<(Entity, &mut Transform, &Hits), With<Player>>,
    mut commands: Commands,
) {
    for (entity, mut player, hits) in &mut players {
        for hit in hits.iter() {
            player.translation.y = hit.y + 128.0;
        }
        if hits.is_empty() {
            commands.entity(entity).remove::<Grounded>();
        } else {
            commands.entity(entity).insert(Grounded);
        }
    }
    // TODO slide
    // TODO bottom grounded
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            hit_test,
            slide_player,
        ));
    }
}
