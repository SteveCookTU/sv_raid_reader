#[derive(Copy, Clone, Default)]
pub struct Filter {
    pub tera_type: Option<u8>,
    pub star_level: Option<u8>,
    pub shiny: bool,
    pub event: bool,
    pub species: Option<u16>,
}

impl Filter {
    pub fn tera_type(mut self, tera_type: Option<u8>) -> Self {
        self.tera_type = tera_type;
        self
    }

    pub fn star_level(mut self, star_level: Option<u8>) -> Self {
        self.star_level = star_level;
        self
    }

    pub fn shiny(mut self, shiny: bool) -> Self {
        self.shiny = shiny;
        self
    }

    pub fn event(mut self, event: bool) -> Self {
        self.event = event;
        self
    }

    pub fn species(mut self, species: Option<u16>) -> Self {
        self.species = species;
        self
    }
}
