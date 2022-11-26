#![allow(non_snake_case)]
#![allow(unused)]

mod delivery_enemy_table_generated;
mod delivery_raid_priority_generated;
mod filter;
mod raid;
mod raid_enemy_table_01_generated;
mod xoroshiro128plus;

pub use filter::*;
use lazy_static::lazy_static;
pub use raid::*;
pub use xoroshiro128plus::*;

const SPECIES_RAW: &str = include_str!("../resources/species.txt");
const TYPES_RAW: &str = include_str!("../resources/types.txt");

lazy_static! {
    pub static ref SPECIES: Vec<&'static str> = load_string_list(SPECIES_RAW);
    pub static ref TYPES: Vec<&'static str> = load_string_list(TYPES_RAW);
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
