use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::process::Command;

#[test]
#[should_panic]
fn stats_graph_feature() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("feature")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/feature_paths/stats.node.s10.txt")
        .arg("-l")
        .arg("5");

    cmd.assert().success();
    let content = fs::read_to_string("data/test/testGraph/feature_paths/stats.node.s10.txt").unwrap();
    assert_eq!(content, "1\n4\n5\n8\n9\n");

    fs::remove_file("data/test/testGraph/feature_paths/stats.node.s10.txt").unwrap();

}

#[test]
#[should_panic]
fn stats_graph_path()  {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("path")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/feature_paths/stats.path.m10M40.txt")
        .arg("-s")
        .arg("Sequence [bp]")
        .arg("-m")
        .arg("10")
        .arg("-M")
        .arg("40");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/feature_paths/stats.path.m10M40.txt").unwrap();

}
