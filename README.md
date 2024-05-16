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

Calculate statistics on GFA file. A list of all stats can be found [here](paper/stats_explained.md). Please consider using the ```--pansn``` option to group the paths by sample. Read more information about PanSN-spec [here](https://github.com/pangenome/PanSN-spec). 

Available options: 
- ```-bins``` Adjust number and size of bins. Histogram-like statistics which classify nodes by their length into bins. 
- ```-path``` Report statistics for each path in the graph.
- ```-y``` Report output in YAML format (default is tsv). 

Graph statistics also include "hybrid" statistics, which are average and standard deviation of all path statistics. All hybrid stats have the prefix "Path". A full list of all statistics be found in paper directory in this repository. 


**Example**
```text

./gretl stats -g /path/to/graph.gfa -o /path/to/output.txt
```

**Result**
- TSV or YAML file with statistics
- Merge the output of multiple graphs to compare them. 
- Example comparison: [plot](scripts/plots/stats.path.scatter.pdf) 
- Example output



### ID2INT
Convert any string-based node identifier to numeric values. Use ```odgi sort``` to sort the graph in pan-genomic order, which will create more meaningful statistics in ```gretl stats``` (see above). Nevertheless, numerical node IDs a required by any ```gretl``` command. 

Available options:
- ```-d, --dict <dict>``` Write new and old IDs to a plain text file. 

**Example**
```
./gretl id2int -g /path/to/graph.gfa -o /path/to/output.gfa -d /path/to/dict.txt
```

**Result**: 
- GFA file with numerical node IDs


**Comment:** 
This function will convert all IDs in the graph. Additional data in (segment-specific) tags will not be converted. 


### Node-list
Individual node statistics. Statistics provided: 
- Length
- Degree
- Depth
- Core


Length and degree are based on the graph itself, while depth and core are based on the paths.

**Example**
```text
./gretl node-list -g /path/to/graph.gfa -o /path/to/output.txt
```

**Result**
- Example output

**Comment**
The information of the reported table can be used as a individual lookup or to create own window-like statistics (over nodes). 

### Core
Compute user-defined statistics of the graph (```-s```). Calculate the statistics for each node and summarize for each possible value the number of nodes and sequence. In a additional file ("...private.txt") we report for each path the amount of nodes and sequence sole present by this sample. 

Available options:
- ```-s, --stats <statistics>```. Define the statistics you want to summarize (see above) [default: similarity].


```
./gretl core -g /path/to/graph.gfa -o /path/to/output.txt
```


**Result**
- [core plot](scripts/plots/pancore.pdf)

### Path similarity (PS)

Calculate for each path the amount of nodes and sequence at each similarity level. 

```
./gretl ps -g /path/to/graph.gfa -o /path/to/output.txt
```

**Result**
[ps plot](scripts/plots/ps.similarity_path.seq.pdf)






### Feature
Select nodes based on input settings. The output can be used as input for gfa2bin. 

```text
./gretl feature -g /path/to/graph.gfa -o /path/to/nodes.txt -D 10 
```

**Result**
- List of nodes which fulfill the input settings (plain-text, one node per line)

### Path
Select paths based on input settings. The output can be used as input for gfa2bin.

```text
./gretl feature -g /path/to/graph.gfa -o /path/to/nodes.txt -s "N/D ration" -m 10
```
**Result**
- List of paths/samples which fulfill the input settings (plain-text, one node per line)


### Bootstrap
Sample-based bootstrapping to calculate number of nodes and sequence for each number of possible samples. Start with a "complete" graph and remove random path for each run. Then recalculate the general statistics. And summarize the amount of sequence/nodes for each level (e.g. similarity).   
We recommend bootstrapping a graphs in PanSN-spec. Use ```--nodes``` if the bootstrap should only run on a subset of nodes.  
You are able to adjust the number of bootstrap, only calculate one "level" or input a meta file as input. Examples are shown in the data/example_data/ directory.  
Meta files can be used to use the same "combinations" for multiple graphs. This only works of the paths/samples of the graphs are in the same order. 


**Available options:**
- ```--nodes <nodes>```Run bootstrap only on these nodes
- ```--meta-input <meta input>``` Use a meta file as input. 
- ```--level <level>```Run bootstrap only for a specific level
- ```--number <number>``` Number of bootstrap for each number of genomes
- ```--meta-line <meta line>``` Run a boots trap of a specific line in the meta file.
- ```--meta <meta>``` Report the meta information in the output.


**Example**
```
./gretl bootstrap -g /path/to/graph.gfa -o /path/to/output.txt -n 20 
```

**Result**
- Using this [script](scripts/bootstrap.ipynb) to get [bootstrap plot](scripts/plots/bootstrap.pdf)

### (Sliding, path) window
Calculate statistics on a node level (graph- or path-based) and summarize them for each path in a sliding window approach. In detail: Iterate over the nodes of a path (window-like), summarize the stats of all nodes in the window and report a single value for each window. 

**Example**
````
./gretl window -g /path/to/graph.gfa -o /path/to/output.txt -s 1000 --step 100
````

**Result**
- Using this [script](scripts/window.ipynb) to get [window plot](scripts/plots/analysis.window.pdf)



### Nwindow
Summarizing the graph by a window of nodes. We iterate numerically over the nodes and calculate the statistics for each window. We start at the current node and move away from it based on provided edges, collecting the new nodes. We repeat this process starting at the "new" nodes until one of the following conditions is met:

- Jumps: A jumps is defined as difference between the current and the next node. Your input referees to the sum of all jumps in the window.
- Steps: A step it the number of moves we make in the graph. Your input is the maximum steps from the starting node. 
- Sequence: Limit the window by a sequence threshold. We stop if the sequence length is larger than the provided threshold.

**Example: How many nodes do I need to collect 1000 bp?**
```text
./gretl nwindow -g /path/to/graph.gfa -o /path/to/output.txt --sequence 1000 --node-number
```

**Output**: You are able to return the number of collected nodes, the total number of jumps or the total sequence. Some combinations of input limitation and output do not gain any additional information.

[nwindow plot](scripts/plots/nwindow.node.pdf)


### Find
Find a specific node (10), directed node (10+), or edge (10+20+) in the graph and get the exact (sequence) position in the paths. Output is a BED file with the positions. You are able to add additional sequence ```-l``` on both sites, which can help if you want to realign to a database and the node is very small. 
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

