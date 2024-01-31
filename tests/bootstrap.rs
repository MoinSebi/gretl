use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::process::Command;
#[test]
fn node_bootstrap() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/bootstrap/test.stats.bootstrap.txt")
        .arg("--meta")
        .arg("testmeta.txt");
    cmd.assert().success();
    fs::remove_file("data/test/bootstrap/test.stats.bootstrap.txt")?;

    Ok(())
}

#[test]
fn node_bootstrap3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/bootstrap/test.stats.bootstrap3.txt");
    cmd.assert().success();
    fs::remove_file("data/test/bootstrap/test.stats.bootstrap3.txt")?;

    Ok(())
}

#[test]
fn node_bootstrap2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/bootstrap/test.stats.bootstrap2.txt")
        .arg("--level")
        .arg("2");
    cmd.assert().success();
    fs::remove_file("data/test/bootstrap/test.stats.bootstrap2.txt")?;

    Ok(())
}
