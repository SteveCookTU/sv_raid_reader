use clap::Parser;
use sv_raid_reader::{Raid, RAID_BLOCK_POINTER, RAID_BLOCK_SIZE};
use sysbot_rs::SysBotClient;

#[derive(Parser)]
struct Cli {
    ip: String,
    #[arg(default_value_t = 6000)]
    port: u16,
}

fn main() {
    let cli: Cli = Cli::parse();

    if let Ok(client) = SysBotClient::connect(&cli.ip, cli.port) {
        let data = client
            .pointer_peek(&RAID_BLOCK_POINTER, RAID_BLOCK_SIZE)
            .unwrap();
        for offset in (0..RAID_BLOCK_SIZE).step_by(Raid::SIZE) {
            let raid_data = &data[offset..(offset + Raid::SIZE)];
            let raid: Raid = raid_data.into();
            if raid.is_valid() {
                println!("{}", raid);
                println!();
            }
        }
    } else {
        println!("Unable to connect to {}:{}", cli.ip, cli.port);
    }
}
