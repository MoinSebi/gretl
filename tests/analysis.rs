use std::fs;
use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;

#[test]
/// Run core subcommand
/// -pansv -> path
/// -stats -> similarity
fn core_no_pansn_similarity() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/analysis/core.pansnno.sim.txt")
        .arg("--pansn")
        .arg("_");
    cmd.assert().success();
    fs::remove_file("data/test/analysis/core.pansnno.sim.txt")?;

    Ok(())
}

#[test]
fn core_pansn_similarity() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/analysis/core.pansn.sim.txt")
        .arg("--pansn")
        .arg("#");
    cmd.assert().success();
    fs::remove_file("data/test/analysis/core.pansn.sim.txt")?;

    Ok(())
}

#[test]
fn core_pansn_depth() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/analysis/core.pansn.depth.txt")
        .arg("--pansn")
        .arg("#")
        .arg("--stats")
        .arg("depth");
    cmd.assert().success();
    fs::remove_file("data/test/analysis/core.pansn.depth.txt")?;

    Ok(())
}

#[test]
fn id2int() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("id2int")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/analysis/id2int.txt");

    cmd.assert().success();
    fs::remove_file("data/test/analysis/id2int.txt")?;

    Ok(())
}

#[test]
fn ps() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("ps")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/analysis/ps.txt")
        .arg("--pansn")
        .arg("_");

    cmd.assert().success();
    fs::remove_file("data/test/analysis/ps.txt")?;

    Ok(())
}

#[test]
fn node_list_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("node-list")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/analysis/nodelist.txt");

    cmd.assert().success();
    fs::remove_file("data/test/analysis/nodelist.txt")?;

    Ok(())
}

#[test]
fn sliding_window_2s() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("window")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/analysis/slidingwindow.txt")
        .arg("-s")
        .arg("2");

    cmd.assert().success();
    fs::remove_file("data/test/analysis/slidingwindow.txt")?;

    Ok(())
}
