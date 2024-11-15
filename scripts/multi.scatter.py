#!/usr/bin/python3
#!python3

import argparse
import pandas as pd
import matplotlib.pyplot as plt
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


def check_col(df: pd.DataFrame, x: str) -> bool:
    """
    Check if the name is in one of the columns of the Dataframe

    :param df: Multiple graphs statistics in pandas Dataframe
    :param x: Name to check
    :return: Yes if present, no if not
    """
    return x in df.columns


def plot_scatter(df: pd.DataFrame, output: str, x: str, y: str):
    """
    Plot a scatter plot of the two columns in the dataframe

    :param df: Multiple graphs statistics in pandas Dataframe
    :param output: File name of the plot (PDF)
    :param x: First column name (x)
    :param y: Second column name (y)
    :return: Plot
    """
    plt.figure(figsize = (5,5))
    plt.scatter(df[x], df[y], color = "royalblue")
    plt.xlabel(x)
    plt.ylabel(y)
    plt.savefig(output + ".multi.scatter." + "".join(x.split()).replace("/", "") + "." + "".join(y.split()).replace("/", "") + ".scatter.pdf")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Multi: Process TSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output file', required=True)
    parser.add_argument('-x', '--x', type=str, help='X-axis', required=True)
    parser.add_argument('-y', '--y', type=str, help='Y-axis', required=True)
    args = parser.parse_args()

    files = read_stats(args.input)
    df = read_data(files)

    # Check if the cols are present
    if not check_col(df, args.x):
        print("Column not found", file = sys.stderr)
        print(args.x)
        sys.exit(1)
    if not check_col(df, args.y):
        print("Column not found", file = sys.stderr)
        print(args.y)
        sys.exit(1)

    plot_scatter(df, args.output, args.x, args.y)