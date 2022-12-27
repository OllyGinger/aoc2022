use clap::Parser;
use std::fs;

mod solutions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u16,
}

fn main() {
    let args: Args = Args::parse();
    let path_to_data = format!("data/day{:02}.txt", args.day);
    let data: String = match fs::read_to_string(&path_to_data) {
        Ok(v) => v,
        Err(e) => {
            eprint!(
                "Error reading input file '{}': {}",
                &path_to_data,
                e.to_string()
            );
            std::process::exit(1);
        }
    };

    let output: String = match &args.day {
        1 => solutions::day1::run(&data),
        2 => solutions::day2::run(&data),
        3 => solutions::day3::run(&data),
        4 => solutions::day4::run(&data),
        5 => solutions::day5::run(&data),
        6 => solutions::day6::run(&data),
        7 => solutions::day7::run(&data),
        8 => solutions::day8::run(&data),
        9 => solutions::day9::run(&data),
        10 => solutions::day10::run(&data),
        _ => "".to_string(),
    };

    println!("{}", output);
}
