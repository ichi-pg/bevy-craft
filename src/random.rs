use bevy::prelude::*;
use rand::*;
use rand_chacha::*;
use web_time::*;

#[derive(Resource, Deref, DerefMut)]
pub struct Random(pub ChaCha8Rng);

pub struct RandomPlugin;

impl Plugin for RandomPlugin {
    fn build(&self, app: &mut App) {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => {
                app.insert_resource(Random(ChaCha8Rng::seed_from_u64(duration.as_secs())));
            }
            Err(_) => todo!(),
        }
    }
}
