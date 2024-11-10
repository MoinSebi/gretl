# Subcommands explained

Give a brief explanation of each subcommand here. 

### Similarity: How many path traverse this node? 
### Depth: How many path traverse this node? 
### Jumps: Node id difference between the two checked nodes. 
### Inverted nodes: Edges which go from forward to reverse or reverse to forward.

## Core 
Calculate the similarity and/or depth of a graph. Summarize the amount of nodes or sequence of each similarity/depth category. 
Example question: How many nodes are traversed more than 10 times in the graph?


## Window
Calculate the similarity and/or depth of a graph. For each path in the graph, iterate over the nodes (in the order of the path) and calculate the average similarity/depth of the nodes in a window of size w. Best representation is a heatmap with the x-axis being the path position, y-axis being the path name and the color being the similarity/depth.

## Nwindow
For each node in the graph move X steps away till an end criteria is reached (see below). This is done step by step, in all directions. When the end criteria is reached, summarize the amount of sequence, nodes and jumpps within this created subgraph (with the starting node in the middle).

## Block 
Requirements: Sorted pangenome graph.
Step 1: Define a block as a set of nodes which seem to be in pangenomic proximity (known by the ID difference between the nodes).
Step 2: For each block, find path, which hold one or more nodes of the defined block node IDs. 
Step 3: Now each block has a set of paths. For each block, summarize the amount of sequence, nodes and jumps and more.

## Find 

## id2int
Creates a **new** graph file with the node IDs replaced by integers. IT DOES NOT SORT THE GRAPH.
Internally, the graph is read once, storing all nodes together and their "number" in a hashmap (memory-efficient). Then the graphs is read again, converting all "old" node IDs to the new integer IDs.


## node-list
This is more or len

## ps 

## Stats
Calculate the several statistics of the graph. It can be run in two "modes": 
1. Path mode: Calculate the statistics of each path in the graph.
2. Whole graph mode: Calculate the statistics of the whole graph, which includes
   1. Graph statistics (e.g. number of nodes, number of edges, number of paths, etc.)
   2. Hybrid statistics (e.g. average number of nodes in a path, average number of edges in a path, etc.)
A list of all statistics can be found in when using "names". 
