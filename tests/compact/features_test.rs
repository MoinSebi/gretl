use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn stats_graph_feature() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("feature_paths.stats.node.s10.txt");
    cmd.arg("feature")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("-l")
        .arg("5");

    cmd.assert().success();
    let content = fs::read_to_string(output.to_str().unwrap()).unwrap();
    assert_eq!(content, "1\n4\n5\n8\n9\n");
}

#[test]
fn stats_graph_path() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("feature_paths.stats.path.m10M40.txt");
    cmd.arg("path")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("-s")
        .arg("Sequence [bp]")
        .arg("-m")
        .arg("10")
        .arg("-M")
        .arg("40");

    cmd.assert().success();
}
