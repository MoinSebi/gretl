use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;

#[test]
fn core() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("core")
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.core.txt")
        .arg("--pansn")
        .arg("_");
    cmd.assert().success();
    Ok(())
}



#[test]
fn id2int() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("id2int")
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.id2int.txt");

    cmd.assert().success();
    Ok(())
}


#[test]
fn ps() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("ps")
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.ps.txt")
        .arg("-s")
        .arg("_");

    cmd.assert().success();
    Ok(())
}


#[test]
fn stats() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("stats")
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.txt");

    cmd.assert().success();
    Ok(())
}


#[test]
fn stats_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("stats")
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.path.txt")
        .arg("--path")
        .arg("-y");

    cmd.assert().success();
    Ok(())
}

#[test]
fn node_list_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("node-list")
        .arg("--gfa")
        .arg("/home/svorbrugg/code/bvd/data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.nodelist.txt");

    cmd.assert().success();
    Ok(())
}
