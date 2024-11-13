# Collected workflows

We have added a collection of python scripts stored in the `scripts` directory. When run correctly, these scripts can be used to generate a variety of plots from the output of the gretl toolkit. Since many scripts return multiple plots, we have included a prefix option to allow you to specify the output file name. In the following, we provide a list of workflows that can be used to answer specific questions about your graph(s). These can be used as a "follow-along" guide to the gretl tool.

The python scripts need the following requirements (in brackets are the versions we used):
- python3 (3.10)
- matplotlib (3.1.2)
- pandas (1.2.3)
- seaborn (0.11.0)
- numpy (1.21.6)


## Single graph workflows 

### 1. How does my graph saturate?
Alternative questions: 
- Is there a tendency that the genomes of my graph saturate with the number of samples?
- How many genomes do I need to sample to capture the full diversity of my organism? (This needs to be identified by visualisation)


**Subcommand: bootstrap**  
**Plotting script: [saturation_plotter.py](../scripts/saturation_plotter.py)**   
**Example: Figure 1** 


** Run bootstrap + plot**
```bash
INPUT= "path/to/your/graph.gfa".gfa
OUTPUT= "path/to/your/output.prefix"
target/release/gretl bootstrap -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/saturation_plotter.py -i - -o $OUTPUT
```

### 2. Are there some nodes that are more connected than others?
Alternative questions:
- Which node is the most connected in my graph? (Identify this by the tabular output provided by the gretl toolkit)
- Are there regions which show exceptional high connectivity?

**Subcommand: nwindow**  
**Plotting script: [nwindow.py](../scripts/nwindow.py)**  
**Example: Figure 1**

```bash
INPUT= "path/to/your/graph.gfa"
OUTPUT= "path/to/your/output.prefix"
target/release/gretl nwindow -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/nwindow.py -i - -o $OUTPUT
```



### 3. On a genomic (linear) scale, which regions are similar to each other and which are not?
Alternative questions:
- Which regions of my graph show high local similarity?


**Subcommand: window**  
**Plotting script: [window.py](../scripts/window.py)**   
**Example: Figure 1**


```bash
INPUT= "path/to/your/graph.gfa"
OUTPUT= "path/to/your/output.prefix"
target/release/gretl bootstrap -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/saturation_plotter.py -i - -o $OUTPUT
```

### 4. How much core, shell and private sequence do I have?
Alternative questions:
- How much of my graph is core, shell and private (in nodes)?
- If running this for different graphs? Which graph exhibit the most core sequence? 

**Subcommand: core**  
**Plotting script: [core.py](../scripts/core.py)**   
**Example: Figure 1**



```bash
INPUT= "path/to/your/graph.gfa"
OUTPUT= "path/to/your/output.prefix"
target/release/gretl bootstrap -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/core.py -i - -o $OUTPUT
```

### 5. My graph seems to be highly connected, create pangenome blocks and return statistics for each block? 
Alternative questions:
- Which regions of my graph is traversed by many paths? 
- Is there a region which is traversed by one path many more times than all the other paths?

**Subcommand: block**  
**Plotting script: [block.py](../scripts/block.py)**   
**Example: Figure 1**

```bash
INPUT= "path/to/your/graph.gfa"
OUTPUT= "path/to/your/output.prefix"
target/release/gretl block -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/block.py -i - -o $OUTPUT
```


### 6. I have selected one graph in my data set, now I am interested in the paths of it
Alternative questions:
- In direct comparison, which paths share most of their statistics? (Or: Are closest related?)
- Is there any path that is unique to all the other paths?

**Subcommand: path**  
**Plotting script: [stats_path.py](../scripts/stats_path.py)**   
**Example: Figure 1**


```bash
INPUT= "path/to/your/graph.gfa"
OUTPUT= "path/to/your/output.prefix"
target/release/gretl path -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/path.py -i - -o $OUTPUT
```

### 7. I have selected a graph in my data set, how much does each path contain of core, shell and private sequence?
Alternative questions:
- Which path contains the most core sequence?

**Subcommand: path**  
**Plotting script: [path.py](../scripts/path.py)**   
**Example: Figure 1**

```bash
INPUT= "path/to/your/graph.gfa"
OUTPUT= "path/to/your/output.prefix"
target/release/gretl path -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/path.py -i - -o $OUTPUT
```

### 8. I want to look into details of my graph, give me a list of all nodes and their statistics
Alternative questions:
- I want to write a script with some of the statistics of my graph? 

**Subcommand: node-list**  

```bash
INPUT= "path/to/your/graph.gfa"
OUTPUT= "path/to/your/output.prefix"
target/release/gretl node-list -g data/example_data/chr5.yeast.gfa -o node.list.txt
```


## Multi-graph workflows - Comparing graphs and find the once that are most interesting
First, create a full set of statistics for all graphs


```bash
INPUT="path/to/your/graph.list.txt"
OUTPUT="path/to/your/output.prefix

for file in $(cat $INPUT); do
    target/release/gretl stats -g $file -o $OUTPUT. 
done

realpath $OUTPUT.*.stats > $OUTPUT.stats.list
```


### 1. How big are the differences of the graphs on a statistical level? 
Alternative questions/ideas:
- Show me the distribution of the differences between the graphs. 

**Plotting script: [multi.histogram.py](../scripts/multi.histogram.py)**   
**Example: Figure 1**

```bash
INPUT= "path/to/your/stats.list.gfa"
OUTPUT= "path/to/your/output.multi.prefix"
python3 scripts/scripts/multi.histogram.py -i $INPUT -o $OUTPUT
```


### 2. Can you show me comparison of all statistics of the graphs?
Alternative questions/ideas:
- Which graph is the most different to all the other graphs? (not detailed, just by eye)?

**Plotting script: [multi.heatmap.py](../scripts/multi.heatmap.py)**   
**Example: Figure 1**



```bash
INPUT= "path/to/your/stats.list.gfa"
OUTPUT= "path/to/your/output.multi.prefix"
python3 scripts/scripts/multi.heatmap.py -i - -o $OUTPUT
```


### 3. Which features span the most interesting range of values? (Scatter plot)
Alterative questions/ideas:
- Which features are most correlated with each other?

**Plotting script: [multi.auto.py](../scripts/multi.auto.py)**   
**Example: Figure 1**

```bash
INPUT= "path/to/your/stats.list.gfa"
OUTPUT= "path/to/your/output.multi.prefix"
python3 scripts/scripts/multi.auto.scatter.py -i - -o $OUTPUT
```

### 4. Show me scatter plot of these two features (name feature here)? (Scatter plot)
Alternative questions/ideas:

**Plotting script: [multi.scatter.py](../scripts/multi.scatter.py)**   
**Example: Figure 1**


```bash
INPUT= "path/to/your/stats.list.gfa"
OUTPUT= "path/to/your/output.multi.prefix"
python3 scripts/scripts/multi.span.py -i - -o $OUTPUT
```


### 5. In my dataset (list of graphs), which statistics do (anti-)correlate?
Alternative questions/ideas:
- Which statistics are the most unique in my dataset? 

**Plotting script: [multi.correlate.py](../scripts/multi.correlate.py)**   
**Example: Figure 1**


```bash
INPUT= "path/to/your/stats.list.gfa"
OUTPUT= "path/to/your/output.multi.prefix"
python3 scripts/scripts/multi.correlate.py -i - -o $OUTPUT
```




