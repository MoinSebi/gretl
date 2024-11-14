#!/usr/bin/python3
#!python3

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
import sys

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

def scale(df):
    return df.apply(lambda col: col / col.max(), axis=0)

def plot_heatmap(df, output):
    dfnew = df.dropna(how = "all", axis = 1)
    if len(dfnew.columns) == 0:
        print("No data to plot", file = sys.stderr)
        return
    else:
        sns.clustermap(dfnew.T)
        plt.xticks([])  # Remove x-axis tick labels
        plt.savefig(output + ".multi.heatmap.png")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Multi: Process TSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output file', required=True)
    args = parser.parse_args()

    files = read_stats(args.input)
    df = read_data(files)
    df = scale(df)
    print (df)
    plot_heatmap(df, args.output)

