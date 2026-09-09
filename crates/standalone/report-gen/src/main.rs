use clap::Parser;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "quantaradar-report-gen", about = "Generate per-crate markdown reports")]
struct Cli {
    #[arg(short, long)]
    crate_name: Option<String>,
    #[arg(short = 'a', long)]
    all: bool,
    #[arg(short, long)]
    tests: bool,
}

fn crates() -> Vec<(String, String)> {
    let mut out = vec![];
    for base in ["crates/shared", "crates/standalone"] {
        if Path::new(base).exists() {
            for d in fs::read_dir(base).unwrap_or_else(|_| fs::read_dir("").unwrap_or_else(|_| fs::read_dir(".").unwrap())) {
                if let Ok(e) = d {
                    let name = e.file_name().to_string_lossy().to_string();
                    if e.path().join("Cargo.toml").exists() {
                        out.push((name.clone(), format!("{}/{}", base, name)));
                    }
                }
            }
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn run_cargo(path: &str, args: &[&str]) -> bool {
    Command::new("cargo")
        .args(args)
        .arg("--manifest-path")
        .arg(format!("{}/Cargo.toml", path))
        .current_dir(path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn main() {
    let cli = Cli::parse();
    let entries: Vec<(String, String)> = if let Some(ref n) = cli.crate_name {
        vec![(n.clone(), format!("crates/shared/{}", n))]
    } else if cli.all {
        crates()
    } else {
        vec![]
    };
    if entries.is_empty() {
        eprintln!("use --crate <NAME> or --all");
        std::process::exit(2);
    }
    fs::create_dir_all("reports").unwrap();
    for (name, base) in entries {
        let codemap = format!("{}/codemap.md", base);
        let lib = format!("{}/src/lib.rs", base);
        let tests = format!("{}/tests", base);
        let cmap_text = fs::read_to_string(&codemap).unwrap_or_default();
        let lib_text = fs::read_to_string(&lib).unwrap_or_default();
        let build_ok = run_cargo(&base, &["check"]);
        let test_ok = cli.tests && run_cargo(&base, &["test", "--quiet"]);
        let report = format!(
            "# Report: {}\n\n## Architecture\n{}\n\n## Metrics\n- lib.rs exists: {}\n- tests dir: {}\n- lines of code: {}\n- cargo check: {}\n- cargo test: {}\n\n## Audit\n- sub-codemap: {}\n",
            name,
            if cmap_text.is_empty() { "(no codemap.md)" } else { &cmap_text },
            Path::new(&lib).exists(),
            Path::new(&tests).exists(),
            lib_text.lines().count(),
            if build_ok { "pass" } else { "fail" },
            if cli.tests { if test_ok { "pass" } else { "fail" } } else { "not-run" },
            if cmap_text.contains("Responsibility") { "present" } else { "absent" },
        );
        fs::write(format!("reports/{}_report.md", name), report).unwrap();
        println!("generated reports/{}_report.md", name);
    }
}
