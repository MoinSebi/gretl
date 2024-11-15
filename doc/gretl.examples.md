# Collected workflows

We have added a collection of python scripts stored in the `scripts` directory. When run correctly, these scripts can be used to generate a variety of plots from the output of the gretl toolkit. Since many scripts return multiple plots, we have included a prefix option to allow you to specify the output file name. In the following, we provide a list of workflows that can be used to answer specific questions about your graph(s). These can be used as a "follow-along" guide to the gretl tool.

The python scripts need the following requirements (in brackets are the versions we used):
- python3 (3.10)
- matplotlib (3.1.2)
- pandas (1.2.3)
- seaborn (0.11.0)
- numpy (1.21.6)
- scipy (1.4.1)

We have run all these commands with a small example graph consisting of 50 Ecoli genomes. You can download the graph from here:  https://zenodo.org/record/7937947/files/ecoli50.gfa.zst. 

## Single graph workflows 

### 1. How does my graph saturate?
Alternative questions: 
- Is there a tendency that the genomes of my graph saturate with the number of samples?
- How many genomes do I need to sample to capture the full diversity of my organism? (This needs to be identified by eye)


**Gretl subcommand: bootstrap**  
**Plotting script: [saturation_plotter.py](../scripts/saturation_plotter.py)**   
**Example: [ExampleS1](../plots/test.bootstrap.seq.pdf) (s. Fig S1)** 

```bash
INPUT="path/to/your/graph.gfa"
OUTPUT="path/to/your/output.prefix"
target/release/gretl bootstrap -g $INPUT -o -  |   python3 scripts/saturation_plotter.py -i - -o $OUTPUT
```

### 2. Are there some nodes that are more connected than others?
Alternative questions:
- Which node is the most connected in my graph? (Identify this by the tabular output provided by the gretl toolkit)
- Are there regions which show exceptional high connectivity?

**Gretl subcommand: nwindow**  
**Plotting script: [nwindow.py](../scripts/nwindow.py)**  
**Example: [ExampleS2](../plots/test.nwindow.jumps.no_correction.pdf) (s. Figure S8)**

```bash
INPUT="path/to/your/graph.gfa"
OUTPUT="path/to/your/output.prefix"
target/release/gretl nwindow -g $INPUT -o -  |   python3 scripts/nwindow.py -g $INPUT -i - -o $OUTPUT
```



### 3. On a genomic (linear) scale, which regions are similar to each other and which are not?
Alternative questions:
- Which regions of my graph show high local similarity?


**Gretl subcommand: window**  
**Plotting script: [window.py](../scripts/window.py)**   
**Example: [ExampleS3](../plots/test.window.pdf) (s. Figure 1C (left))**


```bash
INPUT="path/to/your/graph.gfa"
OUTPUT="path/to/your/output.prefix"
target/release/gretl window -g $INPUT -o -  |   python3 scripts/window.py -i - -o $OUTPUT
```

### 4. How much core, shell and private sequence do I have?
Alternative questions:
- How much of my graph is core, shell and private (in nodes)?
- If running this for different graphs, which graph exhibit the most core sequence? 

**Gretl subcommand: core**  
**Plotting script: [core.py](../scripts/core.py)**   
**Example: [ExampleS4](../plots/test.core.node.pdf) (s. Figure 1C (right))**



```bash
INPUT="path/to/your/graph.gfa"
OUTPUT="path/to/your/output.prefix"
target/release/gretl core -g $INPUT -o -  |   python3 scripts/core.py -i - -o $OUTPUT
```

### 5. Are there specific regions in my graph that are traversed by many paths?
Alternative questions:
- Is there a region which is traversed by one path many more times than by all the other paths? (e.g. private CNVs)

**Gretl subcommand: block**  
**Plotting script: [block.py](../scripts/block.py)**   
**Example: [ExampleS5](../plots/test.block.%23Nodes(average).pdf)**

```bash
INPUT="path/to/your/graph.gfa"
OUTPUT="path/to/your/output.prefix"
target/release/gretl block -g $INPUT -o -  |   python3 scripts/block.py -i - -o $OUTPUT
```


### 6. I have selected one graph in my data set, now I am interested in the paths of it. 
Alternative questions:
- In direct comparison, which paths share most of their statistics? (Alternative: Or are most closely related?)
- Is there a path that is unique in its statistical profile in comparison to the other paths?

**Gretl subcommand: path**  
**Plotting script: [path.py](../scripts/path.py)**   
**Example: [ExampleS6](../plots/test.path.heatmap.pdf)**


```bash
INPUT="path/to/your/graph.gfa"
OUTPUT="path/to/your/output.prefix"
target/release/gretl stats -p -g $INPUT -o -  |   python3 scripts/path.py -i - -o $OUTPUT
```

### 7. I am interested in two statistics from my graph path statistics (`graph stats -p`). How do I directly compare them?  
Alternative questions:
- In direct comparison, which paths share most of their statistics? (Or: Are most closely related?)

**Gretl subcommand: path**  
**Plotting script: [stats_path.py](../scripts/stats_path.py)**   
**Example: [ExampleS7](../plots/test.path.scatter.pdf) (s. Figure 4)**


