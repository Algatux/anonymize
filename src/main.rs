use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Anonymize")]
#[command(version = "1.0")]
#[command(about = "Sql anonimizer cli software", long_about = None)]
struct Cli {
    
    #[arg(short,long,help="Path to the anonymize configuration file")]
    config_file: String,
    
    #[arg(short,long,help="Path to the file to anonymize")]
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

