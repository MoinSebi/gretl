use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::process::Command;
#[test]
fn node_bootstrap() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/testGraph/bootstrap/test.stats.bootstrap.txt")
        .arg("--meta")
        .arg("testmeta.txt");
    cmd.assert().success();
    fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap.txt")?;

    Ok(())
}

#[test]
fn node_bootstrap3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/testGraph/bootstrap/test.stats.bootstrap3.txt");
    cmd.assert().success();
    fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap3.txt")?;

    Ok(())
}

#[test]
fn node_bootstrap2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/testGraph/bootstrap/test.stats.bootstrap2.txt")
        .arg("--level")
        .arg("2");
    cmd.assert().success();
    fs::remove_file("data/test/testGraph/bootstrap/test.stats.bootstrap2.txt")?;

    Ok(())
}

#[test]
fn node_bootstrap_yeast() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/yeast/bootstrap/test.stats.bootstrap.txt")
        .arg("--meta")
        .arg("data/testmeta.txt");
    cmd.assert().success();
    fs::remove_file("data/test/yeast/bootstrap/test.stats.bootstrap.txt")?;

    Ok(())
}

#[test]
fn node_bootstrap3_yeast() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/yeast/bootstrap/test.stats.bootstrap3.txt");
    cmd.assert().success();
    //fs::remove_file("data/test/yeast/bootstrap/test.stats.bootstrap3.txt")?;

    Ok(())
}

#[test]
fn node_bootstrap2_yeast() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/yeast/bootstrap/test.stats.bootstrap2.txt")
        .arg("--level")
        .arg("2");
    cmd.assert().success();
    fs::remove_file("data/test/yeast/bootstrap/test.stats.bootstrap2.txt")?;

    Ok(())
}

