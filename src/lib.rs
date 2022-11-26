#![allow(non_snake_case)]
#![allow(unused)]

mod delivery_enemy_table_generated;
mod delivery_raid_priority_generated;
mod filter;
mod personal_info;
mod personal_info_sv;
pub mod personal_table;
mod raid;
mod raid_enemy_table_01_generated;
mod xoroshiro128plus;

pub use personal_info::*;
pub use personal_info_sv::*;

pub use filter::*;
use lazy_static::lazy_static;
pub use raid::*;
pub use xoroshiro128plus::*;

const SPECIES_RAW: &str = include_str!("../resources/species.txt");
const TYPES_RAW: &str = include_str!("../resources/types.txt");
const MOVES_RAW: &str = include_str!("../resources/moves.txt");
const ABILITIES_RAW: &str = include_str!("../resources/abilities.txt");
const NATURES_RAW: &str = include_str!("../resources/natures.txt");

lazy_static! {
    pub static ref SPECIES: Vec<&'static str> = load_string_list(SPECIES_RAW);
    pub static ref TYPES: Vec<&'static str> = load_string_list(TYPES_RAW);
    pub static ref MOVES: Vec<&'static str> = load_string_list(MOVES_RAW);
    pub static ref ABILITIES: Vec<&'static str> = load_string_list(ABILITIES_RAW);
    pub static ref NATURES: Vec<&'static str> = load_string_list(NATURES_RAW);
}

fn load_string_list(list: &str) -> Vec<&str> {
    list.split('\n')
        .map(|s| {
            if s.is_empty() {
                s
            } else if s.as_bytes()[s.len() - 1] == b'\r' {
                &s[..(s.len() - 1)]
            } else {
                s
            }
        })
        .collect()
}
