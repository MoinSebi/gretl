use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::process::Command;
#[test]
#[should_panic]
fn node_bootstrap() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("--output")
        .arg("./data/test/testGraph/bootstrap/test.stats.bootstrap.txt");
    cmd.assert().success();
    fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap.txt").unwrap();
}

#[test]
#[should_panic]
fn node_bootstrap3() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("--output")
        .arg("./data/test/testGraph/bootstrap/test.stats.bootstrap3.txt");
    cmd.assert().success();
    //fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap3.txt")?;
}

#[test]
#[should_panic]
fn node_bootstrap2() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("--output")
        .arg("./data/test/testGraph/bootstrap/test.stats.bootstrap2.txt")
        .arg("--level")
        .arg("2");
    cmd.assert().success();
    fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap2.txt").unwrap();
}

#[test]
#[should_panic]
fn node_bootstrap4() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_compact_nopw.gfa")
        .arg("--output")
        .arg("./data/test/testGraph/bootstrap/test.stats.bootstrap4.txt")
        .arg("--nodes")
        .arg("data/nodes.txt");
    cmd.assert().success();
    //fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap4.txt")?;
}
