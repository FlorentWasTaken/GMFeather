use clap::Parser;
use feather_core::use_cases;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();
    let message = use_cases::greet_user(&args.name);
    println!("{}", message);
}
