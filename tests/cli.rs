use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;

#[test]
fn core() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.core.txt")
        .arg("--pansn")
        .arg("_")
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
        .arg("/home/svorbrugg/code/gfastats/data/test/test.id2int.txt")
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
        .arg("/home/svorbrugg/code/gfastats/data/test/test.ps.txt")
        .arg("-s")
        .arg("_")
        .arg("ps");

    cmd.assert().success();
    Ok(())
}


#[test]
fn stats() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.txt")
        .arg("stats");

    cmd.assert().success();
    Ok(())
}


#[test]
fn stats_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.path.txt")
        .arg("stats")
        .arg("--path")
        .arg("-y");

    cmd.assert().success();
    Ok(())
}

#[test]
fn node_list_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.nodelist.txt")
        .arg("node-list");

    cmd.assert().success();
    Ok(())
}
