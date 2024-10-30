#!/bin/bash

# Function to display usage information
function usage() {
    echo "Usage: $0 <tool_path> <input_graph_file> <output_file> [separator]"
    echo "tool_path: The full path to the Rust tool executable."
    echo "input_graph_file: The path to the input GFA file."
    echo "output_file: The path where the output should be saved."
    echo "separator: Optional. The separator character to use in the output file."
    exit 1
}

# Check if the minimum number of arguments was provided
if [ "$#" -lt 3 ]; then
    usage
fi

# Assign command line arguments to variables
tool_path="$1"
input_graph="$2"
output_file="$3"
separator="${4:-}"  # Default to empty if not provided

# Check if the tool exists
if [ ! -f "$tool_path" ]; then
    echo "Error: Tool executable does not exist."
    exit 1
fi

# Check if the input file exists
if [ ! -f "$input_graph" ]; then
    echo "Error: Input file does not exist."
    exit 1
fi

# Warn and ask for confirmation if the output file already exists
if [ -f "$output_file" ]; then
    echo "Warning: The output file already exists."
    read -p "Do you want to overwrite it? (y/n): " -n 1 -r
    echo    # Move to a new line
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Exiting without overwriting the file."
        exit 1
    fi
fi

# Run the Rust tool, include the separator if provided
if [[ -n "$separator" ]]; then
    "$tool_path" bootstrap --gfa "$input_graph" --output "$output_file" --pansn "$separator" 2>&1
else
    "$tool_path" bootstrap --gfa "$input_graph" --output "$output_file" 2>&1
fi
status=$?

# Check the exit status of the previous command
if [ "$status" -eq 0 ]; then
    echo "Execution successful. Output stored in $output_file"
else
    echo "Execution failed with status $status."
    exit "$status"
fi




python3 ../scripts/scripts/saturation_plotter.py -i "$output_file" -o "$output_file"