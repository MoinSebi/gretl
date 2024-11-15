#!/usr/bin/python3
#!python

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import sys

def read_path_stats(filename: str) -> pd.DataFrame:
    """
    Read the gretl path statistics from the input file
    Set path as index and normalize the data by max

    :param filename: Input file name
    :return: Data in pandas DataFrame
    """
    if filename == "-":
        df = pd.read_csv(sys.stdin, sep = "\t")
    else:
        df = pd.read_csv(filename, sep = "\t")
    df.index = df["Path"]
    df = df.drop("Path", axis = 1)
    df_normalized = df.divide(df.max())
    return df_normalized


def filter_df(df: pd.DataFrame) -> pd.DataFrame:
    """
    Filter the DataFrame to remove the "median" columns

    :param df: Data in pandas DataFrame
    :return: Filtered data frame
    """
    col = [x for x in df.columns if "median" not in x]
    df = df[col]
    return df



def plot_heatmap(df: pd.DataFrame, output: str) -> None:
    """
    Plot the heatmap of the data

    :param df: Filtered data in pandas DataFrame
    :param output: Output file name
    :return:
    """
    df = df.dropna(axis = 1)
    plt.figure(figsize = (5,10))
    sns.clustermap(df.T)
    plt.savefig(output + ".path.heatmap.pdf")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Path: Process TSV input and PDF output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PDF file', required=True)
    args = parser.parse_args()

    df = read_path_stats(args.input)
    df = filter_df(df)
    plot_heatmap(df, args.output)