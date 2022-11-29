use crate::IvType::{VNum, Value};
pub struct RaidEncounter {
    species: u16,
    level: u8,
    difficulty: u8,
    reusable_moves: [u16; 4],
    gem_type: GemType,
    tokusei: Tokusei,
    flawless_ivs: u8,
    iv_type: IvType,
    ivs: [u8; 6],
    evs: [u8; 6],
    hp_coef: u16,
    shield_hp_trigger: u8,
    shield_time_trigger: u8,
    shield_time_limit: u16,
    shield_cancel_damage: u8,
    shield_damage_rate: u8,
    shield_gem_damage_rate: u8,
    shield_change_gem_damage_rate: u8,
    extra_actions: [ExtraAction; 6],
    game_limit: u32,
    command_limit: u32,
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
    trigger: ExtraActionTrigger,
    action: ExtraActionType,
    value: u16,
    move_no: Option<u16>,
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
            difficulty: (info.no() / 1000) as u8,
            reusable_moves: moves,
            gem_type: info.bossPokePara().gemType().into(),
            tokusei: info.bossPokePara().tokusei().into(),
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
            difficulty: info.difficulty() as u8,
            reusable_moves: moves,
            gem_type: info.bossPokePara().gemType().into(),
            tokusei: info.bossPokePara().tokusei().into(),
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
            extra_actions,
            game_limit: info.raidTimeData().gameLimit() as u32,
            command_limit: info.raidTimeData().commandLimit() as u32,
        }
    }
}
