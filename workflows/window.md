# Window
## Description/Research question
Which regions of my graph show high local similarity?  

## Explained
Statistics are calculated on a graph level, but can be transferred to genomic scale, later merged on a window level. 


## Run window + plot

input= file/to/your/graph  
output= file/to/your/output.prefix

target/release/gretl window -g {input} -o -  |   python3 scripts/scripts/window.py -i - -o ${output}

## bashscript (copy this and adjust input and output)
```bash
input= file/to/your/graph
output= file/to/your/output.prefix
target/release/gretl window -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/window.py -i - -o hello_window.png
```