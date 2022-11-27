use clap::{Parser, ValueEnum};
use sv_raid_reader::{
    personal_table, read_raids, Filter, GameProgress, GameVersion, PersonalInfo,
    RAID_BLOCK_POINTER, RAID_BLOCK_SIZE,
};
use sysbot_rs::SysBotClient;

#[derive(Parser)]
struct Cli {
    ip: String,
    #[arg(value_enum)]
    game: Game,
    #[arg(value_enum)]
    progress: Progress,
    #[arg(default_value_t = 6000)]
    port: u16,
    #[arg(long)]
    species: Option<u16>,
    #[arg(long, value_enum)]
    tera_type: Option<TeraType>,
    #[arg(long)]
    star_level: Option<u8>,
    #[arg(short, long)]
    shiny: bool,
    #[arg(short, long)]
    event: bool,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
#[repr(u8)]
enum TeraType {
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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
#[repr(u8)]
enum Game {
    Scarlet,
    Violet,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, ValueEnum)]
#[repr(u8)]
enum Progress {
    None,
    Badge3,
    Badge7,
    Credits,
    PostGame,
}

impl Into<GameVersion> for Game {
    fn into(self) -> GameVersion {
        match self {
            Game::Scarlet => GameVersion::Scarlet,
            Game::Violet => GameVersion::Violet,
        }
    }
}

impl Into<GameProgress> for Progress {
    fn into(self) -> GameProgress {
        match self {
            Progress::None => GameProgress::None,
            Progress::Badge3 => GameProgress::Badge3,
            Progress::Badge7 => GameProgress::Badge7,
            Progress::Credits => GameProgress::Credits,
            Progress::PostGame => GameProgress::PostGame,
        }
    }
}

fn main() {
    let cli: Cli = Cli::parse();

    let filter = Filter::default()
        .species(cli.species)
        .tera_type(cli.tera_type.map(|tt| tt as u8))
        .star_level(cli.star_level)
        .shiny(cli.shiny)
        .event(cli.event);

    if let Ok(client) = SysBotClient::connect(&cli.ip, cli.port) {
        let data = client
            .pointer_peek(&RAID_BLOCK_POINTER, RAID_BLOCK_SIZE)
            .unwrap();
        let raids = read_raids(&data, cli.game.into(), filter, cli.progress.into());
        for raid in raids {
            println!("{}", raid);
            println!();
        }
    } else {
        println!("Unable to connect to {}:{}", cli.ip, cli.port);
    }
}
