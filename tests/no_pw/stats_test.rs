use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::fs::File;
use std::io::Read;
use std::process::Command;

#[test]
#[should_panic]
fn stats_graph_tsv() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("--output")
        .arg("./data/test/testGraph/stats/stats.graph.tsv");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/stats/stats.graph.tsv").unwrap();
}

#[test]
#[should_panic]
fn stats_graph_yaml() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("-y")
        .arg("--pansn")
        .arg("#")
        .arg("--output")
        .arg("./data/test/testGraph/stats/stats.graph.yaml");

    cmd.assert().success();
    let content: String = {
        let mut file = File::open("data/test/testGraph/stats/stats.graph.yaml").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    };
    assert!(content.contains("Paths: 6"));
    assert!(content.contains("Nodes: 8"));
    assert!(content.contains("Node length (average) [bp]: 7.125"));

    fs::remove_file("data/test/testGraph/stats/stats.graph.yaml").unwrap();
}

#[test]
#[should_panic]
fn stats_path_tsv() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("--path")
        .arg("--output")
        .arg("./data/test/testGraph/stats/stats.path.tsv");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/stats/stats.path.tsv").unwrap();
}

#[test]
#[should_panic]
fn stats_path_tsv_yaml() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("--path")
        .arg("--pansn")
        .arg("test")
        .arg("--output")
        .arg("./data/test/testGraph/stats/stats.path2.tsv");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/stats/stats.path2.tsv").unwrap();
}

#[test]
#[should_panic]
fn stats_path_yaml() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("--path")
        .arg("-y")
        .arg("--output")
        .arg("./data/test/testGraph/stats/stats.path.yaml");

    cmd.assert().success();
    let _content: String = {
        let mut file = File::open("../../data/test/testGraph/stats/stats.path.yaml").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        content
    };
    //fs::remove_file("data/test/testGraph/stats/stats.path.yaml")?;
}
