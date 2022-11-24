mod raid;
mod xoroshiro128plus;

pub use raid::*;
pub use xoroshiro128plus::*;

pub const TYPES: [&str; 18] = [
    "Normal", "Fighting", "Flying", "Poison", "Ground", "Rock", "Bug", "Ghost", "Steel", "Fire",
    "Water", "Grass", "Electric", "Psychic", "Ice", "Dragon", "Dark", "Fairy",
];
