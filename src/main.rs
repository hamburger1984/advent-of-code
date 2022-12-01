use clap::Parser;

#[derive(Parser)]
struct Cli {
    day: u8,
}

mod day01;

fn main() {
    let cli = Cli::parse();

    print!("So you want to run day {}.\n", cli.day);

    if cli.day == 1 {
        day01::run();
    }
}
