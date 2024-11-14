#!/usr/bin/python3
#!python

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

def read_path_stats(filename):
    df = pd.read_csv(filename, sep = "\t")
    df.index = [x.split("#")[0] for x in df["Path"]]
    df = df.drop("Path", axis = 1)
    df_normalized = df.divide(df.max())
    return df

def filter_df(df):
    col = [x for x in df.columns if "median" not in x]
    df = df[col]
    return df

def plot_heatmap(df, output):
    plt.figure(figsize = (5,10))
    sns.clustermap(df.T)
    plt.savefig(output + "path.heatmap.pdf")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Path: Process TSV input and PDF output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PDF file', required=True)
    args = parser.parse_args()

    df = read_path_stats(args.input)
    df = filter_df(df)
    plot_heatmap(df, args.output)