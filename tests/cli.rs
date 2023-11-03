use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command;

#[test]
/// Run core subcommand
/// -pansv -> path
/// -stats -> similarity
fn core_pansnno_sim() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/core.pansnno.sim.txt")
        .arg("--pansn")
        .arg("_");
    cmd.assert().success();
    Ok(())
}


#[test]
fn core_pansn_sim() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.core.pansn.sim.txt")
        .arg("--pansn")
        .arg("#");
    cmd.assert().success();
    Ok(())
}



#[test]
fn core_pansn_depth() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("core")
        .arg("--gfa")
        .arg("./data/example_data/testGraph_complex.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.core.pansn.depth.txt")
        .arg("--pansn")
        .arg("#")
        .arg("--stats")
        .arg("depth");
    cmd.assert().success();
    Ok(())
}





#[test]
fn id2int() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("id2int")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
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
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.ps.txt")
        .arg("--pansn")
        .arg("_");

    cmd.assert().success();
    Ok(())
}


#[test]
fn stats_graph_tsv() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/stats.graph.tsv");

    cmd.assert().success();
    Ok(())
}

#[test]
fn stats_graph_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("-y")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/stats.graph.yaml");

    cmd.assert().success();
    Ok(())
}



#[test]
fn stats_path_tsv() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--path")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/stats.path.tsv");

    cmd.assert().success();
    Ok(())
}

#[test]
fn stats_path_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--path")
        .arg("-y")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/stats.path.yaml");

    cmd.assert().success();
    Ok(())
}



#[test]
fn stats_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("stats")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
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
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.nodelist.txt");

    cmd.assert().success();
    Ok(())
}


#[test]
fn node_bootstrap() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.bootstrap.txt")
        .arg("--meta")
        .arg("testmeta.txt");
    cmd.assert().success();
    Ok(())
}

#[test]
fn node_bootstrap3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.bootstrap3.txt");
    cmd.assert().success();
    Ok(())
}


#[test]
fn node_bootstrap2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("bootstrap")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.bootstrap2.txt")
        .arg("--level")
        .arg("2");
    cmd.assert().success();
    Ok(())
}

#[test]
fn slidinw() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gfastats")?;
    cmd
        .arg("window")
        .arg("--gfa")
        .arg("./data/example_data/testGraph.gfa")
        .arg("--output")
        .arg("/home/svorbrugg/code/gfastats/data/test/test.stats.sw.txt")
        .arg("-s")
        .arg("2");

    cmd.assert().success();
    Ok(())
}

