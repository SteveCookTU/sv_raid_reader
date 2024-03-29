use crate::delivery_enemy_table_generated::root_as_delivery_raid_enemy_table_array;
use crate::raid_enemy_table_01_generated::{root_as_raid_enemy_table_01_array, RaidRomType};
use crate::{
    delivery_enemy_table_generated, personal_table, Filter, GameProgress, GameVersion,
    PersonalInfo, RaidEncounter, Xoroshiro128Plus, ABILITIES, GENDER_SYMBOLS, NATURES, SPECIES,
    TYPES,
};
use lazy_static::lazy_static;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;

//Deliver Group Priority [[[[main+43A7798]+08]+2C0]+10]+88
pub const RAID_BLOCK_POINTER: [u64; 3] = [0x43A77C8, 0x160, 0x50];

pub const RAID_BLOCK_SIZE: usize = 0xC80;

pub const DIFFICULTY_01_RAW: &[u8] = include_bytes!("../resources/difficulty_01");
pub const DIFFICULTY_02_RAW: &[u8] = include_bytes!("../resources/difficulty_02");
pub const DIFFICULTY_03_RAW: &[u8] = include_bytes!("../resources/difficulty_03");
pub const DIFFICULTY_04_RAW: &[u8] = include_bytes!("../resources/difficulty_04");
pub const DIFFICULTY_05_RAW: &[u8] = include_bytes!("../resources/difficulty_05");
pub const DIFFICULTY_06_RAW: &[u8] = include_bytes!("../resources/difficulty_06");
pub const SU1_DIFFICULTY_01_RAW: &[u8] = include_bytes!("../resources/su1_difficulty_01");
pub const SU1_DIFFICULTY_02_RAW: &[u8] = include_bytes!("../resources/su1_difficulty_02");
pub const SU1_DIFFICULTY_03_RAW: &[u8] = include_bytes!("../resources/su1_difficulty_03");
pub const SU1_DIFFICULTY_04_RAW: &[u8] = include_bytes!("../resources/su1_difficulty_04");
pub const SU1_DIFFICULTY_05_RAW: &[u8] = include_bytes!("../resources/su1_difficulty_05");
pub const SU1_DIFFICULTY_06_RAW: &[u8] = include_bytes!("../resources/su1_difficulty_06");
pub const SU2_DIFFICULTY_01_RAW: &[u8] = include_bytes!("../resources/su2_difficulty_01");
pub const SU2_DIFFICULTY_02_RAW: &[u8] = include_bytes!("../resources/su2_difficulty_02");
pub const SU2_DIFFICULTY_03_RAW: &[u8] = include_bytes!("../resources/su2_difficulty_03");
pub const SU2_DIFFICULTY_04_RAW: &[u8] = include_bytes!("../resources/su2_difficulty_04");
pub const SU2_DIFFICULTY_05_RAW: &[u8] = include_bytes!("../resources/su2_difficulty_05");
pub const SU2_DIFFICULTY_06_RAW: &[u8] = include_bytes!("../resources/su2_difficulty_06");
pub const DELIVERY_RAW: &[u8] = include_bytes!("../resources/delivery_enemy_array");

