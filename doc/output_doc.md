# Collected workflows

We have added a collection of python scripts stored in the `scripts` directory. When used correctly, these scripts can be used to generate a variety of plots from the output of the gretl toolkit. Since many scripts return multiple plots, we have included a prefix option to allow you to specify the output file name. In the following, we provide a list of workflows that can be used to answer specific questions about your graph(s). These can be used as a "follow-along" guide to the gretl toolkit.

The python scripts need the following requirements (in brackets are the versions we used):
- python3 (3.10)
- matplotlib (3.1.2)
- pandas (1.2.3)
- seaborn (0.11.0)
- numpy (1.21.6)


## Single graph workflows 

### 1. How does my graph saturate?
Alternative questions: 
- Do the genomes of my graph saturate with the number of samples?
- How many genomes do I need to sample to capture the full diversity of my organism? (This needs to be identified by vizualization)


**Subcommand: bootstrap**  
**Plotting script: saturation_plotter.py**   
**Example: Figure 1** 


** Run bootstrap + plot**
```bash
input= file/to/your/graph
output= file/to/your/output.prefix
target/release/gretl bootstrap -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/saturation_plotter.py -i - -o hello_saturation.png
```

### 2. Show me the local complexity of the graph
Alternative questions:
- Which node is the most connected in my graph? (Identify this by the tabular output provided by the gretl toolkit)
- Are there regions which show exceptional high connectivity?

**Subcommand: nwindow**  
**Plotting script: nwindow.py**  
**Example: Figure 1**

```bash
input= file/to/your/graph
output= file/to/your/output.prefix
target/release/gretl nwindow -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/saturation_plotter.py -i - -o hello_saturation.png
```



### 3. On a genomic linear scale, which regions are similar to each other and which are not?
Alternative questions:
- Which regions of my graph show high local similarity?


**Subcommand: window**  
**Plotting script: window.py**   
**Example: Figure 1**


```bash
input= file/to/your/graph
output= file/to/your/output.prefix
target/release/gretl bootstrap -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/saturation_plotter.py -i - -o hello_saturation.png
```

### 4. How much core, shell and private sequence do I have?
Alternative questions:
- How much of my graph is core, shell and private sequence?
- If running this for different graphs? Which graph exhibit the most core sequence? 

**Subcommand: core**  
**Plotting script: core.py**  
**Example: Figure 1**



```bash
input= file/to/your/graph
output= file/to/your/output.prefix
target/release/gretl bootstrap -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/saturation_plotter.py -i - -o hello_saturation.png
```

### 5. My graph seems to be highly connected, greate pangenome blocks and return statistics for each block? 
Alternative questions:
- Which regions of my graph contain a lot of sequence? 
- Which regions of my graphs have a lot of path running through?  

**Subcommand: block**  
**Plotting script: block.py**  
**Example: Figure 1**

```bash
input= file/to/your/graph
output= file/to/your/output.prefix
target/release/gretl block -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/block.py -i - -o hello_saturation.png
```


### 6. I have selected one graph in my data set, now I am interested in the paths
Alternative questions:
- In direct comparison, which paths share most of their statistics? (Seem to be the closest)
- Is there any path that is unique to all the other paths?

```bash
input= file/to/your/graph
output= file/to/your/output.prefix
target/release/gretl path -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/path.py -i - -o hello_saturation.png
```

### 7. I have selected on graph in my data set, how much does each path contain of core, shell and private sequence?
Alternative questions:
- Which path contains the most core sequence?

**Subcommand: path**
**Plotting script: path.py**
**Example: Figure 1**

```bash
input= file/to/your/graph
output= file/to/your/output.prefix
target/release/gretl path -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/path.py -i - -o hello_saturation.png
```



## Multi-graph workflows - Start with running the stats subcommand for all graphs
First, create a full set of statistics for all graphs


```bash
INPUT="path/to/your/input_list.txt"
OUTPUT="path/to/your/output.prefix

for file in $(cat $INPUT); do
    target/release/gretl stats -g $file -o $output. 
done

ls $output.*.stats > $output.stats.list
```



### 1. Differences between them? (histogram for each feature) 
```bash 
# See above
INPUT=$output.stats.list
for file in $(cat $output.stats.list); do
    python3 scripts/scripts/stats.py -i $file -o $output
```

### 2. Differences between them? (heatmap - representation)
```bash 
# See above
INPUT=$output.stats.list
for file in $(cat $output.stats.list); do
    python3 scripts/scripts/stats.py -i $file -o $output
```


### 3. Which features span the most interesting range of values? (Scatter plot)

### 4. Show me scatter plot of these two features (name feature here)? (Scatter plot)
- python1.py -l list.graph.txt -x "Unique nodes" -y "Unique edges" -o plot1



### 5. Do some of the features calculated by gretl correlate in many of inputed graphs? (heatmap)





