#!/usr/bin/python3
#!python3

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import sys


def read_stats(filename: str) -> list:
    """
    Parse the input file and return a list of files to read.

    :param filename: File with the absolute path of the graphs to read
    :return: A list of files to read
    """
    files = []
    with open(filename, "r") as f:
        for line in f.readlines():
            files.append(line.split())
    return files


def read_data(files: list) -> pd.DataFrame:
    """
    Read all the files in the list and return a combined dataframe.

    :param files: List of all files to read
    :return: A combined dataframe of all the files
    """
    combined = pd.read_csv(files[0][1], sep = "\t")

    for x in files[1:]:
        df = pd.read_csv(x[1], sep = "\t")
        combined = pd.concat([combined, df], ignore_index= True, axis = 0)
    combined.index = [x[0] for x in files]
    return combined


def scale(df: pd.DataFrame) -> pd.DataFrame:
    """
    Scale the input data to a range of 0 to 1
    :param df: multiple graphs statistics in pandas Dataframe
    :return: Scaled Dataframe (0 to 1)
    """
    return df.apply(lambda col: col / col.max(), axis=0)


def plot_clustermap(df: pd.DataFrame, output: str) -> None:
    """
    Plot the scaled data in a clustermap (heatmap with clusering)

    :param df: Scaled graph statistics in pandas Dataframe
    :param output: Output file name prefix
    :return: Plots (clustermap)
    """
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
    plot_clustermap(df, args.output)