lazy_static! {
    pub static ref DIFFICULTY_01: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(DIFFICULTY_01_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref DIFFICULTY_02: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(DIFFICULTY_02_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref DIFFICULTY_03: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(DIFFICULTY_03_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref DIFFICULTY_04: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(DIFFICULTY_04_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref DIFFICULTY_05: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(DIFFICULTY_05_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref DIFFICULTY_06: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(DIFFICULTY_06_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU1_DIFFICULTY_01: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU1_DIFFICULTY_01_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU1_DIFFICULTY_02: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU1_DIFFICULTY_02_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU1_DIFFICULTY_03: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU1_DIFFICULTY_03_RAW)
            .unwrap()
            .values()
            .into_iter().enumerate()
            .map(|(i, v)| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU1_DIFFICULTY_04: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU1_DIFFICULTY_04_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU1_DIFFICULTY_05: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU1_DIFFICULTY_05_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU1_DIFFICULTY_06: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU1_DIFFICULTY_06_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU2_DIFFICULTY_01: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU2_DIFFICULTY_01_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU2_DIFFICULTY_02: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU2_DIFFICULTY_02_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU2_DIFFICULTY_03: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU2_DIFFICULTY_03_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU2_DIFFICULTY_04: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU2_DIFFICULTY_04_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU2_DIFFICULTY_05: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU2_DIFFICULTY_05_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
    pub static ref SU2_DIFFICULTY_06: Vec<RaidEncounter> =
        root_as_raid_enemy_table_01_array(SU2_DIFFICULTY_06_RAW)
            .unwrap()
            .values()
            .into_iter()
            .map(|v| v.raidEnemyInfo().into())
            .collect();
}

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
    pub area: u32,
    _unk_3: u32,
    pub den: u32,
    pub seed: u32,
    pub tera_type: u8,
    pub star_level: u8,
    pub event: bool,
    pub species: u16,
    pub form: u16,
    pub pokemon: Pokemon,
}

impl Display for Raid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let personal = personal_table::SV.get_form_entry(self.species as usize, self.form as usize);
        let ability = personal
            .get_ability_index((self.pokemon.ability - 1) as usize)
            .unwrap();
        writeln!(
            f,
            "Area: {} Den: {}{}",
            AREAS[self.area as usize - 1],
            self.den,
            if self.event { " (Event)" } else { "" }
        )?;
        writeln!(f, "Seed: {:0>8X}", self.seed)?;
        writeln!(
            f,
            "Species: {} {}",
            SPECIES[self.species as usize].trim(),
            GENDER_SYMBOLS[self.pokemon.gender as usize]
        )?;
        writeln!(f, "EC: {:0>8X}", self.pokemon.ec)?;
        writeln!(f, "PID: {:0>8X}", self.pokemon.pid)?;
        writeln!(f, "Shiny: {}", self.pokemon.shiny)?;
        writeln!(f, "IVs: {:?}", self.pokemon.ivs)?;
        writeln!(
            f,
            "Ability: {} Nature: {}",
            ABILITIES[ability].trim(),
            NATURES[self.pokemon.nature as usize].trim()
        )?;
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
        if !self.pokemon.shiny && filter.shiny {
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

impl From<(&[u8], GameVersion, GameProgress)> for Raid {
    fn from(data: (&[u8], GameVersion, GameProgress)) -> Self {
        let _unk1 = u32::from_le_bytes(data.0[0..4].try_into().unwrap());
        let area = u32::from_le_bytes(data.0[4..8].try_into().unwrap());
        let _unk_3 = u32::from_le_bytes(data.0[8..12].try_into().unwrap());
        let den = u32::from_le_bytes(data.0[12..16].try_into().unwrap());
        let seed = u32::from_le_bytes(data.0[16..20].try_into().unwrap());
        let special_type = u32::from_le_bytes(data.0[24..28].try_into().unwrap());

        if special_type >= 2 {
            return generate_event(data);
        }

        let six_star = special_type == 1;

        if seed == 0 {
            return Raid::default();
        }

        let mut rng = Xoroshiro128Plus::new(seed as u64);

        let mut star_level = if six_star {
            6
        } else {
            let mut stars = data.2.get_min_stars() as u8;
            let thresholds = data.2.get_thresholds();
            let mut star_level_rand = rng.next_masked(100) as u8;

            for threshold in thresholds {
                if star_level_rand <= *threshold {
                    break;
                } else {
                    stars += 1;
                    star_level_rand -= *threshold;
                }
            }

            stars
        };

        let slot_info = {
            let table_array = match star_level {
                1 => root_as_raid_enemy_table_01_array(DIFFICULTY_01_RAW).unwrap(),
                2 => root_as_raid_enemy_table_01_array(DIFFICULTY_02_RAW).unwrap(),
                3 => root_as_raid_enemy_table_01_array(DIFFICULTY_03_RAW).unwrap(),
                4 => root_as_raid_enemy_table_01_array(DIFFICULTY_04_RAW).unwrap(),
                5 => root_as_raid_enemy_table_01_array(DIFFICULTY_05_RAW).unwrap(),
                _ => root_as_raid_enemy_table_01_array(DIFFICULTY_06_RAW).unwrap(),
            };

            let opposite_rom_type = match data.1 {
                GameVersion::Scarlet => RaidRomType::TYPE_B,
                GameVersion::Violet => RaidRomType::TYPE_A,
            };

            let sum = table_array
                .values()
                .iter()
                .filter_map(|s| {
                    if s.raidEnemyInfo().romVer() == opposite_rom_type {
                        None
                    } else {
                        Some(s.raidEnemyInfo().rate() as u64)
                    }
                })
                .sum::<u64>();
            let mut slot_rand = rng.next_masked(sum);
            let mut slot = None;
            for value in table_array.values().iter() {
                if value.raidEnemyInfo().romVer() == opposite_rom_type {
                    continue;
                }
                if value.raidEnemyInfo().rate() as u64 > slot_rand {
                    slot = Some(value.raidEnemyInfo().clone());
                    break;
                } else {
                    slot_rand -= value.raidEnemyInfo().rate() as u64;
                }
            }
            slot.unwrap()
        };

        let mut rng = Xoroshiro128Plus::new(seed as u64);
        let tera_type = rng.next_masked(18);

        let species = slot_info.bossPokePara().devId().0;
        let form = slot_info.bossPokePara().formId() as u16;
        let flawless_ivs = slot_info.bossPokePara().talentVnum() as u8;
        let ivs = [
            slot_info.bossPokePara().talentValue().hp() as u8,
            slot_info.bossPokePara().talentValue().atk() as u8,
            slot_info.bossPokePara().talentValue().def() as u8,
            slot_info.bossPokePara().talentValue().spAtk() as u8,
            slot_info.bossPokePara().talentValue().spDef() as u8,
            slot_info.bossPokePara().talentValue().agi() as u8,
        ];
        let ability_param = slot_info.bossPokePara().tokusei().0 as u8;
        let gender_param = slot_info.bossPokePara().sex().0 as u8;
        let nature_param = slot_info.bossPokePara().seikaku().0 as u8;

        Self {
            _unk1,
            area,
            _unk_3,
            den,
            seed,
            tera_type: tera_type as u8,
            star_level,
            event: false,
            species: slot_info.bossPokePara().devId().0,
            form,
            pokemon: Pokemon::generate_from_seed(
                seed,
                species,
                form,
                flawless_ivs,
                ivs,
                ability_param,
                gender_param,
                nature_param,
            ),
        }
    }
}

fn generate_event(data: (&[u8], GameVersion, GameProgress)) -> Raid {
    let _unk1 = u32::from_le_bytes(data.0[0..4].try_into().unwrap());
    let area = u32::from_le_bytes(data.0[4..8].try_into().unwrap());
    let _unk_3 = u32::from_le_bytes(data.0[8..12].try_into().unwrap());
    let den = u32::from_le_bytes(data.0[12..16].try_into().unwrap());
    let seed = u32::from_le_bytes(data.0[16..20].try_into().unwrap());
    let is_seven_star = u32::from_le_bytes(data.0[24..28].try_into().unwrap()) >= 3;

    if seed == 0 {
        return Raid::default();
    }

    let mut rng = Xoroshiro128Plus::new(seed as u64);

    let mut star_level = 0;
    rng.next_masked(100);

    let mut buf = Vec::new();
    let slot_info = {
        let opposite_rom_type = match data.1 {
            GameVersion::Scarlet => delivery_enemy_table_generated::RaidRomType::TYPE_B,
            GameVersion::Violet => delivery_enemy_table_generated::RaidRomType::TYPE_A,
        };
        let table_array = if let Ok(mut file) = File::open("./raid_enemy_array") {
            file.read_to_end(&mut buf).unwrap();
            if let Ok(table) = root_as_delivery_raid_enemy_table_array(&buf) {
                table
            } else {
                root_as_delivery_raid_enemy_table_array(DELIVERY_RAW).unwrap()
            }
        } else {
            root_as_delivery_raid_enemy_table_array(DELIVERY_RAW).unwrap()
        };
        if is_seven_star {
            star_level = 7;
            table_array
                .values()
                .iter()
                .next()
                .unwrap()
                .raidEnemyInfo()
                .clone()
        } else {
            let sum = table_array
                .values()
                .iter()
                .filter_map(|s| {
                    if s.raidEnemyInfo().romVer() == opposite_rom_type
                        || s.raidEnemyInfo().difficulty() < data.2.get_min_stars()
                        || s.raidEnemyInfo().difficulty() > data.2.get_max_stars()
                    {
                        None
                    } else {
                        Some(s.raidEnemyInfo().rate() as u64)
                    }
                })
                .sum::<u64>();
            if sum == 0 {
                return Raid::default();
            }
            let mut slot_rand = rng.next_masked(sum);
            let mut slot_info: Option<delivery_enemy_table_generated::RaidEnemyInfo> = None;
            for value in table_array.values().iter() {
                if value.raidEnemyInfo().romVer() == opposite_rom_type
                    || value.raidEnemyInfo().difficulty() < data.2.get_min_stars()
                    || value.raidEnemyInfo().difficulty() > data.2.get_max_stars()
                {
                    continue;
                }
                if value.raidEnemyInfo().rate() as u64 > slot_rand {
                    star_level = value.raidEnemyInfo().difficulty() as u8;
                    slot_info = Some(value.raidEnemyInfo().clone());
                    break;
                } else {
                    slot_rand -= value.raidEnemyInfo().rate() as u64;
                }
            }
            slot_info.unwrap()
        }
    };

    let mut rng = Xoroshiro128Plus::new(seed as u64);
    let tera_type = if slot_info.bossPokePara().gemType()
        == delivery_enemy_table_generated::GemType::RANDOM
        || slot_info.bossPokePara().gemType() == delivery_enemy_table_generated::GemType::DEFAULT
    {
        rng.next_masked(18) as u8
    } else {
        (slot_info.bossPokePara().gemType().0 - 2) as u8
    };

    let species = slot_info.bossPokePara().devId().0;
    let form = slot_info.bossPokePara().formId() as u16;
    let flawless_ivs = if slot_info.bossPokePara().talentType()
        == delivery_enemy_table_generated::TalentType::VALUE
    {
        7
    } else {
        slot_info.bossPokePara().talentVnum() as u8
    };
    let ability_param = slot_info.bossPokePara().tokusei().0 as u8;
    let gender_param = slot_info.bossPokePara().sex().0 as u8;
    let nature_param = slot_info.bossPokePara().seikaku().0 as u8;
    let ivs = [
        slot_info.bossPokePara().talentValue().hp() as u8,
        slot_info.bossPokePara().talentValue().atk() as u8,
        slot_info.bossPokePara().talentValue().def() as u8,
        slot_info.bossPokePara().talentValue().spAtk() as u8,
        slot_info.bossPokePara().talentValue().spDef() as u8,
        slot_info.bossPokePara().talentValue().agi() as u8,
    ];

    Raid {
        _unk1,
        area,
        _unk_3,
        den,
        seed,
        tera_type,
        star_level,
        event: true,
        species,
        form,
        pokemon: Pokemon::generate_from_seed(
            seed,
            species,
            form,
            flawless_ivs,
            ivs,
            ability_param,
            gender_param,
            nature_param,
        ),
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Pokemon {
    pub ec: u32,
    pub pid: u32,
    pub shiny: bool,
    pub ivs: [u8; 6],
    pub gender: u8,
    pub ability: u8,
    pub nature: u8,
}

impl Pokemon {
    fn generate_from_seed(
        seed: u32,
        species: u16,
        form: u16,
        flawless_ivs: u8,
        mut ivs: [u8; 6],
        ability_param: u8,
        gender_param: u8,
        nature_param: u8,
    ) -> Self {
        let mut rng = Xoroshiro128Plus::new(seed as u64);
        let ec = rng.next_masked(0xFFFFFFFF) as u32;
        let tidsid = rng.next_masked(0xFFFFFFFF) as u32;
        let pid = rng.next_masked(0xFFFFFFFF) as u32;
        let shiny = (pid >> 16) ^ (pid & 0xFFFF) ^ (tidsid >> 16) ^ (tidsid & 0xFFFF) < 0x10;

        if flawless_ivs != 7 {
            if flawless_ivs != 0 {
                for _ in 0..flawless_ivs {
                    let mut index;
                    while {
                        index = rng.next_masked(6) as usize;
                        ivs[index] != 0
                    } {}
                    ivs[index] = 31;
                }
            }
            for iv in &mut ivs {
                if *iv == 0 {
                    *iv = rng.next_masked(32) as u8;
                }
            }
        }

        let ability = {
            match ability_param {
                0 => rng.next_masked(2) + 1,
                1 => rng.next_masked(3) + 1,
                2 => 1,
                3 => 2,
                _ => 3,
            }
        } as u8;

        let personal = personal_table::SV.get_form_entry(species as usize, form as usize);

        let gender = {
            match gender_param {
                0 => match personal.get_gender() {
                    0 => 0,
                    254 => 1,
                    255 => 2,
                    _ => {
                        let rand = rng.next_masked(100) as u8;
                        get_gender(personal.get_gender() as u8, rand)
                    }
                },
                1 => 0,
                _ => 1,
            }
        };

        let nature = if nature_param == 0 {
            rng.next_masked(25) as u8
        } else {
            nature_param - 1
        };

        Self {
            ec,
            pid,
            shiny,
            ivs,
            gender,
            ability,
            nature,
        }
    }
}

fn get_gender(ratio: u8, rand: u8) -> u8 {
    match ratio {
        0x1F => {
            if rand < 12 {
                1
            } else {
                0
            }
        }
        0x3F => {
            if rand < 25 {
                1
            } else {
                0
            }
        }
        0x7F => {
            if rand < 50 {
                1
            } else {
                0
            }
        }
        0xBF => {
            if rand < 75 {
                1
            } else {
                0
            }
        }
        0xE1 => {
            if rand < 89 {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}
