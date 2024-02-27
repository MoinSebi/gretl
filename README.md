# gretl - Graph evaluation toolkit
#### Description 
Small tool for basic graph statistics using GFA format. 
  


#### Installation: 

**Git**  
```
git clone https://github.com/MoinSebi/gretl  
cd gretl   
cargo build --release  
./target/release/gretl  
```


## Usage
### Stats
Graph or path statistics. Graph statistics also include "hybrid" statistics, which are average and standard deviation of path statistics. All hybrid stats have the prefix "Path". 
```text
gretl-stats 

Create statists about the graph or its path

USAGE:
    gretl stats [FLAGS] [OPTIONS] --gfa <gfa> --output <output>

FLAGS:
    -h, --help       Print help information
        --haplo      Make stats for each haplotype (not sample, not path). Only in combination with
                     Pan-SN
    -p, --path       Report path statistics (default:off -> report graph stats)
    -V, --version    Print version information
    -y               Report output in YAML format (default:off -> report in tsv)

OPTIONS:
        --bins <bins>        Size of bins. Example: Format 10,20,30 -> (0-10, 11-20, 30+)[default:
                             1,50,100,1000]
    -g, --gfa <gfa>          Input GFA file
    -o, --output <output>    Output
        --pansn <Pan-SN>     Separate by first entry in Pan-SN spec

```


### Feature
Extract feature which do not fall into a specific setting. The output can be used in gfa2bin. 

````
gretl-feature 

Get list of nodes which do not fall into settings

USAGE:
    gretl feature [OPTIONS] --gfa <gfa> --output <output>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -d, --min-depth <min-depth>      Minimum depth
    -D, --max-depth <max-depth>      Maximum node depth
    -g, --gfa <gfa>                  Input GFA file
    -l, --min-len <min-len>          Minimum length
    -L, --max-len <max-len>          Maximum node length 
    -n, --min-degree <min-degree>    Minimum degree
    -N, --max-degree <max-degree>    Maximum node degree 
    -o, --output <output>            Output file name

````

### Paths
Extract paths which do not fall into a specific setting. The output can be used in gfa2bin.

````
gretl-path

Remove paths

USAGE:
gretl path [FLAGS] --gfa <gfa> --output <output> --stats <stats> --mins <mins> --maxs <maxs>

FLAGS:
-h, --help       Print help information
--haplo      Haplo mode
-V, --version    Print version information

OPTIONS:
-g, --gfa <gfa>          Input GFA file
-m, --mins <mins>...
-M, --maxs <maxs>...
-o, --output <output>    Output file
-s, --stats <stats>...      Which stats to filter?
````

### ID2INT
Convert any string-based node identifier to numeric values.

**Comment**: Do not use when graph is already digits only. The order of identifier does not reflect a sorted graph structure. I highly advise to use ```odgi sort``` on the graph. If not, some statistics are not very meaningful. 
```text
gretl-id2int 

USAGE:
    gretl --gfa <gfa> --output <output> id2int [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -d, --dict <dict>    Write a dictionary for Old->New identifiers in this file.

```
**Example usage**
```text
./gretl -g /path/to/graph.gfa -o /path/to/output.gfa id2int -d /path/to/dict.txt
```

### Node-list
Get several statistics for each node in the graph. 

```text
gretl-node-list 

Some information about each node

USAGE:
    gretl node-list [OPTIONS] --gfa <gfa> --output <output>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -f, --feature <Features>    Name the features you need. If nothing is used, report everything.
                                Example -f Length, Core
    -g, --gfa <gfa>             Input GFA file
    -o, --output <output>       Output
    -s, --pansn <Pan-SN>        Separate by first entry in Pan-SN spec

```


### Core
Compute core statistics of the graph. 

```text
gretl-core 

Graph similarity statistics

USAGE:
    gretl --gfa <gfa> --output <output> core

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information
```

**Example usage**
```text
./gretl -g /path/to/graph.gfa -o /path/to/core.stats.txt core
```
**Output plotted**


### Nwindow
Summarizing the graph by a window of nodes.


````
gretl-nwindow

USAGE:
    gretl nwindow --gfa <gfa> --output <output>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

Input options:
    -g, --gfa <gfa>         Input GFA file
        --pansn <Pan-SN>    Seperate by first entry in Pan-SN spec

Window criteria options:
        --jumps                  Sum of 'id jumps' away from the starting node
        --sequence <sequence>    Amount of sequence away from the starting node
        --step <steps>           Number of steps away from the starting node

Window summary options:
        --jumps-summary      
        --node-number        
        --sequence-length    

Output option:
    -o, --output <output>    Output

````


### (Sliding, path) window
Summarizing the graph by a window of sequence in the path


````
gretl-window 

Sliding window along the samples

USAGE:
    gretl window [FLAGS] [OPTIONS] --gfa <gfa> --output <output>

FLAGS:
    -h, --help       Print help information
    -n, --node       Window on nodes nodes ([default: off] -> on sequence)
    -V, --version    Print version information

OPTIONS:
    -g, --gfa <gfa>          Input GFA file
    -m, --metric <metric>    Which metric
    -o, --output <output>    Output
        --pansn <Pan-SN>     Seperate by first entry in Pan-SN spec
    -s, --window <size>      Window on sequence
        --step <step>        Step size (If nothing is set, step size -> bin size

````

### Bootstrap 

We recommend bootstrapping a graphs in PanSN-spec. Use ```--nodes``` if the bootstrap should only run on a subset of nodes (e.g. gene vs intergenic).
 ```
 gretl-bootstrap 

Bootstrap approach

USAGE:
    gretl bootstrap --gfa <gfa>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

Input options:
    -g, --gfa <gfa>                  Input GFA file
        --meta-input <meta input>    Take a specific meta file as input
        --nodes <nodes>              Run bootstrap only on these nodes
        --pansn <Pan-SN>             Separate by first entry in Pan-SN spec
    -t, --threads <Threads>          Number of threads [default: 1]

Modifications:
        --level <level>            Calculate a specific level
        --meta-line <meta line>    Take a specific line of the meta file (only works when meta file
                                   is provided)
        --number <number>          How many bootstraps do you want to run

Output options:
        --meta <meta>        Report an additional meta file with all combinations
    -o, --output <output>    Output

```
### Path similarity (PS)

```
gretl-ps 

Detailed similarity information for each path

USAGE:
    gretl ps [OPTIONS] --gfa <gfa> --output <output>

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -g, --gfa <gfa>          Input GFA file
    -o, --output <output>    Output
        --pansn <Pan-SN>     Seperate by first entry in Pan-SN spec


```


## Scripts 
We provide multiple jupyter notebooks to visualize the output of the tool. 

**Requirements**
- Jupyter
- Matplotlib
- Pandas
- Numpy
- Seaborn

