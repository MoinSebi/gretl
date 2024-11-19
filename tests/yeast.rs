// Imports
use assert_cmd::prelude::*; // Add methods on commands
use std::fs::File;
use std::io::Read;
use std::process::Command;
use tempfile::tempdir;

// Filename for all tests
const FILENAME: &str = "./data/example_data/chr5.yeast.gfa";
const OUTPUT: &str = "yeast";

//---
// Stats
#[test]
fn stats_graph_tsv() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".graph.tsv");
    cmd.arg("stats")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn stats_graph_yaml() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".graph.yaml");
    cmd.arg("stats")
        .arg("--gfa")
        .arg(FILENAME)
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
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".path.tsv");
    cmd.arg("stats")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--path")
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn stats_path_tsv_yaml() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".path2.tsv");
    cmd.arg("stats")
        .arg("--gfa")
        .arg(FILENAME)
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
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".path.yaml");
    cmd.arg("stats")
        .arg("--gfa")
        .arg(FILENAME)
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

#[test]
/// Run core subcommand
/// -pansv -> path
/// -stats -> similarity
fn analysis_test_core_no_pansn_similarity() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir
        .path()
        .join(OUTPUT.to_string() + ".core.pansnno.sim.txt");
    cmd.arg("core")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--pansn")
        .arg("_");

    cmd.assert().success();
}

#[test]
fn analysis_test_core_pansn_similarity() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".core.pansn.sim");
    cmd.arg("core")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--pansn")
        .arg("#");

    cmd.assert().success();
}

#[test]
fn analysis_test_core_pansn_similarity2() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir
        .path()
        .join(OUTPUT.to_string() + ".core.pansn.sim.txt");
    cmd.arg("core")
        .arg("--gfa")
        .arg(FILENAME)
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
    let output = tmp_dir
        .path()
        .join(OUTPUT.to_string() + ".core.pansn.depth.txt");
    cmd.arg("core")
        .arg("--gfa")
        .arg(FILENAME)
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
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".id2int.gfa");
    cmd.arg("id2int")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--dict")
        .arg(tmp_dir.path().join(OUTPUT.to_string() + ".id2int.dict.txt"));

    cmd.assert().success();
}

#[test]
fn analysis_ps_tg() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".ps.txt");
    cmd.arg("ps")
        .arg("--gfa")
        .arg(FILENAME)
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
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".nodelist2.txt");
    cmd.arg("node-list")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn analysis_test_sliding_window_2s() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".window.txt");
    cmd.arg("window")
        .arg("--gfa")
        .arg(FILENAME)
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
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".nwindow.txt");
    cmd.arg("nwindow")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--output")
        .arg(output.to_str().unwrap());

    cmd.assert().success();
}

#[test]
fn analysis_find_tg() {
    let mut cmd = Command::cargo_bin("gretl").unwrap();
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".find.tg.txt");
    cmd.arg("find")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--length")
        .arg("200")
        .arg("--features")
        .arg("./data/example_data/dirnodes.txt");

    cmd.assert().success();
}

//---
// Bootstrap
#[test]
fn node_bootstrap1() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".bootstrap1.txt");
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--output")
        .arg(output.to_str().unwrap());
    cmd.assert().success();

    Ok(())
}

#[test]
fn node_bootstrap2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".bootstrap2.txt");
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--pansn")
        .arg("#");
    cmd.assert().success();

    Ok(())
}

#[test]
fn node_bootstrap3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    let tmp_dir = tempdir().expect("Failed to create temp dir");
    let output = tmp_dir.path().join(OUTPUT.to_string() + ".bootstrap3.txt");
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg(FILENAME)
        .arg("--output")
        .arg(output.to_str().unwrap())
        .arg("--nodes")
        .arg("data/nodes.txt");
    cmd.assert().success();

    Ok(())
}
