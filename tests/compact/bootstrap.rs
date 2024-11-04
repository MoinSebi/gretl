use assert_cmd::prelude::*; // Add methods on commands

use std::process::Command;
use tempfile::tempdir;

#[test]
fn node_bootstrap() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("test.stats.bootstrap.txt");
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn node_bootstrap3() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("test.stats.bootstrap3.txt");
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn node_bootstrap2() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("test.stats.bootstrap2.txt");

    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--level")
        .arg("2");

    cmd.assert().success();
}

#[test]
fn node_bootstrap4() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("test.stats.bootstrap4.txt");
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--nodes")
        .arg("data/nodes.txt");
    cmd.assert().success();
}
