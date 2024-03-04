# gretl - Graph evaluation toolkit
## Description 
Small tool for basic graph statistics using GFA format. Our statistics are based on nodes, edges and paths. Walks can also be used, 

## Installation: 

**Git**  
```
git clone https://github.com/MoinSebi/gretl  
cd gretl   
cargo build --release  
./target/release/gretl  
```
## Testing
We provide a small test suite to test the basic functionality of the tool. If you are interested in output format, check the data/test/yeast/ directory after running this command.

```
cargo test
```

## Usage
### Stats

Calculate graph or path (-p) statistics. Please consider using the ```--pansn``` option to separate by sample and not path. You are able to adjust the bins if you want using the ```--bins``` option.  

Graph statistics also include "hybrid" statistics, which are average and standard deviation of path statistics. All hybrid stats have the prefix "Path". A full list of all statistics be found in paper directory in this repository. 

```
./gretl stats -g /path/to/graph.gfa -o /path/to/output.txt
```
[stats path plot](scripts/plots/stats.path.scatter.pdf)


### ID2INT
Convert any string-based node identifier to numeric values. Use ```odgi sort``` to sort the graph in pan-genomic order. This will create more meaningful statistics.

```
./gretl id2int -g /path/to/graph.gfa -o /path/to/output.gfa -d /path/to/dict.txt
```


### Node-list
Caculate statistics for each node in the graph. Return a list of nodes with their statistics. 
Possible statistics are: 
- Length
- Degree
- Depth
- Core

```text
./gretl node-list -g /path/to/graph.gfa -o /path/to/output.txt
```

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

We recommend bootstrapping a graphs in PanSN-spec. Use ```--nodes``` if the bootstrap should only run on a subset of nodes (e.g. gene vs intergenic).  
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





## Scripts 
We provide multiple jupyter notebooks to visualize the output of the tool. 

**Requirements**
- Jupyter
- Matplotlib
- Pandas
- Numpy
- Seaborn

