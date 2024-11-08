# Saturation analysis
## Research question: 
Do the genomes of my graph saturate with the number of samples?

## Input data
input= file/to/your/graph
output= file/to/your/output.prefix

# Run bootstrap + plot
```bash
input= file/to/your/graph
output= file/to/your/output.prefix
target/release/gretl bootstrap -g data/example_data/chr5.yeast.gfa -o -  |   python3 scripts/scripts/saturation_plotter.py -i - -o hello_saturation.png
```


