
input= file/to/your/graph
output= file/to/your/output.prefix

# Run bootstrap + plot

bash workflows/saturation.sh target/release/gretl --gfa data/assembly.gfa -o -  |
  python3 scripts/scripts/saturation_plotter.py -i - -o data/saturation.png

