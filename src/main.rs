use clap::Parser;

mod days;
use days::*;

mod common;
use common::*;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The day to run
    #[arg(short, long)]
    day: u8,

    /// The part to run
    #[arg(short, long)]
    section: Option<u8>,
}
fn main() {
    let args = Args::parse();

    let section = match args.section {
        Some(section) => SectionSelection::Single(section),
        None => SectionSelection::All,
    };
    
    match args.day {
        1 => day_01::invoke(section),
        // 2 => day_02::invoke(section),
        // 3 => day_03::invoke(section),
        // 4 => day_04::invoke(section),
        // 5 => day_05::invoke(section),
        // 6 => day_06::invoke(section),
        // 7 => day_07::invoke(section),
        // 8 => day_08::invoke(section),
        // 9 => day_09::invoke(section),
        // 10 => day_10::invoke(section),
        // 11 => day_11::invoke(section),
        // 12 => day_12::invoke(section),
        // 13 => day_13::invoke(section),
        // 14 => day_14::invoke(section),
        // 15 => day_15::invoke(section),
        // 16 => day_16::invoke(section),
        // 17 => day_17::invoke(section),
        // 18 => day_18::invoke(section),
        // 19 => day_19::invoke(section),
        // 20 => day_20::invoke(section),
        // 21 => day_21::invoke(section),
        // 22 => day_22::invoke(section),
        // 23 => day_23::invoke(section),
        // 24 => day_24::invoke(section),
        // 25 => day_25::invoke(section),
        _ => println!("Day {} not implemented", args.day),
    }
    
}
