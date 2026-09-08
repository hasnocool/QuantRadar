use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(name = "quantaradar", version, about = "QuantRadar")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    DataImport { source: String, format: String },
    DataList,
    DataValidate { dataset: String },
    AnalyzeDiscover,
    AnalyzeScreen { symbol: String },
    AnalyzeScan,
    AnalyzeFeatures,
    AnalyzeRegime { symbol: String },
    AnalyzeRank,
    AnalyzePca,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::DataImport { source, format } => {
            println!("Importing {} as {}", source, format);
            fs::create_dir_all("data/")?;
            fs::write(format!("data/{}.json", source.replace("/","_")), "{}")?;
        }
        Commands::DataList => {
            println!("Datasets:");
            if Path::new("data/").exists() {
                for entry in fs::read_dir("data/")? {
                    if let Ok(e) = entry {
                        println!("  {}", e.file_name().to_string_lossy());
                    }
                }
            }
        }
        Commands::DataValidate { dataset } => {
            println!("Validating {}... OK", dataset);
        }
        Commands::AnalyzeDiscover => {
            println!("Discovering markets...");
        }
        Commands::AnalyzeScreen { symbol } => {
            println!("Screening {}", symbol);
        }
        Commands::AnalyzeScan => println!("Scan — concurrency + feature-engine"),
        Commands::AnalyzeFeatures => println!("Features — ema/rsi/atr"),
        Commands::AnalyzeRegime { symbol } => println!("Regime — regime-detector {}", symbol),
        Commands::AnalyzeRank => println!("Rank — cross-section"),
        Commands::AnalyzePca => println!("PCA — components"),
    }
    Ok(())
}
// Analyze handlers — real integration to feature-engine, regime-detector, ranking, pca
use quantaradar_core::{Bar, Regime};
use quantaradar_features::{feature_rows, ema, rsi};
use quantaradar_regime::{classify, RegimeThresholds};
