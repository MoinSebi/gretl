use assert_cmd::prelude::*; // Add methods on commands

use std::process::Command;

#[test]
/// Run core subcommand
/// -pansv -> path
/// -stats -> similarity
fn analysis_test_core_no_pansn_similarity() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data//test/yeast/analysis/yeast.chr5.test.core.pansnno.sim.txt")
        .arg("--pansn")
        .arg("_");
    cmd.assert().success();

    Ok(())
}

#[test]
#[cfg(feature = "a1")]
fn analysis_test_core_pansn_similarity() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data//test/yeast/analysis/yeast.chr5.test.core.pansn.sim.txt")
        .arg("--pansn")
        .arg("#");
    cmd.assert().success();

    Ok(())
}

#[test]
#[cfg(feature = "a1")]
fn analysis_test_core_pansn_similarity2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data//test/yeast/analysis/yeast.chr5.test.core.pansn.sim.txt")
        .arg("--pansn")
        .arg("#");
    cmd.assert().success();

    Ok(())
}

#[test]
fn analysis_test_core_pansn_depth() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data//test/yeast/analysis/yeast.chr5.core.pansn.depth.txt")
        .arg("--pansn")
        .arg("#")
        .arg("--stats")
        .arg("depth");
    cmd.assert().success();

    Ok(())
}

#[test]
fn id2int_test_graph() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("id2int")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data//test/yeast/analysis/yeast.chr5.id2int.gfa")
        .arg("--dict")
        .arg("./data/example_data/chr5.yeast.id2int.txt");

    cmd.assert().success();

    Ok(())
}

#[test]
fn analysis_ps_tg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("ps")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data//test/yeast/analysis/yeast.chr5.ps.txt")
        .arg("--pansn")
        .arg("_");

    cmd.assert().success();

    Ok(())
}

#[test]
fn analysis_node_list_path_tg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("node-list")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data//test/yeast/analysis/yeast.chr5.nodelist2.txt");

    cmd.assert().success();

    Ok(())
}

#[test]
fn analysis_test_sliding_window_2s() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("window")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data//test/yeast/analysis/yeast.chr5.window.txt")
        .arg("-w")
        .arg("2");

    cmd.assert().success();

    Ok(())
}

#[test]
fn analysis_sliding_nwindow_2s_tg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("nwindow")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data/test/yeast/analysis/yeast.chr5.nwindow.txt");

    cmd.assert().success();

    Ok(())
}

#[test]
fn analysis_find_tg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("find")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("./data/test/yeast/analysis/yeast.chr5.find.tg.txt")
        .arg("--length")
        .arg("200")
        .arg("--features")
        .arg("./data/example_data/dirnodes.txt");
    cmd.assert().success();

    Ok(())
}