```bash
INPUT="path/to/your/graph.gfa"
OUTPUT="path/to/your/output.prefix"
target/release/gretl stats -p -g $INPUT -o -  |   python3 scripts/stats_path.py -i - -o $OUTPUT -x "Covered sequence [%]" -y "Edges"
```
Comment: Replace **-x "Covered sequence [%]"** and **-y "Edges"** with the two statistics you want to plot.

### 8. I have selected a graph in my data set, how much core, shell and private sequence does each path contain?
Alternative questions:
- Which path contains the most core sequence?

**Gretl subcommand: path**  
**Plotting script: [ps.py](../scripts/ps.py)**   
**Example: [ExampleS8](../plots/test.ps.node.pdf) (s. Figure S6)**

```bash
INPUT="path/to/your/graph.gfa"
OUTPUT="path/to/your/output.prefix"
target/release/gretl ps -g $INPUT -o -  |   python3 scripts/ps.py -i - -o $OUTPUT
```

### 9. I want to look into details of my graph, give me a list of all nodes and their statistics

**Gretl subcommand: node-list**  

```bash
INPUT="path/to/your/graph.gfa"
OUTPUT="path/to/your/output.prefix"
target/release/gretl node-list -g $INPUT -o node.list.txt
```


## Multi-graph workflows - Comparing graphs and find the once that are most interesting
First, create a full set of statistics for all graphs. If you want to run this bash snippet, you need to provide a list of paths to graph files in the input text file. 


```bash
INPUT="path/to/your/graph.list.txt"
OUTPUT="path/to/your/output.prefix"

# Create a list of statistics for all graphs
for file in $(cat $INPUT); do
  basename=$(basename $file)
  target/release/gretl stats -g $file -o $OUTPUT.$basename.stats
done

# Create a file with the graph names in the first column and the paths to the stats files in the 2nd
# This is going to be used in the following snippets to create the plots
echo -n "" > $OUTPUT.stats.list
for file in $(cat $INPUT); do
  basename=$(basename $file)
  echo -n -e $basename"\t"  >> $OUTPUT.stats.list
  realpath $OUTPUT.$basename.stats >> $OUTPUT.stats.list
done
```


### 1. How big are the differences of the graphs on a statistical level? 
Alternative questions/ideas:
- Show me the distribution of the differences between the graphs. 

**Plotting script: [multi.histogram.py](../scripts/multi.histogram.py)**   
**Example: [ExampleM1](../plots/multi.histogram.Compression.histo.pdf)**

The followig snippet creates a histogram for each statistic in the provided list. This results in over 100 plots in the OUTPUT directory.
```bash
INPUT="path/to/your/stats.list"
OUTPUT="path/to/your/output.multi.histogram.prefix"
python3 scripts/multi.histogram.py -i $INPUT -o $OUTPUT
```


### 2. Can you show me comparison of all statistics of the graphs?
Alternative questions/ideas:
- Which graph is the most different to all the other graphs? (not detailed, just by eye)?

**Plotting script: [multi.heatmap.py](../scripts/multi.heatmap.py)**   
**Example: [ExampleM2](../plots/test.multi.heatmap.pdf) (s. Figure 1B (left))**



```bash
INPUT="path/to/your/stats.list"
OUTPUT="path/to/your/output.multi.heatmap.prefix"
python3 scripts/multi.heatmap.py -i $INPUT -o $OUTPUT
```


### 3. Which features span the most interesting range of values? (Scatter plot)
Alterative questions/ideas:
- Which features are most correlated with each other?

**Plotting script: [multi.auto.py](../scripts/multi.auto.py)**   
**Example: [ExampleM3](../plots/multi.auto.scatter.PathNodesizestd%5Bbp%5D(std)--Edges.scatter.pdf) (s. Figure 1B (right), Figure S3)**

If you run this script, the relative deviation will be calculated for each statistic. Those with the highest deviation will be selected and all pairs of those will be plotted in a scatter plot (always 2). The default amount of selected features is 10, therefore 45 scatter plots will be created. Those can be found in the OUTPUT directory.
```bash
INPUT="path/to/your/stats.list"
OUTPUT="path/to/your/output.multi.auto.prefix"
python3 scripts/multi.auto.py -i $INPUT -o $OUTPUT
```

### 4. Show me scatter plot of these two features (name feature here)? 

**Plotting script: [multi.scatter.py](../scripts/multi.scatter.py)**   
**Example:  [ExampleM4](../plots/test.multi.scatter.Nodes.Nodelength(average)%5Bbp%5D.scatter.pdf) (s. Figure 1B (right), Figure S3)**


```bash
INPUT="path/to/your/stats.list"
OUTPUT="path/to/your/output.multi.scatter.prefix"
python3 scripts/multi.scatter.py -i $INPUT -o $OUTPUT -x "Nodes" -y "Node length (average) [bp]"
```


### 5. Which statistics do (anti-)correlate the most)?
Alternative questions/ideas:
- Which statistics are the most unique in my dataset? 

**Plotting script: [multi.correlate.py](../scripts/multi.correlate.py)**   
**Example: [ExampleM5](../plots/test.multi.corr.heatmap.pdf) (s. Figure S5 (top))**


```bash
INPUT="path/to/your/stats.list"
OUTPUT="path/to/your/output.multi.correlation.prefix"
python3 scripts/multi.correlate.py -i $INPUT -o $OUTPUT
```




