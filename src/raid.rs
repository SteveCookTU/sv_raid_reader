use crate::delivery_enemy_table_generated::root_as_delivery_raid_enemy_table_array;
use crate::raid_enemy_table_01_generated::{root_as_raid_enemy_table_01_array, RaidRomType};
use crate::{delivery_enemy_table_generated, Filter, Xoroshiro128Plus, SPECIES, TYPES};
use std::fmt::{Display, Formatter};

pub const RAID_BLOCK_POINTER: [u64; 3] = [0x42FD560, 0x160, 0x50];

pub const RAID_BLOCK_SIZE: usize = 0xC80;

pub const DIFFICULTY_01_RAW: &[u8] = include_bytes!("../resources/difficulty_01");
pub const DIFFICULTY_02_RAW: &[u8] = include_bytes!("../resources/difficulty_02");
pub const DIFFICULTY_03_RAW: &[u8] = include_bytes!("../resources/difficulty_03");
pub const DIFFICULTY_04_RAW: &[u8] = include_bytes!("../resources/difficulty_04");
pub const DIFFICULTY_05_RAW: &[u8] = include_bytes!("../resources/difficulty_05");
pub const DIFFICULTY_06_RAW: &[u8] = include_bytes!("../resources/difficulty_06");
pub const DELIVERY_RAW: &[u8] = include_bytes!("../resources/delivery_enemy_array");

pub const AREAS: [&str; 22] = [
    "South Province (Area 1)",
    "", // Unused
    "", // Unused
    "South Province (Area 2)",
    "South Province (Area 5)",
    "South Province (Area 6)",
    "South Province (Area 5)",
    "South Province (Area 3)",
    "West Province (Area 1)",
    "Asado Desert",
    "West Province (Area 2)",
    "West Province (Area 3)",
    "Tagtree Thicket",
    "East Province (Area 3)",
    "East Province (Area 1)",
    "East Province (Area 2)",
    "Dalizapa Passage",
    "Casseroya Lake",
    "Glaseado Mountain",
    "North Province (Area 3)",
    "North Province (Area 1)",
    "North Province (Area 2)",
];

#[derive(Copy, Clone, Default, Debug)]
pub struct Raid {
    _unk1: u32,
    area: u32,
    _unk_3: u32,
    den: u32,
    seed: u32,
    ec: u32,
    pid: u32,
    pub shiny: bool,
    ivs: [u8; 6],
    pub tera_type: u8,
    star_level: u8,
    event: bool,
    species: u16,
}

impl Display for Raid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Area: {} Den: {}{}",
            AREAS[self.area as usize - 1],
            self.den,
            if self.event { " (Event)" } else { "" }
        )?;
        writeln!(f, "Seed: {:0>8X}", self.seed)?;
        writeln!(f, "Species: {}", SPECIES[self.species as usize])?;
        writeln!(f, "EC: {:0>8X}", self.ec)?;
        writeln!(f, "PID: {:0>8X}", self.pid)?;
        writeln!(f, "IVs: {:?}", self.ivs)?;
        writeln!(f, "Shiny: {}", self.shiny)?;
        write!(
            f,
            "{} Star {}-Tera",
            self.star_level, TYPES[self.tera_type as usize]
        )
    }
}

impl Raid {
    pub const SIZE: usize = 0x20;

    pub fn is_valid(&self) -> bool {
        self.seed != 0
    }

    pub fn passes_filter(&self, filter: &Filter) -> bool {
        if !self.shiny && filter.shiny {
            return false;
        }

        if !self.event && filter.event {
            return false;
        }

        if let Some(&tera_type) = filter.tera_type.as_ref() {
            if tera_type != self.tera_type {
                return false;
            }
        }

        if let Some(&star_level) = filter.star_level.as_ref() {
            if star_level != self.star_level {
                return false;
            }
        }

        if let Some(&species) = filter.species.as_ref() {
            if species != self.species {
                return false;
            }
        }

        true
    }
}

