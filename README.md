# gretl - Graph evaluation toolkit
---
```gretl``` is a tool for basic graph statistics using GFA format input. Our statistics are based on nodes, edges and paths/walks. Walks can also be used, but will be represented as paths internally. Many commands do not work without paths/walk information.  
In addition, we added some commands, using graph-based statistics, but represent it on a path (sample scale). We also offer commands for more complex analysis - an overview can be seen below. 

---
## Requirements on GFA file: 
- GFA format v1.0, v1.1 or v1.2.
- Numerical node ID

**Comment**:
- Sorted node IDs (in 1D SGD) are not required, but all *"jump"* related statistics and the *block* subcommand will be based on the order of the nodes in the GFA file. Check this [paper](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC10542513/) for more information. 
  - Run `odgi sort -Y` to sort the graph in pan-genomic order.
- We recommend dense node ID, starting at 1 and end at the number of nodes +1. Memory efficient on multiple levels.

### Prepare your graph (optional)
This is normally not needed if your graph is constructed by PGGB, Minigraph-Cactus or vg construct
```bash
# Convert string-based node IDs to numerical IDs
./target/release/gretl id2int -g /path/to/old_grapg.gfa -o /path/to/new_graph.gfa

# Sort the graph in pan-genomic order
odgi sort -Y -i /path/to/new_graph.gfa -o /path/to/new_graph_sorted.og
# Convert the ODGI file back to GFA
odgi view -i /path/to/new_graph_sorted.og -g > /path/to/new_graph_sorted.gfa
```
---
## Plotting your results / Follow-along documentation
We have added python scripts to visualize the output of the different commands of this tool. The scripts are stored in the `scripts` directory. Almost all commands of gretl are covered, some even multiple times. For better understanding we added a **[follow-along markdown](doc/gretl.examples.md)** with examples, which highlights research questions and ideas can be answered.  

---
## Explanation
Since some of the commands run relative complex computation, we have added a **detailed description** of each subcommand in the `doc` directory, which can be found **[here](doc/gretl.explained.md)**. It covers how internal each command is structured, what is calculated and how this calculation is done. 


---
## Installation: 

**Git**  
```
git clone https://github.com/MoinSebi/gretl  
cd gretl   
cargo build --release  
```

**Running gretl:**  
A list of all subcommands is at the bottom
```
./target/release/gretl [subcommand] [options]
```

---
## Testing
We provide a small test suite to test the basic functionality of the tool.
```
cargo test
```

---
## Usage

---
### Stats

