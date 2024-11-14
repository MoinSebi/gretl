#!/home/svorbrugg/miniconda3/bin/python
#!python3

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
import sys
import os
import itertools


def read_stats(filename):
    files = []
    with open(filename, "r") as f:
        for line in f.readlines():
            files.append(line.split())
    return files

def read_data(files):
    combined = pd.read_csv(files[0][1], sep = "\t")

    for x in files[1:]:
        df = pd.read_csv(x[1], sep = "\t")
        combined = pd.concat([combined, df], ignore_index= True, axis = 0)
    combined.index = [x[0] for x in files]
    return combined


def check_col(df, x):
    return x in df.columns


def plot_scatter(df, output, x, y):
    plt.figure(figsize = (5,5))
    plt.scatter(df[x], df[y], color = "royalblue")
    plt.xlabel(x)
    plt.ylabel(y)
    plt.savefig(output + "." + "".join(x[0].split()).replace("/", "") + "." + "".join(x[1].split()).replace("/", "") + ".scatter.png")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Multi: Process TSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output file', required=True)
    parser.add_argument('-x', '--x', type=str, help='X-axis', required=True)
    parser.add_argument('-y', '--y', type=str, help='Y-axis', required=True)
    args = parser.parse_args()

    files = read_stats(args.input)
    df = read_data(files)

    if not check_col(df, args.x):
        print("Column not found", file = sys.stderr)
        sys.exit(1)
    if not check_col(df, args.y):
        print("Column not found", file = sys.stderr)
        sys.exit(1)

    plot_scatter(df, args.output, args.x, args.y)