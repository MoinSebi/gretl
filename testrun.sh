#!/bin/bash


cargo run stats -g ./data/example_data/chr5.yeast.gfa -o ./data/test/yeast/stats/stats.graph.tsv
cargo run stats -g ./data/example_data/chr5.yeast.gfa -o ./data/test/yeast/stats/stats.graph.yaml -y
cargo run stats -g ./data/example_data/chr5.yeast.gfa -o ./data/test/yeast/stats/stats.graph.path.tsv -p
cargo run stats -g ./data/example_data/chr5.yeast.gfa -o ./data/test/yeast/stats/stats.graph.path.yaml -y -p
cargo run stats -g ./data/example_data/chr5.yeast.gfa -o ./data/test/yeast/stats/stats.graph.path.pansn.yaml -y -p --pansn '#'


cargo run feature -g ./data/example_data/chr5.yeast.gfa -o ./data//test/yeast/feature_paths/stats.node.s10.txt -l 5
cargo run path -g ./data/example_data/chr5.yeast.gfa -o ./data//test/yeast/feature_paths/stats.path.m10M40.txt -s Sequence [bp] -m 10 -M 40


cargo run bootstrap -g ./data/example_data/chr5.yeast.gfa -o ./data//test/yeast/bootstrap/test.stats.bootstrap.txt
cargo run bootstrap -g ./data/example_data/chr5.yeast.gfa -o ./data//test/yeast/bootstrap/test.stats.bootstrap.txt --level 2
cargo run bootstrap -g ./data/example_data/chr5.yeast.gfa -o ./data//test/yeast/bootstrap/test.stats.bootstrap.txt --nodes data/nodes.txt


cargo run core -g ./data/example_data/chr5.yeast.gfa -o /data/test/yeast/analysis/analysis.yeast.core.pansnno.sim.txt --pansn '#'
cargo run id2int -g ./data/example_data/chr5.yeast.gfa -o ./data//test/testGraph/analysis/id2int.txt
cargo run ps -g ./data/example_data/chr5.yeast.gfa -o ./data/test/yeast/analysis/ps.txt --pansn '_'
cargo run node-list -g ./data/example_data/chr5.yeast.gfa -o ./data/test/yeast/analysis/nodelist.txt
cargo run window -g ./data/example_data/chr5.yeast.gfa -o ./data/test/yeast/analysis/window.txt -w 5000
cargo run nwindow -g ./data/example_data/chr5.yeast.gfa -o ./data/test/yeast/analysis/nwindow.txt
cargo run find -g ./data/example_data/chr5.yeast.gfa -o ./data/test/yeast/analysis/find.txt --length 200 --features ./data/example_data/dirnodes.txt