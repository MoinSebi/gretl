# Subcommands explained

Brief explanation of each subcommand. 

## Gretl terminology (more to come)
- Similarity: Amount of different path traversing this node.
- Depth: Total amount of traversals. Multiple traversals of the same path are counted.
- Jumps: Difference between two nodes in the graph (e.g. connected by an edge). 
- Inverted nodes: Edges which go from forward to reverse or reverse to forward.

## Core 
Calculate the similarity and/or depth of a graph. Summarize the amount of nodes or sequence of each similarity/depth category. 

## Window
Calculate the similarity and/or depth of a graph. For each path in the graph, iterate over the nodes (in the order of the path) and calculate the average similarity/depth of the nodes in a window of size w. Best representation is a heatmap with the x-axis being the path position, y-axis being the path name and the color being the similarity/depth.

## Nwindow
For each node in the graph move X steps away till an end criteria is reached (see below). This is done step by step, in all directions. When the end criteria is reached, summarize the amount of sequence, nodes and jumpps within this created subgraph (with the starting node in the middle).

## Block 
Requirements: Sorted pangenome graph.
Step 1: Define a block as a set of nodes which seem to be in pangenomic proximity (known by the ID difference between the nodes).
Step 2: For each block, find path, which hold one or more nodes of the defined block node IDs. 
Step 3: Now each block has a set of paths. For each block, summarize the amount of sequence, nodes and jumps and more.


## id2int
Creates a **new** graph file with the node IDs replaced by integers. IT DOES NOT SORT THE GRAPH.
Internally, the graph is read once, storing all nodes together and their "number" in a hashmap (memory-efficient). Then the graphs is read again, converting all "old" node IDs to the new integer IDs.


## Stats
Calculate the several statistics of the graph. It can be run in two "modes": 
1. Path mode: Calculate the statistics of each path in the graph.
2. Whole graph mode: Calculate the statistics of the whole graph, which includes
   1. Graph statistics (e.g. number of nodes, number of edges, number of paths, etc.)
   2. Hybrid statistics (e.g. average number of nodes in a path, average number of edges in a path, etc.)

## Bootstrap
Bootstrapping different combinations of samples from the graph. These sub-sets create a smaller graph, which can be interpreted as a "sample" of the original graph. We sample from one sample to n samples, where n is the number of samples in the graph. Multiple times for each level (n), multiple different (random) samples are selected. We calculate similarity statistics on these sub-graphs, and note the amount of nodes and sequence in each similarity level.

## PS (Path similarity)
We count the number of path traversing each node. Then, this method iterates over each path, and summarizes the amount of sequence and nodes in each similarity category.