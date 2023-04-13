use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;

#[test]
fn core() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/test.core.txt")
        .arg("core");

    cmd.assert().success();
    Ok(())
}



#[test]
fn id2int() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/test.id2int.txt")
        .arg("id2int");

    cmd.assert().success();
    Ok(())
}


#[test]
fn ps() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/test.ps.txt")
        .arg("ps");

    cmd.assert().success();
    Ok(())
}
