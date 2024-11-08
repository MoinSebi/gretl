# Nwindow
## Description/Research question
Which nodes of my graph show high local complexity?

## Input data
input= file/to/your/graph
output= file/to/your/output.prefix

## Run nwindow + plot
target/release/gretl nwindow -g {input} -o - 



Possible bashscript (copy this and adjust input and output)

Heatmap 
```bash

python3 scripts/scripts/nwindow.py -i - -o ${output}
```