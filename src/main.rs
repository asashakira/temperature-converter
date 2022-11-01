use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The temperature to convert - must be a number
    temperature: f64,

    /// Celcius to Fahrenheit (default)
    #[arg(short, long)]
    celcius: bool,

    /// Fahrenheit to Celcius
    #[arg(short, long)]
    fahrenheit: bool,
}

fn main() {
    let args = Cli::parse();
    if args.fahrenheit {
        let c = (args.temperature - 32.0) * 5.0 / 9.0;
        println!("{} Celcius", c);
    } else {
        let f = args.temperature * 1.8 + 32.0;
        println!("{} Fahrenheit", f);
    }
}
