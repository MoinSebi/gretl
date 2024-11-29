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
Since some of the commands run relative complex computation, we have added a **detailed description** of each subcommand in the `doc` directory, which can be found **[here](doc/gretl.explained.md)**. It covers how internal each command is structured, what is calculated and how this calculation is performed (very roughly).


---
## Installation: 

### Github
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
### From bioconda channel
You need to install [conda](https://docs.conda.io/en/latest/)/[mamba](https://anaconda.org/conda-forge/mamba) first. We recommend creating a new environment for gretl to avoid conflicts with other packages `conda create -n gretl_env`.
```
mambda install -c conda-forge -c bioconda gretl
``` 


---
## Testing
We provide a small test suite to test the basic functionality of the tool.
```
cargo test
```
---
## Performance
We tried to optimize the code as much as possible. Most steps are dependent on the number of nodes and the length of the paths. Runtime should be linear in most cases. All commands can be multithreaded, which can be controlled by the ```-t``` option. 


---
## Usage

---
### Stats

Calculate statistics of a graph in GFA format. A list of all stats can be found [here](paper/stats_explained.md). Please consider using the ```--pansn``` option to group the paths by sample. Read more information about PanSN-spec [here](https://github.com/pangenome/PanSN-spec). 

In general the `stats` command has two possible options: 
1. Path-based statistics: Calculate statistics for each path in the graph (```-p``` flag needed)
2. Graph-based statistics: Calculate statistics for the whole graph, including hybrid statistics based on path statistics (default). These hybrid statistics are the average and standard deviation of all path statistics. A full list of all statistics be found in paper directory in this repository. 


**Example**
```bash
/// Graph-based + hybrid statistics
./gretl stats -g /path/to/graph.gfa -o /path/to/output.txt

/// Path-based statistics
./gretl stats -g /path/to/graph.gfa -o /path/to/output.path.txt -p
```

**Result**
- TSV or YAML file with statistics

---
### id2int
Convert any string-based node identifier to numeric values. Afterward, use ```odgi sort -Y``` to sort the graph in pan-genomic order, which will create more meaningful statistics in ```gretl stats``` and is required in ````block´´´ subcommand (see above). Nevertheless, numerical node IDs are required by any ```gretl``` command. The new node identifiers will start at 1 and end at the number of nodes +1. More information can be found [here](./doc/gretl.explained.md). 

Available options:
- ```-d, --dict <dict>``` Write a dictionary with the new and old IDs to a plain text file. 

**Example usage**
```bash
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
- Node Degree
- Depth
- Core
 
**Comment**  
The information of the reported table can be used as a individual lookup or to create own window-like statistics (over nodes).

**Example usage**
```bash
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
Count the amount of sequence and the number of nodes found in each similarity level. The method calculates the similarity for each node and summarize for each possible level the number of nodes and the amount sequence. 


**Example usage**
```
./gretl core -g /path/to/graph.gfa -o /path/to/output.txt
```

---
### Path similarity (PS)

Calculate for each path the number of nodes and the amount of sequence at each similarity level. 

**Example usage**
```
./gretl ps -g /path/to/graph.gfa -o /path/to/output.txt
```

**Example output**
Core sequence/nodes are those which are traversed by all other samples as well (here: 5). Since nodes can be traversed multiple times, the amount of sequence and the number of nodes can be different in each path.  
*Type: N = Number of nodes, S = Amount of sequence [bp]*

| Accession  | Type | 0 | 1      | 2    | 3     | 4     | 5      |
|------------|------|---|--------|------|-------|-------|--------|
| ABQ_6.ChrX | N    | 0 | 336    | 891  | 1785  | 6630  | 7655   |
| BIH_4.ChrX | N    | 0 | 278    | 764  | 1753  | 6592  | 7655   |
| ABF_6.ChrX | N    | 0 | 2181   | 1093 | 1206  | 5216  | 7655   |
| BPN_2.ChrX | N    | 0 | 1250   | 690  | 916   | 5170  | 7672   |
| BCK_8.ChrX | N    | 0 | 3275   | 948  | 1062  | 3734  | 7655   |
| ABQ_6.ChrX | S    | 0 | 47050  | 3792 | 12833 | 73540 | 597805 |
| BIH_4.ChrX | S    | 0 | 26389  | 2445 | 12195 | 73030 | 597805 |
| ABF_6.ChrX | S    | 0 | 33120  | 5969 | 12142 | 71593 | 597805 |
| BPN_2.ChrX | S    | 0 | 104353 | 5352 | 1650  | 9894  | 598957 |
| BCK_8.ChrX | S    | 0 | 53334  | 4054 | 2600  | 67558 | 597805 |
---
### Bootstrap
We bootstrap different samples of the graph. We iterate from one sample to the maximum number of samples, each time bootstrapping n different combinations. At each iteration, we calculate the number of nodes and the amount of sequence in each similarity level. If you want to exclude speicifc nodes, you can use the ```--nodes``` option. The most important parameter is the number of bootstraps, which is defined by the ```--number``` option. 


**Available options:**
- ```--nodes <nodes>```Run bootstrap only on these nodes
- ```--number <number>``` Number of bootstrap for each number of genomes

**Example usage**
```
./gretl bootstrap -g /path/to/graph.gfa -o /path/to/output.txt -n 20 
```

**Example output**
- Size: Number of samples bootstrapped
- Run: Which iteration of this bootstrap-level
- Type: N = Number of nodes, S = Amount of sequence
- 1-5: Number of nodes or amount of sequence in each similarity level (depends on the type)

| Size | Run | Type | 1     | 2     | 3     | 4    | 5    |
|------|-----|------|-------|-------|-------|------|------|
| 2    | 0   | N    | 12238 | 10890 | 0     | 0    | 0    |
| 2    | 1   | N    | 7650  | 13368 | 0     | 0    | 0    |
| 2    | 2   | N    | 10165 | 11903 | 0     | 0    | 0    |
| 3    | 0   | N    | 7773  | 7263  | 9587  | 0    | 0    |
| 3    | 1   | N    | 8621  | 7833  | 8466  | 0    | 0    |
| 3    | 2   | N    | 4370  | 4041  | 13077 | 0    | 0    |
| 4    | 0   | N    | 6756  | 2420  | 6487  | 9325 | 0    |
| 4    | 1   | N    | 6227  | 2303  | 6249  | 9275 | 0    |
| 4    | 2   | N    | 7870  | 3085  | 7085  | 7858 | 0    |
| 5    | 0   | N    | 7315  | 2191  | 2240  | 6833 | 7655 |

---
### Window (sliding, path)
Calculate statistics on a node level (graph- or path-based) and summarize them for each path in a sliding window approach. In detail: Iterate over the nodes of a path (window-like), summarize the stats of all nodes in the window and report a single value for each window. 

**Example Usage**
````
./gretl window -g /path/to/graph.gfa -o /path/to/output.txt -s 1000 --step 100
````


**Example output**
Table: Path in col1, similarity values on all the other values (each column is 1000 bp, going 100 bp steps)

| ABQ_6.ChrX | 5 | 5   | 5 | 5 | 5 | 5   |
|------------|---|-----|---|---|---|-----|
| BIH_4.ChrX | 5 | 3.5 | 5 | 5 | 5 | 5   |
| ABF_6.ChrX | 5 | 5   | 5 | 5 | 5 | 5   |
| BPN_2.ChrX | 5 | 5   | 5 | 5 | 5 | 5   |
| BCK_8.ChrX | 5 | 5   | 5 | 5 | 5 | 4.5 |

---
### Nwindow
For each node we calculate how many other nodes can be reached by a certain number of steps, sequence or jumps. The output is a table with the node ID, the number of nodes, the amount of sequence and the sum of jumps.

**Step criteria**
- Jumps: A jumps is defined as difference between the current and the next node. Your input referees to the sum of all jumps in the window.
- Steps: A step it the number of moves we make in the graph. Your input is the maximum steps from the starting node. 
- Sequence: Limit the window by a sequence threshold. We stop if the sequence length is larger than the provided threshold.

**Example usage: How many nodes do I need to collect 1000 bp?**
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

**Important: This is the only command which requires a sorted node ID space (1D SGD), which can be achieved by ```odgi sort -Y```.** 

**Example usage**
```text
./gretl block -g /path/to/graph.gfa -o /path/to/output.txt
```

**Example result**
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

**Example input**
```text
10
11
12
```

**Example output** 
BED format. Tags: ID: Node ID, NS: Node start, NB: Node end
```text
BPN_2.ChrX	0	21377	ID:1+;NS:21176;NB:0
BCK_8.ChrX	0	15731	ID:3+;NS:15530;NB:0
```

## Citation 
If you use gretl in your work, please cite:  
Gretl - Variation GRaph Evaluation TooLkit
Sebastian Vorbrugg, Ilja Bezrukov, Zhigui Bao, Detlef Weigel
doi: [https://doi.org/10.1101/2024.03.04.580974](https://doi.org/10.1101/2024.03.04.580974)




