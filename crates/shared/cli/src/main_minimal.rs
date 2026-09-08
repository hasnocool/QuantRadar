use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Analyze(AnalyzeCommand),
}

#[derive(Subcommand, Debug)]
enum AnalyzeCommand {
    Discover { #[arg(long)] output: Option<String> },
    Screen { symbol: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Analyze(cmd) => {
            match cmd {
                AnalyzeCommand::Discover { output } => {
                    println!("Discovering markets...");
                    if let Some(out) = output {
                        println!("Output to {}", out);
                    }
                }
                AnalyzeCommand::Screen { symbol } => {
                    println!("Screening {}", symbol);
                }
            }
        }
    }
    Ok(())
}
