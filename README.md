# gretl - Graph evaluation toolkit
## Description 
```gretl``` is a tool for basic graph statistics using GFA format input. Our statistics are based on nodes, edges and paths. Walks can also be used, but will be represented as paths internally. Many commands do not work without paths/walk information. 

## Requirements on GFA file: 
GFA file is 
- format v1.0, v1.1 or v1.2.
- has numerical node ID

**Comment**:  
- Sorted node IDs are not required, but all "Jump" related statistics will be based on the order of the nodes in the GFA file.
- We recommend dense node ID, starting at 1 and end at the number of nodes +1. Memory efficient on multiple levels. 

## Installation: 

**Git**  
```
git clone https://github.com/MoinSebi/gretl  
cd gretl   
cargo build --release  
./target/release/gretl  
```
## Testing
We provide a small test suite to test the basic functionality of the tool. If you are interested in output format, check the data/test/yeast/ directory after running the following command.

```
cargo test
```

## Usage
### Stats

Calculate statistics on GFA file. A list of all stats can be found [here](paper/stats_explained.md). Please consider using the ```--pansn``` option to separate by sample and not path. Read more information about PanSN-spec [here](https://github.com/pangenome/PanSN-spec). 

Available options: 
- ```-bins``` Adjust number and size of bins. Histogram-like statistics which classify nodes by their length into bins. 
- ```-path``` Report statistcs for each path in the graph.
- ```-y``` Report output in YAML format.

Graph statistics also include "hybrid" statistics, which are average and standard deviation of all path statistics. All hybrid stats have the prefix "Path". A full list of all statistics be found in paper directory in this repository. 


**Example**
```text

./gretl stats -g /path/to/graph.gfa -o /path/to/output.txt
```
- [plot](scripts/plots/stats.path.scatter.pdf) using the provided scripts 
- Example output



### ID2INT
Convert any string-based node identifier to numeric values. Use ```odgi sort``` to sort the graph in pan-genomic order afterward. This will create more meaningful statistics when using ````gretl stats```. Nevertheless, numerical node IDs a required by any ```gretl``` command. 

**Example**
```
./gretl id2int -g /path/to/graph.gfa -o /path/to/output.gfa -d /path/to/dict.txt
```

**Comment:** 
This function will convert all IDs in the graph. Additional data in (segment-specific) tags will not be converted. 


### Node-list
Individual node statistics. Statistics provided: 
- Length
- Degree
- Depth
- Core

```text
./gretl node-list -g /path/to/graph.gfa -o /path/to/output.txt
```

- Example output

### Core
Compute similarity statistics of the graph. 

```
./gretl core -g /path/to/graph.gfa -o /path/to/output.txt
```
[core plot](scripts/plots/pancore.pdf)


### Path similarity (PS)

Calculate the similarity of paths in the graph

```
./gretl ps -g /path/to/graph.gfa -o /path/to/output.txt
```

[ps plot](scripts/plots/ps.similarity_path.seq.pdf)






### Feature
Filter nodes, edges or directed nodes (dirnode) based on input settings. The output can be used as input for gfa2bin. 

```text
./gretl feature -g /path/to/graph.gfa -o /path/to/nodes.txt -D 10 
```
### Path
Filter paths based on input settings. The output can be used as input for gfa2bin.

```text
./gretl feature -g /path/to/graph.gfa -o /path/to/nodes.txt -s "N/D ration" -m 10
```

### Bootstrap

We recommend bootstrapping a graphs in PanSN-spec. Use ```--nodes``` if the bootstrap should only run on a subset of nodes.  
You are able to adjust the number of bootstrap, only calculate one "level" or input a meta file as input. Examples are shown in the data/example_data/ directory.  
Meta files can be used to use the same "combinations" for multiple graphs. This only works of the paths/samples of the graphs are in the same order. 
```
./gretl bootstrap -g /path/to/graph.gfa -o /path/to/output.txt -n 20 
```
[bootstrap plot](scripts/plots/bootstrap.pdf)

### (Sliding, path) window
Summarizing the graph by a window of sequence in the path. Similar to approaches on reference genomes, but the statistics are based on the graph structure.


````
./gretl window -g /path/to/graph.gfa -o /path/to/output.txt -s 1000 --step 100
````
[window plot](scripts/plots/analysis.window.pdf)



### Nwindow
Summarizing the graph by a window of nodes. We iterate numerically over the nodes and calculate the statistics for each window. We start at the current node and move away from it based on provided edges, collecting the new nodes. We repeat this process starting at the "new" nodes until one of the following conditions is met:

- Jumps: A jumps is defined as difference between the current and the next node. Your input referees to the sum of all jumps in the window.
- Steps: A step it the number of moves we make in the graph. Your input is the maximum steps from the starting node. 
- Sequence: Limit the window by a sequence threshold. We stop if the sequence length is larger than the provided threshold. 

**Output**: You are able to return the number of collected nodes, the total number of jumps or the total sequence. Some combinations of input limitation and output do not gain any additional information. 


**How many nodes do I need to collect 1000 bp?**
```text
./gretl nwindow -g /path/to/graph.gfa -o /path/to/output.txt --sequence 1000 --node-number
```
[nwindow plot](scripts/plots/nwindow.node.pdf)


### Find
Find a specific node, dirnode, or edge in the graph and get the exact (sequence) position in the paths. Output is a BED file with the positions. You are able to add additional sequence ```-l``` on both sites, which can help if you want to realign to a database and the node is very small. 
```text
./gretl find -g /path/to/graph.gfa -o /path/to/output.txt --length 1000 -f feature.txt 
```
Example of feature file is data/example_data/dirnodes.txt


## Scripts 
We provide multiple jupyter notebooks to visualize the output of the tool. 

**Requirements**
- Jupyter
- Matplotlib
- Pandas
- Numpy
- Seaborn

