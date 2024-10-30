#!/bin/bash

# Function to display usage information
function usage() {
    echo "Usage: $0 <input_graph_file> <output_file> [separator]"
    echo "input_graph_file: The path to the input GFA file."
    echo "output_file: The path where the output should be saved."
    echo "separator: Optional. The separator character to use in the output file."
    exit 1
}

# Check if the minimum number of arguments was provided
if [ "$#" -lt 2 ]; then
    usage
fi

# Assign command line arguments to variables
input_graph="$1"
output_file="$2"
separator="${3:-}"  # Default to empty if not provided

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
    /path/to/your/rust/tool bootstrap --gfa "$input_graph" --output "$output_file" --separator "$separator" 2>&1
else
    /path/to/your/rust/tool bootstrap --gfa "$input_graph" --output "$output_file" 2>&1
fi
status=$?

# Check the exit status of the previous command
if [ "$status" -eq 0 ]; then
    echo "Execution successful. Output stored in $output_file"
else
    echo "Execution failed with status $status."
    exit "$status"
fi


python3 bootstrap -i "$output_file" -o "$output_file".png