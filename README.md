# gfastats
#### Description 
Small tool for basic graph statistics using GFA format.   
  


#### Installation: 

**Git**  
```
git clone https://github.com/MoinSebi/gfastats  
cd gfastats   
cargo build --release  
./target/release/gfastats  
```

**Cargo** 
```
cargo install gfastats
```

## Usage

### ID2INT
Convert any string-based node identifier to numeric values.

**Comment**: Do not use when graph is already digits only. The order of identifier does not reflect a sorted graph structure. I highly advise to use ```odgi sort``` on the graph. If not, some statistics are not very meaningful. 
```text
gfastats-id2int 

USAGE:
    gfastats --gfa <gfa> --output <output> id2int [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -d, --dict <dict>    Write a dictionary for Old->New identifiers in this file.

```
**Example usage**
```text
./gfastats -g /path/to/graph.gfa -o /path/to/output.gfa id2int -d /path/to/dict.txt
```




### Core
Compute core statistics of the graph. 

```text
gfastats-core 

Graph similarity statistics

USAGE:
    gfastats --gfa <gfa> --output <output> core

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information
```

**Example usage**
```text
./gfastats -g /path/to/graph.gfa -o /path/to/core.stats.txt core
```
**Output plotted**


### Nwindow
Summarizing the graph by a window of nodes.


````
gfastats-nwindow

USAGE:
    gfastats nwindow --gfa <gfa> --output <output>

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

### Bootstrap 

We recommend bootstrapping a graphs in PanSN-spec. Use ```--nodes``` if the bootstrap should only run on a subset of nodes (e.g. gene vs intergenic).
 ```
 gfastats-bootstrap 

Bootstrap approach

USAGE:
    gfastats bootstrap --gfa <gfa>

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

## Scripts 
We provide multiple jupyter notebooks to visualize the output of the tool. 

**Requirements**
- Jupyter
- Matplotlib
- Pandas
- Numpy
- Seaborn

