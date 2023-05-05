use std::path::Path;

use clap::Parser;
use regex::bytes::Regex;
use ui_test::{Config, CommandBuilder, OutputConflictHandling, Mode, Match};

/// Krabcake test runner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Write .stderr and .stdout files
    #[arg(long)]
    bless: bool,
}
fn main() {
    let args = Args::parse();

    let program = CommandBuilder::cmd("./runner.py");
    let regex = Regex::new(r"\S*?(--\d+--|==\d+==)\s*").unwrap();

    let config = Config {
        quiet: true,
        root_dir: "tests".into(),
        out_dir: Some("./build".into()),
        mode: Mode::Pass,
        program,
        output_conflict_handling: if args.bless {
             OutputConflictHandling::Bless
        } else {
            OutputConflictHandling::Error
        },
        stderr_filters: vec![(Match::Regex(regex), b"")],
        dependencies_crate_manifest_path: Some(Path::new("test_dependencies").join("Cargo.toml")),
        ..Config::default()
    };
    ui_test::run_tests(config).unwrap();
}
