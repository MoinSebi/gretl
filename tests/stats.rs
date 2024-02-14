use assert_cmd::prelude::*; // Add methods on commands
use std::fs;
use std::process::Command;

#[test]
fn stats_graph_tsv() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/testGraph/stats/stats.graph.tsv");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/stats/stats.graph.tsv")?;

    Ok(())
}

#[test]
fn stats_graph_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("-y")
        .arg("--pansn")
        .arg("#")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/testGraph/stats/stats.graph.yaml");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/stats/stats.graph.yaml")?;

    Ok(())
}

#[test]
fn stats_path_tsv() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--path")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/testGraph/stats/stats.path.tsv");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/stats/stats.path.tsv")?;

    Ok(())
}

#[test]
fn stats_path_tsv_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--path")
        .arg("--pansn")
        .arg("test")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/testGraph/stats/stats.path2.tsv");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/stats/stats.path2.tsv")?;

    Ok(())
}

#[test]
fn stats_path_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--path")
        .arg("-y")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/testGraph/stats/stats.path.yaml");

    cmd.assert().success();
    fs::remove_file("data/test/testGraph/stats/stats.path.yaml")?;
    Ok(())
}





#[test]
fn stats_graph_tsv_yeast() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/yeast/stats/stats.graph.tsv");

    cmd.assert().success();

    Ok(())
}

#[test]
fn stats_graph_yaml_yeast() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("-y")
        .arg("--pansn")
        .arg("#")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/yeast/stats/stats.graph.yaml");

    cmd.assert().success();

    Ok(())
}

#[test]
fn stats_path_tsv_yeast() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--path")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/yeast/stats/stats.path.tsv");

    cmd.assert().success();

    Ok(())
}

#[test]
fn stats_path_tsv_yaml_yeast() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--path")
        .arg("--pansn")
        .arg("test")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/yeast/stats/stats.path2.tsv");

    cmd.assert().success();

    Ok(())
}

#[test]
fn stats_path_yaml_yeast() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd.arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/chr5.yeast.gfa")
        .arg("--path")
        .arg("-y")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/yeast/stats/stats.path.yaml");

    cmd.assert().success();
    Ok(())
}
