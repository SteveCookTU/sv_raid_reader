use clap::{Parser, ValueEnum};
use sv_raid_reader::{Filter, Raid, RAID_BLOCK_POINTER, RAID_BLOCK_SIZE};
use sysbot_rs::SysBotClient;

#[derive(Parser)]
struct Cli {
    ip: String,
    #[arg(default_value_t = 6000)]
    port: u16,
    #[arg(short, long)]
    species: Option<u16>,
    #[arg(short, long, value_enum)]
    tera_type: Option<TeraType>,
    #[arg(short, long)]
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
    Fairy
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
        for offset in (0..RAID_BLOCK_SIZE).step_by(Raid::SIZE) {
            let raid_data = &data[offset..(offset + Raid::SIZE)];
            let raid: Raid = raid_data.into();
            if raid.is_valid() && raid.passes_filter(&filter) {
                println!("{}", raid);
                println!();
            }
        }
    } else {
        println!("Unable to connect to {}:{}", cli.ip, cli.port);
    }
}
