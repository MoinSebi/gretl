#!/usr/bin/python3
#!python

# Imports
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import argparse
import sys


def read_data(filename: str) -> pd.DataFrame:
    """
    Parse the "gretl window" output in DataFrame

    :param filename: Input file name (or stdin of "-")
    :return: Data in pandas DataFrame
    """
    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t", header = None)
        df.set_index(0, inplace=True)

        return df
    else:
        df = pd.read_csv(filename, sep = "\t", header = None)
        df.set_index(0, inplace=True)

        return df

def plotter_window(df: pd.DataFrame, output: str) -> None:
    """
    Plot the heatmap of the data
    - Each column in one window (always same size)
    - Each row is a path


    :param df:
    :param output:
    :return:
    """


    # Plot the figure
    plt.figure(figsize=(10, 6))  # Set the figure size
    sns.heatmap(df, cmap="coolwarm", cbar_kws={'label': 'Similarity'})
    plt.tight_layout(rect=[0.05, 0.05, 0.85, 0.95])  # Adjust as needed
    plt.xlabel("Sequence window")
    plt.savefig(output + ".window.pdf")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Window: Process TSV input and PDF output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PDF file', required=True)
    args = parser.parse_args()

    df = read_data(args.input)
    plotter_window(df, args.output)
