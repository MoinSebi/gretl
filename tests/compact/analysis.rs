use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
/// Run core subcommand
/// -pansv -> path
/// -stats -> similarity
fn analysis_test_core_no_pansn_similarity() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("analysis.test.core.pansnno.sim.txt");
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--pansn")
        .arg("_");

    cmd.assert().success();
}

#[test]
#[cfg(feature = "a1")]
fn analysis_test_core_pansn_similarity() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("analysis.test.core.pansn.sim.txt");
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--pansn")
        .arg("#");

    cmd.assert().success();
}

#[test]
#[cfg(feature = "a1")]
fn analysis_test_core_pansn_similarity2() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("analysis.test.core.pansn.sim.txt");
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--pansn")
        .arg("#");

    cmd.assert().success();
}

#[test]
fn analysis_test_core_pansn_depth() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("core.pansn.depth.txt");
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--pansn")
        .arg("#")
        .arg("--stats")
        .arg("depth");

    cmd.assert().success();
}

#[test]
fn id2int_test_graph() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("id2int.txt");
    cmd.arg("id2int")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn analysis_ps_tg() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("ps.txt");
    cmd.arg("ps")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--pansn")
        .arg("_");

    cmd.assert().success();
}

#[test]
fn analysis_node_list_path_tg() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("nodelist2.txt");
    cmd.arg("node-list")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn analysis_test_sliding_window_2s() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("analysis.window.txt");
    cmd.arg("window")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("-w")
        .arg("2");

    cmd.assert().success();
}

#[test]
fn analysis_sliding_nwindow_2s_tg() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("analysis.nwindow.txt");
    cmd.arg("nwindow")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn analysis_find_tg() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join("analysis.find.tg.txt");
    cmd.arg("find")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--length")
        .arg("200")
        .arg("--features")
        .arg("./data/example_data/dirnodes.txt");

    cmd.assert().success();
}
