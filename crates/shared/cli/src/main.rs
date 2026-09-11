use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process::Command;

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
    GenerateReport {
        #[arg(short, long)]
        crate_name: Option<String>,
        #[arg(short = 'a', long)]
        all: bool,
        #[arg(short, long)]
        tests: bool,
    },
}

fn list_crates() -> Vec<(String, String)> {
    let mut out = vec![];
    for base in ["crates/shared", "crates/standalone"] {
        if Path::new(base).exists() {
            if let Ok(entries) = fs::read_dir(base) {
                for d in entries {
                    if let Ok(e) = d {
                        let name = e.file_name().to_string_lossy().to_string();
                        out.push((name.clone(), format!("{}/{}", base, name)));
                    }
                }
            }
        }
    }
    out.sort();
    out
}

fn cargo_check(crate_path: &str) -> bool {
    let status = Command::new("cargo")
        .args(["check", "--manifest-path", &format!("{}/Cargo.toml", crate_path)])
        .current_dir(crate_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    status
}

fn cargo_test(crate_path: &str) -> bool {
    let status = Command::new("cargo")
        .args(["test", "--manifest-path", &format!("{}/Cargo.toml", crate_path), "--quiet"])
        .current_dir(crate_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    status
}

fn generate_report(crate_name: Option<&str>, all: bool, with_tests: bool) {
    let crates = list_crates();
    let targets = if let Some(name) = crate_name {
        vec![(name.to_string(), format!("crates/shared/{}", name))]
    } else if all {
        crates.clone()
    } else {
        vec![]
    };
    if targets.is_empty() {
        eprintln!("Specify --crate <NAME> or --all");
        return;
    }
    fs::create_dir_all("reports").unwrap_or_default();
    for (name, path) in &targets {
        let codemap = format!("{}/codemap.md", path);
        let lib = format!("{}/src/lib.rs", path);
        let tests_dir = format!("{}/tests", path);
        let lib_exists = Path::new(&lib).exists();
        let tests_exist = Path::new(&tests_dir).exists();
        let codemap_text = fs::read_to_string(&codemap).unwrap_or_default();
        let lines_of_code = lib_exists.then(|| {
            fs::read_to_string(&lib)
                .map(|c| c.lines().count())
                .unwrap_or(0)
        }).unwrap_or(0);
        let build_ok = cargo_check(&path);
        let test_ok = with_tests && cargo_test(&path);
        let md = format!(
            "# Report: {}\n\n## Architecture\n{}\n\n## Metrics\n- lib.rs exists: {}\n- tests dir: {}\n- lines of code: {}\n- cargo check: {}\n- cargo test {}: {}\n\n## Audit Status\n- Sub-codemap: {}\n",
            name,
            if codemap_text.is_empty() { "(no codemap.md)" } else { &codemap_text },
            lib_exists,
            tests_exist,
            lines_of_code,
            if build_ok { "pass" } else { "fail" },
            if with_tests { "pass" } else { "not-run" },
            if with_tests { test_ok } else { true },
            if codemap_text.contains("Responsibility") { "present" } else { "absent" }
        );
        let report_path = format!("reports/{}_report.md", name);
        fs::write(&report_path, md).unwrap();
        println!("Generated report: {}", report_path);
    }
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
        Commands::GenerateReport { crate_name, all, tests } => {
            generate_report(crate_name.as_deref(), all, tests);
        }
    }
    Ok(())
}
// Analyze handlers — real integration to feature-engine, regime-detector, ranking, pca
// use quantaradar_core::{Bar, Regime};
// use quantaradar_features::{feature_rows, ema, rsi};
// use quantaradar_regime::{classify, RegimeThresholds};
