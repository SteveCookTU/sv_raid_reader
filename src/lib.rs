#![allow(non_snake_case)]
#![allow(unused)]

pub mod delivery_enemy_table_generated;
pub mod delivery_raid_priority_generated;
mod filter;
mod item_list;
mod personal_info;
mod personal_info_sv;
pub mod personal_table;
mod raid;
mod raid_encounter;
pub mod raid_enemy_table_01_generated;
pub mod raid_fixed_reward_item_generated;
pub mod raid_lottery_reward_item_generated;
mod xoroshiro128plus;

pub use item_list::*;
pub use personal_info::*;
pub use personal_info_sv::*;
pub use raid_encounter::*;

pub use filter::*;
use lazy_static::lazy_static;
pub use raid::*;
pub use xoroshiro128plus::*;

const SPECIES_RAW: &str = include_str!("../resources/species.txt");
const TYPES_RAW: &str = include_str!("../resources/types.txt");
const MOVES_RAW: &str = include_str!("../resources/moves.txt");
const ABILITIES_RAW: &str = include_str!("../resources/abilities.txt");
const NATURES_RAW: &str = include_str!("../resources/natures.txt");
const ITEMS_RAW: &str = include_str!("../resources/items.txt");
pub const GENDER_SYMBOLS: [char; 3] = ['♂', '♀', '-'];

lazy_static! {
    pub static ref SPECIES: Vec<&'static str> = load_string_list(SPECIES_RAW);
    pub static ref TYPES: Vec<&'static str> = load_string_list(TYPES_RAW);
    pub static ref MOVES: Vec<&'static str> = load_string_list(MOVES_RAW);
    pub static ref ABILITIES: Vec<&'static str> = load_string_list(ABILITIES_RAW);
    pub static ref NATURES: Vec<&'static str> = load_string_list(NATURES_RAW);
    pub static ref ITEMS: Vec<&'static str> = load_string_list(ITEMS_RAW);
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum GameVersion {
    Scarlet,
    Violet,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum GameProgress {
    None,
    Badge3,
    Badge7,
    Credits,
    PostGame,
}

impl GameProgress {
    pub fn get_min_stars(&self) -> i32 {
        match self {
            GameProgress::None => 1,
            GameProgress::Badge3 => 1,
            GameProgress::Badge7 => 1,
            GameProgress::Credits => 3,
            GameProgress::PostGame => 3,
        }
    }

    pub fn get_max_stars(&self) -> i32 {
        match self {
            GameProgress::None => 2,
            GameProgress::Badge3 => 3,
            GameProgress::Badge7 => 4,
            GameProgress::Credits => 5,
            GameProgress::PostGame => 5,
        }
    }

    pub fn get_thresholds(&self) -> &'static [u8] {
        match self {
            GameProgress::None => &[80, 20],
            GameProgress::Badge3 => &[30, 40, 30],
            GameProgress::Badge7 => &[20, 20, 30, 30],
            GameProgress::Credits => &[40, 35, 25],
            GameProgress::PostGame => &[30, 40, 30],
        }
    }
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

pub fn read_raids(
    data: &[u8],
    game: GameVersion,
    filter: Filter,
    progress: GameProgress,
) -> Vec<Raid> {
    let mut raids = Vec::with_capacity(69);
    for offset in (0..RAID_BLOCK_SIZE).step_by(Raid::SIZE) {
        let raid_data = &data[offset..(offset + Raid::SIZE)];
        let raid: Raid = (raid_data, game, progress).into();
        if raid.is_valid() && raid.passes_filter(&filter) {
            raids.push(raid);
        }
    }
    raids
}