Calculate statistics of a graph in GFA format. A list of all stats can be found [here](paper/stats_explained.md). Please consider using the ```--pansn``` option to group the paths by sample. Read more information about PanSN-spec [here](https://github.com/pangenome/PanSN-spec). 

In general the `stats` command has two possible options: 
1. Path-based statistics: Calculate statistics for each path in the graph (```-p``` flag needed)
2. Graph-based statistics: Calculate statistics for the whole graph, including hybrid statistics based on path statistics (default). These hybrid statistics are the average and standard deviation of all path statistics. A full list of all statistics be found in paper directory in this repository. 


**Example**
```Rust
/// Graph-based statistics
./gretl stats -g /path/to/graph.gfa -o /path/to/output.txt

/// Path-based statistics
./gretl stats -g /path/to/graph.gfa -o /path/to/output.path.txt -p
```

**Result**
- TSV or YAML file with statistics
- Check our [examples](doc/gretl.examples.md) for more ideas. 


---
### id2int
Convert any string-based node identifier to numeric values. Afterwards, use ```odgi sort -O``` to sort the graph in pan-genomic order, which will create more meaningful statistics in ```gretl stats``` (see above). Nevertheless, numerical node IDs a required by any ```gretl``` command. 

Available options:
- ```-d, --dict <dict>``` Write a dictionary with the new and old IDs to a plain text file. 

**Example**
```
./gretl id2int -g /path/to/graph.gfa -o /path/to/output.gfa -d /path/to/dict.txt
```

**Result**: 
- GFA file with numerical node IDs


**Important:** 
This function will convert **all** node IDs in the graph. **Additional data in tags will not be converted.** 

---
### Node-list
Individual node statistics based on graph-related features. Statistics provided: 
- Length
- Degree
- Depth
- Core

**Comment**
The information of the reported table can be used as a individual lookup or to create own window-like statistics (over nodes).

**Example**
```text
./gretl node-list -g /path/to/graph.gfa -o /path/to/output.txt
```

**Result**
- TSV file output 

| Node_id | Length | Core | Depth | ND |
|---------|--------|------|-------|----|
| 1       | 21176  | 1    | 1     | 0  |
| 2       | 15530  | 1    | 1     | 0  |
| 3       | 15530  | 1    | 1     | 0  |
| 4       | 24351  | 1    | 1     | 0  |
| 5       | 24367  | 1    | 1     | 0  |
| 6       | 100    | 2    | 2     | 2  |
| 7       | 1      | 1    | 1     | 1  |
| 8       | 1      | 1    | 1     | 1  |

---
### Core
Compute user-defined statistics of the graph (```-s```). Calculate the statistics for each node and summarize for each possible value the number of nodes and sequence. In an additional file ("```*.private.txt```") we report for each path the amount of nodes and sequence sole present by this sample. 

Available options:
- ```-s, --stats <statistics>```. Define the statistics you want to summarize (see above) [default: similarity].


```
./gretl core -g /path/to/graph.gfa -o /path/to/output.txt
```


**Result**
- [core plot](scripts/notebooks/plots/pancore.pdf)

### Path similarity (PS)

Calculate for each path the amount of nodes and sequence at each similarity level. 

```
./gretl ps -g /path/to/graph.gfa -o /path/to/output.txt
```

**Result**
[ps plot](scripts/notebooks/plots/ps.similarity_path.seq.pdf)

Example output: General path similarity

| Similarity | Sequence[bp] | #Node |
|------------|--------------|-------|
| 0          | 0            | 0     |
| 1          | 264241       | 7315  |
| 2          | 10804        | 2191  |
| 3          | 13800        | 2240  |
| 4          | 73893        | 6833  |
| 5          | 597805       | 7655  |

Private table: 

| Path       | Sequence[bp] | #Node |
|------------|--------------|-------|
| ABQ_6.ChrX | 47050        | 336   |
| BIH_4.ChrX | 26389        | 278   |
| ABF_6.ChrX | 33120        | 2181  |
| BPN_2.ChrX | 104353       | 1250  |
| BCK_8.ChrX | 53334        | 3275  |


---
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
- Using this [script](scripts/notebooks/bootstrap.ipynb) to get [bootstrap plot](scripts/notebooks/plots/bootstrap.pdf)

| Size | Run | Node:1 | Node:2 | Node:3 | Node:4 | Node:5 | Seq:1  | Seq:2  | Seq:3  | Seq:4  | Seq:5  |
|------|-----|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|
| 2    | 0   | 7651   | 13495  |        |        |        | 94383  | 680604 |        |        |        |
| 2    | 1   | 12238  | 10890  |        |        |        | 112920 | 666501 |        |        |        |
| 2    | 2   | 10184  | 11766  |        |        |        | 105283 | 665966 |        |        |        |
| 3    | 0   | 7773   | 7263   | 9587   |        |        | 122129 | 23996  | 662555 |        |        |
| 3    | 1   | 7710   | 7317   | 9657   |        |        | 140411 | 25086  | 663453 |        |        |
| 3    | 2   | 5255   | 5680   | 11466  |        |        | 131387 | 23065  | 664906 |        |        |
| 4    | 2   | 6756   | 2420   | 6487   | 9325   |        | 165241 | 7105   | 22037  | 661811 |        |
| 4    | 3   | 7870   | 3085   | 7085   | 7858   |        | 220983 | 19845  | 74507  | 598158 |        |
| 4    | 4   | 4988   | 2305   | 4912   | 10754  |        | 214961 | 9350   | 78758  | 604140 |        |
| 5    | 0   | 7315   | 2191   | 2240   | 6833   | 7655   | 264241 | 10804  | 13800  | 73893  | 597805 |

---
### Window (sliding, path)
Calculate statistics on a node level (graph- or path-based) and summarize them for each path in a sliding window approach. In detail: Iterate over the nodes of a path (window-like), summarize the stats of all nodes in the window and report a single value for each window. 

**Example**
````
./gretl window -g /path/to/graph.gfa -o /path/to/output.txt -s 1000 --step 100
````

**Result**
- Using this [script](scripts/notebooks/window.ipynb) to get [window plot](scripts/notebooks/plots/analysis.window.pdf)

Table: Path in col1, similarity values on all the other values (each column is 1000 bp, going 100 bp steps)

| ABQ_6.ChrX | 5 | 5   | 5 | 5 | 5 | 5   |
|------------|---|-----|---|---|---|-----|
| BIH_4.ChrX | 5 | 3.5 | 5 | 5 | 5 | 5   |
| ABF_6.ChrX | 5 | 5   | 5 | 5 | 5 | 5   |
| BPN_2.ChrX | 5 | 5   | 5 | 5 | 5 | 5   |
| BCK_8.ChrX | 5 | 5   | 5 | 5 | 5 | 4.5 |

---
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


Table: NodesID, Number of nodes, amount of sequence and sum of jumps (collected in a window)

| nodeid | node | sequence | jumps |
|--------|------|----------|-------|
| 240    | 54   | 50623    | 1321  |
| 241    | 53   | 50589    | 1296  |
| 242    | 46   | 1862     | 696   |
| 243    | 46   | 1862     | 709   |
| 244    | 44   | 1832     | 637   |
| 245    | 38   | 1762     | 458   |
| 246    | 38   | 1762     | 463   |
| 247    | 37   | 1567     | 410   |
| 248    | 33   | 575      | 280   |
| 249    | 33   | 575      | 280   |
| 250    | 33   | 391      | 256   |


---
### Block
Statistics of pangenome blocks. Blocks are defined to be a collection of nodes which are in proximity. Proximity is measured by the node id differences, therefore the order of the nodes in the GFA file is important. Blocks normally contain nodes of consecutive node IDs and can be defined by either number of nodes of each block OR total amount of sequence in each block. 

**Important: This is the only command which requires a sorted node ID space (1D SGD), which can be achieved by ```odgi sort -O```.** 

```text
./gretl block -g /path/to/graph.gfa -o /path/to/output.txt
```

**Result**   
Table: start and end node ID of the block, Number of nodes, amount of sequence and sum of jumps (collected in a window)

| start_end | #Traversal | #Path | #Nodes (sum)      | #Nodes (average)   | Sequence [bp]  (average) |
|-----------|------------|-------|-------------------|--------------------|--------------------------|
| 1-101     | 5          | 5     | 5                 | 25.6               | 12201.2                  |
| 101-201 2 | 2          | 2     | 66.5              | 1748.5             | 1748.5                   |
| 201-301 4 | 4          | 4     | 57.75             | 884.0              | 884.0                    |
| 301-401 4 | 4          | 4     | 65.25             | 472.75             | 472.75                   |
| 401-501 4 | 4          | 4     | 66.0              | 418.0              | 418.0                    |
| 501-601 4 | 4          | 4     | 52.25             | 1823.25            | 1823.25                  |
| 601-701 3 | 3          | 3     | 69.33333333333333 | 1646.3333333333333 | 1646.3333333333333       |



---
### Find
Find a specific node (e.g. 10), directed node (e.g. 10+), or edge (e.g. 10+20+) in the graph and get the exact (sequence) position in the paths. Output is a BED file with the positions. You are able to add additional sequence (using flag ```-l```) on both sites, which can help if you want to realign to a database and external sequence.  
```text
./gretl find -g /path/to/graph.gfa -o /path/to/output.txt --length 1000 -f feature.txt 
```

---
### Feature
Return a list of nodes which based on input settings. The output can be used as input for gfa2bin.

```text
./gretl feature -g /path/to/graph.gfa -o /path/to/nodes.txt -D 10 
```

**Result**
- List of nodes which fulfill the input settings (plain-text, one node per line)

---
### Path
Select paths based on input settings. The output can be used as input for gfa2bin.

```text
./gretl feature -g /path/to/graph.gfa -o /path/to/nodes.txt -s "N/D ration" -m 10
```
**Result**
- List of paths/samples which fulfill the input settings (plain-text, one node per line)

## Citation 
If you use gretl in your work, please cite:  
Gretl - Variation GRaph Evaluation TooLkit
Sebastian Vorbrugg, Ilja Bezrukov, Zhigui Bao, Detlef Weigel
doi: [https://doi.org/10.1101/2024.03.04.580974](https://doi.org/10.1101/2024.03.04.580974)




