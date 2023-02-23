use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    number: usize,
}

fn main() {
    let cli = Cli::parse();

    let c = rualg::stairs::count(cli.number);
    println!(
        "the input number is {}, and the result is: {}",
        cli.number, c
    );
}
