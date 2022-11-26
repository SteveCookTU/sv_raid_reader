use crate::personal_info::PersonalInfo;
use crate::personal_info_sv::PersonalInfoSV;
use lazy_static::lazy_static;
use std::ops::Index;

pub const PERSONAL_SV: &[u8] = include_bytes!("../resources/personal_array.bin");

lazy_static! {
    pub static ref SV: PersonalTable<PersonalInfoSV> = PersonalTable::new(PERSONAL_SV);
}

pub struct PersonalTable<T: PersonalInfo> {
    table: Vec<T>,
    max_species_id: usize,
}

impl<T: PersonalInfo> Index<usize> for PersonalTable<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[index]
    }
}

impl<T: PersonalInfo> PersonalTable<T> {
    pub fn new(data: &[u8]) -> Self {
        let count = data.len() / 0x44;
        let mut table = Vec::with_capacity(count);
        for i in 0..count {
            table.push(T::new((data[(0x44 * i)..((0x44 * i) + 0x44)]).to_vec()))
        }
        Self {
            table,
            max_species_id: 1010,
        }
    }

    pub fn get_form_index(&self, species: usize, form: usize) -> usize {
        if species <= self.max_species_id {
            self[species].form_index(species, form)
        } else {
            0
        }
    }

    pub fn get_form_entry(&self, species: usize, form: usize) -> &T {
        &self[self.get_form_index(species, form)]
    }

    pub fn table_length(&self) -> usize {
        self.table.len()
    }

    pub fn get_form_list(&self, species: Vec<String>, max_species: usize) -> Vec<Vec<String>> {
        let mut form_list = Vec::with_capacity(max_species + 1);
        for i in 0..(max_species + 1) {
            let form_count = self[i].get_form_count();
            form_list[i] = Vec::with_capacity(form_count);
            if form_count == 0 {
                continue;
            }

            form_list[i][0] = species[i].clone();

            for j in 1..form_count {
                form_list[i][j] = format!("{} {}", species[i], j);
            }
        }
        form_list
    }

    pub fn get_personal_entry_list(
        &self,
        forms: Vec<Vec<String>>,
        species: Vec<String>,
        max_species: usize,
        base_form: &mut Vec<usize>,
        form_val: &mut Vec<usize>,
    ) -> Vec<String> {
        let mut result = Vec::with_capacity(self.table.len());
        *base_form = Vec::with_capacity(self.table.len());
        *form_val = Vec::with_capacity(self.table.len());

        for i in 0..max_species {
            result[i] = species[i].clone();
            if forms[i].is_empty() {
                continue;
            }
            let base_ptr = self[i].get_form_stats_index();
            if base_ptr == 0 {
                continue;
            }

            for j in 1..forms[i].len() {
                let ptr = base_ptr + j - 1;
                base_form[ptr] = i;
                form_val[ptr] = j;
                result[ptr] = forms[i][j].clone();
            }
        }

        result
    }
}
