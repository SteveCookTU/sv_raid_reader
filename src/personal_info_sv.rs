use crate::personal_info::PersonalInfo;

pub struct PersonalInfoSV {
    data: Vec<u8>
}

impl PersonalInfo for PersonalInfoSV {
    fn new(data: Vec<u8>) -> Self {
        Self {
            data
        }
    }

    fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    fn write(&mut self) -> Vec<u8> {
        unimplemented!()
    }

    fn get_hp(&self) -> usize {
        self.data[0] as usize
    }

    fn get_atk(&self) -> usize {
        self.data[1] as usize
    }

    fn get_def(&self) -> usize {
        self.data[2] as usize
    }

    fn get_spe(&self) -> usize {
        self.data[3] as usize
    }

    fn get_spa(&self) -> usize {
        self.data[4] as usize
    }

    fn get_spd(&self) -> usize {
        self.data[5] as usize
    }

    fn get_ev_hp(&self) -> usize {
        self.get_ev_yield() & 0x3
    }

    fn get_ev_atk(&self) -> usize {
        (self.get_ev_yield() >> 2) & 0x3
    }

    fn get_ev_def(&self) -> usize {
        (self.get_ev_yield() >> 4) & 0x3
    }

    fn get_ev_spe(&self) -> usize {
        (self.get_ev_yield() >> 6) & 0x3
    }

    fn get_ev_spa(&self) -> usize {
        (self.get_ev_yield() >> 8) & 0x3
    }

    fn get_ev_spd(&self) -> usize {
        (self.get_ev_yield() >> 10) & 0x3
    }

    fn get_type_1(&self) -> usize {
        self.data[6] as usize
    }

    fn get_type_2(&self) -> usize {
        self.data[7] as usize
    }

    fn get_egg_group_1(&self) -> usize {
        self.data[0x10] as usize
    }

    fn get_egg_group_2(&self) -> usize {
        self.data[0x11] as usize
    }

    fn get_catch_rate(&self) -> usize {
        self.data[8] as usize
    }

    fn get_items(&self) -> Vec<usize> {
        todo!()
    }

    fn get_gender(&self) -> usize {
        self.data[0xC] as usize
    }

    fn get_hatch_cycles(&self) -> usize {
        self.data[0xD] as usize
    }

    fn get_base_friendship(&self) -> usize {
        self.data[0xE] as usize
    }

    fn get_exp_growth(&self) -> usize {
        self.data[0xF] as usize
    }

    fn get_abilities(&self) -> Vec<usize> {
        vec![self.ability_1(), self.ability_2(), self.ability_H()]
    }

    fn get_ability_index(&self, ability_id: usize) -> Option<usize> {
        match ability_id {
            0 => Some(self.ability_1()),
            1 => Some(self.ability_2()),
            2 => Some(self.ability_H()),
            _ => None
        }
    }

    fn get_escape_rate(&self) -> usize {
        todo!()
    }

    fn get_base_exp(&self) -> usize {
        todo!()
    }

    fn get_color(&self) -> usize {
        todo!()
    }

    fn get_evo_stage(&self) -> usize {
        self.data[9] as usize
    }
}

impl PersonalInfoSV {
    fn get_ev_yield(&self) -> usize {
        u16::from_le_bytes(self.data[0xA..0xC].try_into().unwrap()) as usize
    }

    pub fn ability_1(&self) -> usize {
        u16::from_le_bytes(self.data[0x12..0x14].try_into().unwrap()) as usize
    }

    pub fn ability_2(&self) -> usize {
        u16::from_le_bytes(self.data[0x14..0x16].try_into().unwrap()) as usize
    }

    pub fn ability_H(&self) -> usize {
        u16::from_le_bytes(self.data[0x16..0x18].try_into().unwrap()) as usize
    }
}