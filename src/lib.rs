#![allow(non_snake_case)]
#![allow(unused)]

mod delivery_enemy_table_generated;
mod delivery_raid_priority_generated;
mod raid;
mod raid_enemy_table_01_generated;
mod xoroshiro128plus;

pub use raid::*;
pub use xoroshiro128plus::*;

pub const TYPES: [&str; 18] = [
    "Normal", "Fighting", "Flying", "Poison", "Ground", "Rock", "Bug", "Ghost", "Steel", "Fire",
    "Water", "Grass", "Electric", "Psychic", "Ice", "Dragon", "Dark", "Fairy",
];
