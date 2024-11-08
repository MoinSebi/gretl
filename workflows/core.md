# Core pipeline 


input= file/to/your/graph
output= file/to/your/output.prefix

# Run core + plot
target/release/gretl core -g {input} -o -  |   python3 scripts/scripts/core.py -i - -o ${output}
