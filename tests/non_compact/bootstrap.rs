use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::process::Command;
#[test]
fn node_bootstrap() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/bootstrap/test.stats.bootstrap.txt");
    cmd.assert().success();
    fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap.txt")?;

    Ok(())
}

#[test]
fn node_bootstrap3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/bootstrap/test.stats.bootstrap3.txt");
    cmd.assert().success();
    //fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap3.txt")?;

    Ok(())
}

#[test]
fn node_bootstrap2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/bootstrap/test.stats.bootstrap2.txt")
        .arg("--level")
        .arg("2");
    cmd.assert().success();
    fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap2.txt")?;

    Ok(())
}

#[test]
fn node_bootstrap4() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gretl")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("./data//test/testGraph/bootstrap/test.stats.bootstrap4.txt")
        .arg("--nodes")
        .arg("data/nodes.txt");
    cmd.assert().success();
    //fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap4.txt")?;

    Ok(())
}
