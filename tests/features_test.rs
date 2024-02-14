use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::process::Command;

#[test]
fn stats_graph_feature() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("feature")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/testGraph/feature_paths/stats.node.s10.txt")
        .arg("-l")
        .arg("5");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/feature_paths/stats.node.s10.txt")?;

    Ok(())
}

#[test]
fn stats_graph_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("path")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/testGraph/feature_paths/stats.path.m10M40.txt")
        .arg("-s")
        .arg("Sequence [bp]")
        .arg("-m")
        .arg("10")
        .arg("-M")
        .arg("40");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/feature_paths/stats.path.m10M40.txt")?;

    Ok(())
}


#[test]
fn stats_graph_feature_yeast() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("feature")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/yeast/feature_paths/stats.node.s10.txt")
        .arg("-l")
        .arg("5");

    cmd.assert().success();
    fs::remove_file("data/test/yeast/feature_paths/stats.node.s10.txt")?;

    Ok(())
}

#[test]
fn stats_graph_path_yeast() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("path")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/yeast/feature_paths/stats.path.m10M40.txt")
        .arg("-s")
        .arg("Sequence [bp]")
        .arg("-m")
        .arg("10")
        .arg("-M")
        .arg("40");

    cmd.assert().success();
    fs::remove_file("data/test/yeast/feature_paths/stats.path.m10M40.txt")?;

    Ok(())
}

