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

def relative_deviation(vector):
    mean = np.mean(vector)
    if mean == 0:
        return 0  # to handle division by zero if mean is zero
    deviations = np.abs(vector - mean) / mean
    return np.sum(deviations)

def cal_dev(df):
    dev = []
    for x in df.columns:
        dev.append(relative_deviation(df[x]))
    df = pd.concat([df, pd.DataFrame([dev], index = ["dev"], columns = df.columns)], axis = 0)
    print(df)
    return df

def get_top10(df):
    df = df.T
    df = df.sort_values("dev", ascending = False)
    df = df.head(10)
    return df

def plot_scatter(df, output):
    df = df.T
    combi = list(itertools.combinations(df.columns, 2))
    print(combi)
    for x in combi:
        plt.figure(figsize = (5,5))
        plt.scatter(df[x[0]], df[x[1]], color = "royalblue")
        plt.xlabel(x[0])
        plt.ylabel(x[1])
        plt.savefig(output + "." + "".join(x[0].split()).replace("/", "") + "." + "".join(x[1].split()).replace("/", "") + ".scatter.png")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Multi: Process TSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output file', required=True)
    args = parser.parse_args()

    files = read_stats(args.input)
    df = read_data(files)
    df = cal_dev(df)
    print(df)
    df = get_top10(df)
    os.makedirs(args.output, exist_ok=True)

    plot_scatter(df, args.output + "/" + args.output)