use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::fs::File;
use std::io::Read;
use std::process::Command;

#[test]
fn stats_graph_tsv() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data/test/yeast/stats/yeast.chr5.graph.tsv");

    cmd.assert().success();
    fs::remove_file("data/test/yeast/stats/yeast.chr5.graph.tsv")?;

    Ok(())
}

#[test]
fn stats_graph_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("-y")
        .arg("--pansn")
        .arg("#")
        .arg("--output")
        .arg("./data/test/yeast/stats/yeast.chr5.graph.yaml");

    cmd.assert().success();
    let content: String = {
        let mut file = File::open("data/test/yeast/stats/yeast.chr5.graph.yaml")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        content
    };
    assert!(content.contains("Paths: 5"));

    Ok(())
}

#[test]
fn stats_path_tsv() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--path")
        .arg("--output")
        .arg("./data/test/yeast/stats/yeast.chr5.path.tsv");

    cmd.assert().success();

    Ok(())
}

#[test]
fn stats_path_tsv_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--path")
        .arg("--pansn")
        .arg("test")
        .arg("--output")
        .arg("./data/test/yeast/stats/yeast.chr5.path2.tsv");

    cmd.assert().success();

    Ok(())
}

#[test]
fn stats_path_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--path")
        .arg("-y")
        .arg("--output")
        .arg("./data/test/yeast/stats/yeast.chr5.path.yaml");

    cmd.assert().success();
    let _content: String = {
        let mut file = File::open("./data/test/yeast/stats/yeast.chr5.path.yaml")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        content
    };
    Ok(())
}
