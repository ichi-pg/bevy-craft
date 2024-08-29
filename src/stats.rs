use crate::item_stats::*;
use bevy::prelude::*;

pub trait Stats {
    fn get(&self) -> f32;
    fn set(&mut self, stats: f32);
    fn get_item_stats(stats: &ItemStats) -> f32;
}

macro_rules! define_stats {
    ( $( ( $x:tt, $y:tt) ),* ) => {
        $(
            #[derive(Component)]
            pub struct $x(pub f32);

            impl Stats for $x {
                fn get(&self) -> f32 {
                    self.0
                }
                fn set(&mut self, stats: f32) {
                    self.0 = stats;
                }
                fn get_item_stats(stats: &ItemStats) -> f32 {
                    stats.$y
                }
            }
        )*
    };
}

define_stats!(
    (Health, health),
    (MaxHealth, max_health),
    (PickaxePower, pickaxe_power),
    (AttackPower, attack_power),
    (AttackSpeed, attack_speed),
    (MoveSpeed, move_speed),
    (JumpPower, jump_power)
);
