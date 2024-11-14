#!/usr/bin/python3
#!python

# Imports
import seaborn as sns
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib as mpl
import sys
import argparse


def read_data(filename):
    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t", header = True)
        return df
    else:
        df = pd.read_csv(filename, sep = "\t", header = True)
        return df

def plotter(df, output, what):
    plt.figure(figsize = (5,4))
    plt.scatter(df.index, df[what], edgecolor = "black", color = "royalblue")
    plt.xlabel("Block_number")
    plt.ylabel("Average path size")
    plt.tight_layout()
    plt.savefig(output + "." + what + ".block.pdf")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Path: Process TSV input and PDF output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PDF file', required=True)
    args = parser.parse_args()

    df = read_data(args.input)
    plotter(df, args.output, "average_path_size")
    plotter(df, args.output, "path_size")
    plotter(df, args.output, "traversal number")