//! Integration test for `--metadata-cache-stats` combined with
//! `--no-metadata-cache`.
//!
//! No existing test exercises either flag. This spawns the binary directly,
//! as `tests/extract_subcommand.rs` does, using the checked-in fixtures
//! under `tests/wasm/`.

use std::path::PathBuf;
use std::process::Command;

fn wasm(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("wasm")
        .join(name)
}

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_soroban-upgrade-safeguard"))
}

#[test]
fn metadata_cache_stats_reports_bypass_with_no_metadata_cache() {
    let output = bin()
        .arg(wasm("v1.wasm"))
        .arg(wasm("v1.wasm"))
        .arg("--metadata-cache-stats")
        .arg("--no-metadata-cache")
        .output()
        .expect("failed to run binary");

    let stderr = String::from_utf8(output.stderr).expect("stderr was not valid UTF-8");

    assert!(
        stderr.contains("metadata-cache: bypassed (--no-metadata-cache)"),
        "stderr must report the bypass. stderr:\n{stderr}"
    );
}
