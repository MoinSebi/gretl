# Graph stats 

| Paths                    | Number of Paths                                                                                                                                                                     |
|--------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Samples                  | Number of samples (after PanSN-spec)                                                                                                                                                |
| Nodes                    | Number of nodes in the graph                                                                                                                                                        |
| Edges                    | Number of edges                                                                                                                                                                     |
| N/E ration               | Ratio of number of nodes to number of edges                                                                                                                                         |
| Graph size [bp]          | The total sequence of all nodes in the graph                                                                                                                                        |
| Input genomes size [bp]  | Total sequence of all paths in the graph                                                                                                                                            |
| Compress                 | (Input genome size [bp])/(Graph size [bp])                                                                                                                                          |
| Node length              | The average or median of the node size in bp                                                                                                                                        |
| Node length top 5 %      | The average or median size of the top 5 % node (sorted by size)                                                                                                                     |
| Bin[x-y]                 | Number of nodes from size x to size y                                                                                                                                               |
| Similarity               | Count the number of samples/paths which travers this node. Compute is for each node and calculate mean/median and standard deviation                                                |
| Depth                    | Count the number oif samples/path which travers this node (loops/multiple runs by the same paths allows). Compute is for each node and calculate mean/median and standard deviation |
| Node degree              | Number or in (in) and outcoming (out) edges. Sum both up to get total amount                                                                                                        |
| Inverted edges           | Number of edges which change direction + -> - or + -> +                                                                                                                             |
| Negative edges           | Number of edges which are - -> -                                                                                                                                                    |
| Self edges               | Number of edges which start and end at the same node                                                                                                                                |
| Graph density            |                                                                                                                                                                                     |


Path stats:

| Name                                           | Defintion                                                                                                                        |
|------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| Path                                           | Path name                                                                                                                        |
| Sequence [bp]                                  | Total sequence in this path                                                                                                      |
| Nodes                                          | Number of nodes                                                                                                                  |
| Unique edges                                   | Number of unique edges                                                                                                           |
| Directed edges                                 | Number of directed edges (1+, 2+)                                                                                                |
| Edges                                          | Number of edges (total)                                                                                                          |
| Unique edges                                   | Number of unique edghes                                                                                                          |
| Unique nodes                                   | Number of unique nodes                                                                                                           |
| Unique nodes [bp]                              | Total size of these nodes                                                                                                        |
| Unique nodes (normalizes)                      | Unique nodes / Number of nodes                                                                                                   |
| Unique nodes [bp] (normalizes)                 | Unique nodes [bp] / Total sequence                                                                                               |
| Unique edges (normalized)                      | Number of unique edges / Number of edges                                                                                         |
| Inverted nodes                                 | Number of inverted nodes                                                                                                         |
| Inverted nodes [bp]                            | Total sequence of all inverted nodes                                                                                             |
| Unverted nodes (normalized)                    | Number of inverted nodes / Total number of nodes                                                                                 |
| Inverted nodes [bp] (normalized)               | Total sequence of all inverted nodes / Total size                                                                                |
| Jumps total                                    | Total number of jumps: A jump is the difference between end and starting node of a edge. Them summarize all differences together |
| Jumps total (normalized)                       | Jumps total / Number of edges                                                                                                    |
| Jumps bigger than X                            | Number of edges/jumps which have a bigger difference than X                                                                      |
| Jumps bigger than X (normalized)               | Jumps bigger than X / Number of edges                                                                                            |
| Node size (average, median, std)               | Average, median and standard deviation of the total amount of nodes sizes in this path                                           |
| Depth (average, median, std)                   | Average, median and standard deviation of the depth computed for each node in the path                                           |
| Depth (average, median, std) (normalized)      | Normalize the above statistics by the number of nodes                                                                            |
| Similarity (average, median, std)              | Average, median and standard deviation of the similarity computed for each node in the path (definition of similarity see above) |
| Similarity (average, median, std) (normalized) | Normalize the above statistics by the number of nodes                                                                            |
| Degree (average, median, std)                  | Average, median and standard deviation of the degree computed for each node in the path                                          |



