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
Convert any string as node identifier to numeric.  

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

**Output plotted**



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






