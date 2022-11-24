use crate::{Xoroshiro128Plus, TYPES};
use std::fmt::{Display, Formatter};

pub const RAID_BLOCK_POINTER: [u64; 3] = [0x42FD560, 0x160, 0x50];

pub const RAID_BLOCK_SIZE: usize = 0xC80;

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
    shiny: bool,
    ivs: [u8; 6],
    tera_type: u8,
    star_level: u8,
}

impl Display for Raid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Area: {} Den: {}",
            AREAS[self.area as usize - 1],
            self.den
        )?;
        writeln!(f, "Seed: {:0>8X}", self.seed)?;
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

    pub fn regenerate_ivs(&mut self, star_level: u8) {
        self.star_level = star_level;

        let mut rng = Xoroshiro128Plus::new(self.seed as u64);
        rng.next();
        rng.next();
        rng.next();

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

        self.ivs = ivs;
    }
}

impl From<&[u8]> for Raid {
    fn from(data: &[u8]) -> Self {
        let _unk1 = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let area = u32::from_le_bytes(data[4..8].try_into().unwrap());
        let _unk_3 = u32::from_le_bytes(data[8..12].try_into().unwrap());
        let den = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let seed = u32::from_le_bytes(data[16..20].try_into().unwrap());

        if seed == 0 {
            return Raid::default();
        }

        let mut rng = Xoroshiro128Plus::new(seed as u64);
        let star_level_rand = rng.next_masked(100);

        let star_level = if star_level_rand < 30 {
            3
        } else if star_level_rand < 70 {
            4
        } else {
            5
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
            star_level: star_level as u8,
        }
    }
}
