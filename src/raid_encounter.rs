use crate::IvType::{VNum, Value};

pub struct RaidEncounter {
    pub species: u16,
    pub level: u8,
    pub shiny: ShinyType,
    pub difficulty: u8,
    pub reusable_moves: [u16; 4],
    pub gem_type: GemType,
    pub tokusei: Tokusei,
    pub seikaku: Seikaku,
    pub gender: Gender,
    pub flawless_ivs: u8,
    pub iv_type: IvType,
    pub ivs: [u8; 6],
    pub evs: [u8; 6],
    pub hp_coef: u16,
    pub shield_hp_trigger: u8,
    pub shield_time_trigger: u8,
    pub shield_time_limit: u16,
    pub shield_cancel_damage: u8,
    pub shield_damage_rate: u8,
    pub shield_gem_damage_rate: u8,
    pub shield_change_gem_damage_rate: u8,
    pub second_shield_hp_trigger: u8,
    pub second_shield_time_trigger: u8,
    pub second_shield_damage_rate: u8,
    pub extra_actions: [ExtraAction; 6],
    pub game_limit: u32,
    pub command_limit: u32,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum ShinyType {
    #[default]
    Random,
    No,
    Yes,
}

impl From<crate::raid_enemy_table_01_generated::RareType> for ShinyType {
    fn from(rare: crate::raid_enemy_table_01_generated::RareType) -> Self {
        match rare.0 {
            1 => ShinyType::No,
            2 => ShinyType::Yes,
            _ => ShinyType::Random,
        }
    }
}

impl From<crate::delivery_enemy_table_generated::RareType> for ShinyType {
    fn from(rare: crate::delivery_enemy_table_generated::RareType) -> Self {
        match rare.0 {
            1 => ShinyType::No,
            2 => ShinyType::Yes,
            _ => ShinyType::Random,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum GemType {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
    #[default]
    Random,
}

impl From<crate::raid_enemy_table_01_generated::GemType> for GemType {
    fn from(gem: crate::raid_enemy_table_01_generated::GemType) -> Self {
        if gem.0 > 1 {
            match gem.0 - 2 {
                0 => GemType::Normal,
                1 => GemType::Fighting,
                2 => GemType::Flying,
                3 => GemType::Poison,
                4 => GemType::Ground,
                5 => GemType::Rock,
                6 => GemType::Bug,
                7 => GemType::Ghost,
                8 => GemType::Steel,
                9 => GemType::Fire,
                10 => GemType::Water,
                11 => GemType::Grass,
                12 => GemType::Electric,
                13 => GemType::Psychic,
                14 => GemType::Ice,
                15 => GemType::Dragon,
                16 => GemType::Dark,
                17 => GemType::Fairy,
                _ => GemType::Random,
            }
        } else {
            Self::Random
        }
    }
}

impl From<crate::delivery_enemy_table_generated::GemType> for GemType {
    fn from(gem: crate::delivery_enemy_table_generated::GemType) -> Self {
        if gem.0 > 1 {
            match gem.0 - 2 {
                0 => GemType::Normal,
                1 => GemType::Fighting,
                2 => GemType::Flying,
                3 => GemType::Poison,
                4 => GemType::Ground,
                5 => GemType::Rock,
                6 => GemType::Bug,
                7 => GemType::Ghost,
                8 => GemType::Steel,
                9 => GemType::Fire,
                10 => GemType::Water,
                11 => GemType::Grass,
                12 => GemType::Electric,
                13 => GemType::Psychic,
                14 => GemType::Ice,
                15 => GemType::Dragon,
                16 => GemType::Dark,
                17 => GemType::Fairy,
                _ => GemType::Random,
            }
        } else {
            Self::Random
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Tokusei {
    Random12,
    Random123,
    Set1,
    Set2,
    Set3,
}

impl From<crate::raid_enemy_table_01_generated::TokuseiType> for Tokusei {
    fn from(tokusei: crate::raid_enemy_table_01_generated::TokuseiType) -> Self {
        match tokusei.0 {
            1 => Tokusei::Random123,
            2 => Tokusei::Set1,
            3 => Tokusei::Set2,
            4 => Tokusei::Set3,
            _ => Tokusei::Random12,
        }
    }
}

impl From<crate::delivery_enemy_table_generated::TokuseiType> for Tokusei {
    fn from(tokusei: crate::delivery_enemy_table_generated::TokuseiType) -> Self {
        match tokusei.0 {
            1 => Tokusei::Random123,
            2 => Tokusei::Set1,
            3 => Tokusei::Set2,
            4 => Tokusei::Set3,
            _ => Tokusei::Random12,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Seikaku {
    #[default]
    Random,
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

impl From<crate::raid_enemy_table_01_generated::SeikakuType> for Seikaku {
    fn from(seikaku: crate::raid_enemy_table_01_generated::SeikakuType) -> Self {
        match seikaku.0 {
            1 => Seikaku::Normal,
            2 => Seikaku::Fighting,
            3 => Seikaku::Flying,
            4 => Seikaku::Poison,
            5 => Seikaku::Ground,
            6 => Seikaku::Rock,
            7 => Seikaku::Bug,
            8 => Seikaku::Ghost,
            9 => Seikaku::Steel,
            10 => Seikaku::Fire,
            11 => Seikaku::Water,
            12 => Seikaku::Grass,
            13 => Seikaku::Electric,
            14 => Seikaku::Psychic,
            15 => Seikaku::Ice,
            16 => Seikaku::Dragon,
            17 => Seikaku::Dark,
            18 => Seikaku::Fairy,
            _ => Seikaku::Random,
        }
    }
}

impl From<crate::delivery_enemy_table_generated::SeikakuType> for Seikaku {
    fn from(seikaku: crate::delivery_enemy_table_generated::SeikakuType) -> Self {
        match seikaku.0 {
            1 => Seikaku::Normal,
            2 => Seikaku::Fighting,
            3 => Seikaku::Flying,
            4 => Seikaku::Poison,
            5 => Seikaku::Ground,
            6 => Seikaku::Rock,
            7 => Seikaku::Bug,
            8 => Seikaku::Ghost,
            9 => Seikaku::Steel,
            10 => Seikaku::Fire,
            11 => Seikaku::Water,
            12 => Seikaku::Grass,
            13 => Seikaku::Electric,
            14 => Seikaku::Psychic,
            15 => Seikaku::Ice,
            16 => Seikaku::Dragon,
            17 => Seikaku::Dark,
            18 => Seikaku::Fairy,
            _ => Seikaku::Random,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Gender {
    Random,
    Male,
    Female,
}

impl From<crate::raid_enemy_table_01_generated::SexType> for Gender {
    fn from(gender: crate::raid_enemy_table_01_generated::SexType) -> Self {
        match gender.0 {
            1 => Gender::Male,
            2 => Gender::Female,
            _ => Gender::Random,
        }
    }
}

impl From<crate::delivery_enemy_table_generated::SexType> for Gender {
    fn from(gender: crate::delivery_enemy_table_generated::SexType) -> Self {
        match gender.0 {
            1 => Gender::Male,
            2 => Gender::Female,
            _ => Gender::Random,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum IvType {
    Random,
    VNum,
    Value,
}

impl From<crate::raid_enemy_table_01_generated::TalentType> for IvType {
    fn from(talent: crate::raid_enemy_table_01_generated::TalentType) -> Self {
        match talent.0 {
            1 => IvType::VNum,
            2 => IvType::Value,
            _ => IvType::Random,
        }
    }
}

impl From<crate::delivery_enemy_table_generated::TalentType> for IvType {
    fn from(talent: crate::delivery_enemy_table_generated::TalentType) -> Self {
        match talent.0 {
            1 => IvType::VNum,
            2 => IvType::Value,
            _ => IvType::Random,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct ExtraAction {
    pub trigger: ExtraActionTrigger,
    pub action: ExtraActionType,
    pub value: u16,
    pub move_no: Option<u16>,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum ExtraActionTrigger {
    #[default]
    None,
    Time,
    Hp,
}

impl From<crate::raid_enemy_table_01_generated::RaidBossExtraTimingType> for ExtraActionTrigger {
    fn from(trigger: crate::raid_enemy_table_01_generated::RaidBossExtraTimingType) -> Self {
        match trigger.0 {
            1 => ExtraActionTrigger::Time,
            2 => ExtraActionTrigger::Hp,
            _ => ExtraActionTrigger::None,
        }
    }
}

impl From<crate::delivery_enemy_table_generated::RaidBossExtraTimingType> for ExtraActionTrigger {
    fn from(trigger: crate::delivery_enemy_table_generated::RaidBossExtraTimingType) -> Self {
        match trigger.0 {
            1 => ExtraActionTrigger::Time,
            2 => ExtraActionTrigger::Hp,
            _ => ExtraActionTrigger::None,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum ExtraActionType {
    #[default]
    None,
    BossStatusReset,
    PlayerStatusReset,
    Move,
    GemCount,
}

impl From<crate::raid_enemy_table_01_generated::RaidBossExtraActType> for ExtraActionType {
    fn from(act: crate::raid_enemy_table_01_generated::RaidBossExtraActType) -> Self {
        match act.0 {
            1 => ExtraActionType::BossStatusReset,
            2 => ExtraActionType::PlayerStatusReset,
            3 => ExtraActionType::Move,
            4 => ExtraActionType::GemCount,
            _ => ExtraActionType::None,
        }
    }
}

impl From<crate::delivery_enemy_table_generated::RaidBossExtraActType> for ExtraActionType {
    fn from(act: crate::delivery_enemy_table_generated::RaidBossExtraActType) -> Self {
        match act.0 {
            1 => ExtraActionType::BossStatusReset,
            2 => ExtraActionType::PlayerStatusReset,
            3 => ExtraActionType::Move,
            4 => ExtraActionType::GemCount,
            _ => ExtraActionType::None,
        }
    }
}

impl From<crate::raid_enemy_table_01_generated::RaidBossExtraData<'_>> for ExtraAction {
    fn from(data: crate::raid_enemy_table_01_generated::RaidBossExtraData<'_>) -> Self {
        Self {
            trigger: data.timming().into(),
            action: data.action().into(),
            value: data.value() as u16,
            move_no: if data.wazano().0 == 0 {
                None
            } else {
                Some(data.wazano().0)
            },
        }
    }
}

impl From<crate::delivery_enemy_table_generated::RaidBossExtraData<'_>> for ExtraAction {
    fn from(data: crate::delivery_enemy_table_generated::RaidBossExtraData<'_>) -> Self {
        Self {
            trigger: data.timming().into(),
            action: data.action().into(),
            value: data.value() as u16,
            move_no: if data.wazano().0 == 0 {
                None
            } else {
                Some(data.wazano().0)
            },
        }
    }
}

impl From<crate::raid_enemy_table_01_generated::RaidEnemyInfo<'_>> for RaidEncounter {
    fn from(info: crate::raid_enemy_table_01_generated::RaidEnemyInfo<'_>) -> Self {
        let mut ivs = [0; 6];
        let mut evs = [0; 6];
        let mut moves = [0; 4];
        let mut extra_actions = [ExtraAction::default(); 6];

        ivs[0] = info.bossPokePara().talentValue().hp() as u8;
        ivs[1] = info.bossPokePara().talentValue().atk() as u8;
        ivs[2] = info.bossPokePara().talentValue().def() as u8;
        ivs[3] = info.bossPokePara().talentValue().spAtk() as u8;
        ivs[4] = info.bossPokePara().talentValue().spDef() as u8;
        ivs[5] = info.bossPokePara().talentValue().agi() as u8;

        evs[0] = info.bossPokePara().effortValue().hp() as u8;
        evs[1] = info.bossPokePara().effortValue().atk() as u8;
        evs[2] = info.bossPokePara().effortValue().def() as u8;
        evs[3] = info.bossPokePara().effortValue().spAtk() as u8;
        evs[4] = info.bossPokePara().effortValue().spDef() as u8;
        evs[5] = info.bossPokePara().effortValue().agi() as u8;

        moves[0] = info.bossPokePara().waza1().wazaId().0;
        moves[1] = info.bossPokePara().waza2().wazaId().0;
        moves[2] = info.bossPokePara().waza3().wazaId().0;
        moves[3] = info.bossPokePara().waza4().wazaId().0;

        extra_actions[0] = info.bossDesc().extraAction1().into();
        extra_actions[1] = info.bossDesc().extraAction2().into();
        extra_actions[2] = info.bossDesc().extraAction3().into();
        extra_actions[3] = info.bossDesc().extraAction4().into();
        extra_actions[4] = info.bossDesc().extraAction5().into();
        extra_actions[5] = info.bossDesc().extraAction6().into();

        RaidEncounter {
            species: info.bossPokePara().devId().0,
            level: info.bossPokePara().level() as u8,
            shiny: info.bossPokePara().rareType().into(),
            difficulty: (info.no() / 1000) as u8,
            reusable_moves: moves,
            gem_type: info.bossPokePara().gemType().into(),
            tokusei: info.bossPokePara().tokusei().into(),
            seikaku: info.bossPokePara().seikaku().into(),
            gender: info.bossPokePara().sex().into(),
            flawless_ivs: info.bossPokePara().talentVnum() as u8,
            iv_type: info.bossPokePara().talentType().into(),
            ivs,
            evs,
            hp_coef: info.bossDesc().hpCoef() as u16,
            shield_hp_trigger: info.bossDesc().powerChargeTrigerHp() as u8,
            shield_time_trigger: info.bossDesc().powerChargeTrigerTime() as u8,
            shield_time_limit: info.bossDesc().powerChargeLimitTime() as u16,
            shield_cancel_damage: info.bossDesc().powerChargeCancelDamage() as u8,
            shield_damage_rate: info.bossDesc().powerChargeDamageRate() as u8,
            shield_gem_damage_rate: info.bossDesc().powerChargeGemDamageRate() as u8,
            shield_change_gem_damage_rate: info.bossDesc().powerChargeChangeGemDamageRate() as u8,
            second_shield_hp_trigger: info.bossDesc().doubleActionTrigerHp() as u8,
            second_shield_time_trigger: info.bossDesc().doubleActionTrigerTime() as u8,
            second_shield_damage_rate: info.bossDesc().doubleActionRate() as u8,
            extra_actions,
            game_limit: info.raidTimeData().gameLimit() as u32,
            command_limit: info.raidTimeData().commandLimit() as u32,
        }
    }
}

impl From<crate::delivery_enemy_table_generated::RaidEnemyInfo<'_>> for RaidEncounter {
    fn from(info: crate::delivery_enemy_table_generated::RaidEnemyInfo<'_>) -> Self {
        let mut ivs = [0; 6];
        let mut evs = [0; 6];
        let mut moves = [0; 4];
        let mut extra_actions = [ExtraAction::default(); 6];

        ivs[0] = info.bossPokePara().talentValue().hp() as u8;
        ivs[1] = info.bossPokePara().talentValue().atk() as u8;
        ivs[2] = info.bossPokePara().talentValue().def() as u8;
        ivs[3] = info.bossPokePara().talentValue().spAtk() as u8;
        ivs[4] = info.bossPokePara().talentValue().spDef() as u8;
        ivs[5] = info.bossPokePara().talentValue().agi() as u8;

        evs[0] = info.bossPokePara().effortValue().hp() as u8;
        evs[1] = info.bossPokePara().effortValue().atk() as u8;
        evs[2] = info.bossPokePara().effortValue().def() as u8;
        evs[3] = info.bossPokePara().effortValue().spAtk() as u8;
        evs[4] = info.bossPokePara().effortValue().spDef() as u8;
        evs[5] = info.bossPokePara().effortValue().agi() as u8;

        moves[0] = info.bossPokePara().waza1().wazaId().0;
        moves[1] = info.bossPokePara().waza2().wazaId().0;
        moves[2] = info.bossPokePara().waza3().wazaId().0;
        moves[3] = info.bossPokePara().waza4().wazaId().0;

        extra_actions[0] = info.bossDesc().extraAction1().into();
        extra_actions[1] = info.bossDesc().extraAction2().into();
        extra_actions[2] = info.bossDesc().extraAction3().into();
        extra_actions[3] = info.bossDesc().extraAction4().into();
        extra_actions[4] = info.bossDesc().extraAction5().into();
        extra_actions[5] = info.bossDesc().extraAction6().into();

        RaidEncounter {
            species: info.bossPokePara().devId().0,
            level: info.bossPokePara().level() as u8,
            shiny: info.bossPokePara().rareType().into(),
            difficulty: info.difficulty() as u8,
            reusable_moves: moves,
            gem_type: info.bossPokePara().gemType().into(),
            tokusei: info.bossPokePara().tokusei().into(),
            seikaku: info.bossPokePara().seikaku().into(),
            gender: info.bossPokePara().sex().into(),
            flawless_ivs: info.bossPokePara().talentVnum() as u8,
            iv_type: info.bossPokePara().talentType().into(),
            ivs,
            evs,
            hp_coef: info.bossDesc().hpCoef() as u16,
            shield_hp_trigger: info.bossDesc().powerChargeTrigerHp() as u8,
            shield_time_trigger: info.bossDesc().powerChargeTrigerTime() as u8,
            shield_time_limit: info.bossDesc().powerChargeLimitTime() as u16,
            shield_cancel_damage: info.bossDesc().powerChargeCancelDamage() as u8,
            shield_damage_rate: info.bossDesc().powerChargeDamageRate() as u8,
            shield_gem_damage_rate: info.bossDesc().powerChargeGemDamageRate() as u8,
            shield_change_gem_damage_rate: info.bossDesc().powerChargeChangeGemDamageRate() as u8,
            second_shield_hp_trigger: info.bossDesc().doubleActionTrigerHp() as u8,
            second_shield_time_trigger: info.bossDesc().doubleActionTrigerTime() as u8,
            second_shield_damage_rate: info.bossDesc().doubleActionRate() as u8,
            extra_actions,
            game_limit: info.raidTimeData().gameLimit() as u32,
            command_limit: info.raidTimeData().commandLimit() as u32,
        }
    }
}
