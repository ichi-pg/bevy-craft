use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct ItemID(pub i32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct ItemAmount(pub i32);
