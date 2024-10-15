use assert_cmd::prelude::*; // Add methods on commands
use std::fs;

use std::process::Command;

#[test]
#[should_panic]
/// Run core subcommand
/// -pansv -> path
/// -stats -> similarity
fn analysis_test_core_no_pansn_similarity() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_non-num.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/analysis/analysis.test.core.pansnno.sim.txt")
        .arg("--pansn")
        .arg("_");
    cmd.assert().success();
    fs::remove_file("data/test/testGraph/analysis/analysis.test.core.pansnno.sim.txt").unwrap();
}

#[test]
#[should_panic]
fn analysis_test_core_pansn_depth() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_non-num.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/analysis/core.pansn.depth.txt")
        .arg("--pansn")
        .arg("#")
        .arg("--stats")
        .arg("depth");
    cmd.assert().success();
    fs::remove_file("data/test/testGraph/analysis/core.pansn.depth.txt").unwrap();
}

#[test]
#[should_panic]
fn id2int_test_graph() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("id2int")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_non-num.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/analysis/id2int.txt");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/analysis/id2int.txt").unwrap();
}

#[test]
#[should_panic]
fn analysis_ps_tg() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("ps")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_non-num.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/analysis/ps.txt")
        .arg("--pansn")
        .arg("_");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/analysis/ps.txt").unwrap();
}

#[test]
#[should_panic]
fn analysis_node_list_path_tg() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("node-list")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_non-num.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/analysis/nodelist2.txt");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/analysis/nodelist2.txt").unwrap();
}

#[test]
#[should_panic]
fn analysis_test_sliding_window_2s() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("window")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_non-num.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/analysis/analysis.window.txt")
        .arg("-w")
        .arg("2");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/analysis/analysis.window.txt").unwrap();
}

#[test]
#[should_panic]
fn analysis_sliding_nwindow_2s_tg() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("nwindow")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_non-num.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/analysis/analysis.nwindow.txt");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/analysis/analysis.nwindow.txt").unwrap();
}

#[test]
#[should_panic]
fn analysis_find_tg() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    cmd.arg("find")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_non-num.gfa")
        .arg("--output")
        .arg("./data/test/testGraph/analysis/analysis.find.tg.txt")
        .arg("--length")
        .arg("200")
        .arg("--features")
        .arg("./data/example_data/dirnodes.txt");
    cmd.assert().success();
    fs::remove_file("./data/test/testGraph/analysis/analysis.find.tg.txt").unwrap();
}
