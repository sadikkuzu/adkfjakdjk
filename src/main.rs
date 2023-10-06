use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
    /// Print crates.io link
    #[clap(short, long, action = clap::ArgAction::Count)]
    crates: u8,
}

fn main() {
    println!("Hello, world!");

    let cli = Cli::parse();

    if cli.crates > 0 {
        println!("https://crates.io/crates/adkfjakdjk")
    }
}
