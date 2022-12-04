use clap::Parser;
use std::fs;

pub mod day1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u16,

    #[arg(short, long)]
    input_path: String,
}

fn main() {
    let args: Args = Args::parse();
    let data: String = match fs::read_to_string(&args.input_path) {
        Ok(v) => v,
        Err(e) => {
            eprint!(
                "Error reading input file '{}': {}",
                &args.input_path,
                e.to_string()
            );
            std::process::exit(1);
        }
    };

    let output: String = match &args.day {
        1 => day1::run(&data),
        _ => "".to_string(),
    };

    println!("{}", output);
}
