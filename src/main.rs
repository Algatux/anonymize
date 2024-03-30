use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    // path to the anonymize configuration file
    #[arg(short,long)]
    config_file: String,
    // path to the file to anonymize
    #[arg(short,long)]
    source_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let source_file = cli.source_file.unwrap_or_default();

    println!("Configuration file -> '{}'", cli.config_file);
    println!("Source file -> '{}'", source_file);

    if source_file.is_empty() {
        println!("Empty source_file path, I will take input from stdin");
    }
}