impl From<&[u8]> for Raid {
    fn from(data: &[u8]) -> Self {
        let _unk1 = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let area = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let _unk_3 = u32::from_le_bytes(data[8..12].try_into().unwrap());
        let den = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let seed = u32::from_le_bytes(data[16..20].try_into().unwrap());
        let special_type = u32::from_le_bytes(data[24..28].try_into().unwrap());
        let (six_star, event) = match special_type {
            1 => (true, false),
            2 => (false, true),
            _ => (false, false),
        };

        if seed == 0 {
            return Raid::default();
        }

        let mut rng = Xoroshiro128Plus::new(seed as u64);

        let mut star_level = if six_star {
            6
        } else if event {
            rng.next_masked(100);
            7
        } else {
            let star_level_rand = rng.next_masked(100);
            if star_level_rand <= 30 {
                3
            } else if star_level_rand <= 70 {
                4
            } else {
                5
            }
        };

        let species = if event {
            let table_array = root_as_delivery_raid_enemy_table_array(DELIVERY_RAW).unwrap();
            let sum = table_array
                .values()
                .iter()
                .filter_map(|s| {
                    if s.raidEnemyInfo().romVer()
                        == delivery_enemy_table_generated::RaidRomType::TYPE_A
                        || s.raidEnemyInfo().difficulty() < 3
                    {
                        None
                    } else {
                        Some(s.raidEnemyInfo().rate() as u64)
                    }
                })
                .sum::<u64>();
            let mut slot_rand = rng.next_masked(sum);
            let mut species = 0;
            for value in table_array.values().iter() {
                if value.raidEnemyInfo().romVer()
                    == delivery_enemy_table_generated::RaidRomType::TYPE_A
                    || value.raidEnemyInfo().difficulty() < 3
                {
                    continue;
                }
                if value.raidEnemyInfo().rate() as u64 > slot_rand {
                    star_level = value.raidEnemyInfo().difficulty() as u8;
                    species = value.raidEnemyInfo().bossPokePara().devId().0;
                    break;
                } else {
                    slot_rand -= value.raidEnemyInfo().rate() as u64;
                }
            }
            species
        } else {
            let table_array = match star_level {
                3 => root_as_raid_enemy_table_01_array(DIFFICULTY_03_RAW).unwrap(),
                4 => root_as_raid_enemy_table_01_array(DIFFICULTY_04_RAW).unwrap(),
                5 => root_as_raid_enemy_table_01_array(DIFFICULTY_05_RAW).unwrap(),
                _ => root_as_raid_enemy_table_01_array(DIFFICULTY_06_RAW).unwrap(),
            };
            let sum = table_array
                .values()
                .iter()
                .filter_map(|s| {
                    if s.raidEnemyInfo().romVer() == RaidRomType::TYPE_A {
                        None
                    } else {
                        Some(s.raidEnemyInfo().rate() as u64)
                    }
                })
                .sum::<u64>();
            let mut slot_rand = rng.next_masked(sum);
            let mut species = 0;
            for value in table_array.values().iter() {
                if value.raidEnemyInfo().romVer() == RaidRomType::TYPE_A {
                    continue;
                }
                if value.raidEnemyInfo().rate() as u64 > slot_rand {
                    species = value.raidEnemyInfo().bossPokePara().devId().0;
                    break;
                } else {
                    slot_rand -= value.raidEnemyInfo().rate() as u64;
                }
            }
            species
        };

        let mut rng = Xoroshiro128Plus::new(seed as u64);
        let tera_type = rng.next_masked(18);

        let mut rng = Xoroshiro128Plus::new(seed as u64);
        let ec = rng.next_masked(0xFFFFFFFF) as u32;
        let tidsid = rng.next_masked(0xFFFFFFFF) as u32;
        let pid = rng.next_masked(0xFFFFFFFF) as u32;
        let shiny =
            (((pid >> 16) ^ (pid & 0xFFFF)) >> 4) == (((tidsid >> 16) ^ (tidsid & 0xFFFF)) >> 4);
        let mut ivs = [0u8; 6];
        let flawless_ivs = star_level - 1;

        for _ in 0..flawless_ivs {
            let mut index;
            while {
                index = rng.next_masked(6) as usize;
                ivs[index] != 0
            } {}
            ivs[index] = 31;
        }

        for iv in &mut ivs {
            if *iv == 0 {
                *iv = rng.next_masked(32) as u8;
            }
        }

        Self {
            _unk1,
            area,
            _unk_3,
            den,
            seed,
            ec,
            pid,
            shiny,
            ivs,
            tera_type: tera_type as u8,
            star_level,
            event,
            species,
        }
    }
}