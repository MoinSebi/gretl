#!/usr/bin/python3
#!python

# Imports
import seaborn as sns
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib as mpl
import sys
import argparse


def read_data(filename: str) -> pd.DataFrame:
    """
    Read the data from a TSV file (from gretl block) and return a pandas DataFrame

    :param filename: File name
    :return: Block data in a pandas DataFrame
    """
    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t")
        return df
    else:
        df = pd.read_csv(filename, sep = "\t")
        return df

def plotter(df: pd.DataFrame, output: str, what: str) -> None:
    """
    Plot each of the different stats of a block to a separate PDF file

    :param df: Block data in pandas DataFrame
    :param output: Output file name
    :param what: What to plot (average_path_size, path_size, traversal number)
    :return:
    """

    plt.figure(figsize = (5,4))
    plt.scatter(df.index, df[what], edgecolor = "black", color = "royalblue")
    plt.xlabel("Block number")
    plt.ylabel(what)
    plt.tight_layout()
    plt.savefig(output + ".block." + "".join(what.split()) + ".pdf")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Path: Process TSV input and PDF output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PDF file', required=True)
    args = parser.parse_args()

    df = read_data(args.input)
    plotter(df, args.output, "#Nodes (average)")
    plotter(df, args.output, "Sequence [bp] (average)")
    plotter(df, args.output, "#Paths")
    plotter(df, args.output, "#Traversals")