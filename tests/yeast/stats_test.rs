use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn stats_graph_tsv() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("yeast.chr5.graph.tsv");
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn stats_graph_yaml() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("yeast.chr5.graph.yaml");
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("-y")
        .arg("--pansn")
        .arg("#")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
    let content: String = {
        let mut file = File::open(output.to_str().unwrap()).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    };
    assert!(content.contains("Paths: 5"));
}

#[test]
fn stats_path_tsv() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("yeast.chr5.path.tsv");
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--path")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn stats_path_tsv_yaml() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("yeast.chr5.path2.tsv");
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--path")
        .arg("--pansn")
        .arg("test")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn stats_path_yaml() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("yeast.chr5.path.yaml");
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--path")
        .arg("-y")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
    let _content: String = {
        let mut file = File::open(output.to_str().unwrap()).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    };
}
