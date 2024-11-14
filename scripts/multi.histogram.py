#!/home/svorbrugg/miniconda3/bin/python
#!python3

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import seaborn as sns
import sys
import os


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

def plot_test(df, output):
    for x in df.columns:
        plt.figure(figsize = (5,4))
        plt.hist(df[x], edgecolor = "black", color = "royalblue")
        plt.xlabel(x)
        plt.ylabel("Frequency")
        plt.tight_layout()
        plt.savefig(output + "." + "".join(x.split()).replace("/", "") + ".block.pdf")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Multi: Process TSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output file', required=True)
    args = parser.parse_args()

    files = read_stats(args.input)
    df = read_data(files)
    os.makedirs(args.output, exist_ok=True)

    plot_test(df, args.output + "/" + args.output)

