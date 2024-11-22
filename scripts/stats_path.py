#!/usr/bin/env python3


# Imports
import sys
import argparse
import pandas as pd
import matplotlib.pyplot as plt
import logging

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s    [%(levelname)s] - %(filename)s: %(message)s',
    datefmt='%d/%m/%Y %H:%M:%S',  # 24-hour format
    handlers=[logging.StreamHandler(stream=sys.stderr)]
)

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
    return df

def check_col(df: pd.DataFrame, x) -> bool:
    """
    Check if the name is in one of the columns of the Dataframe

    :param df: Path statistics in pandas Dataframe
    :param x: Name to check
    :return: Yes if present, no if not
    """
    if x in df.columns:
        return True


def plotter(df: pd.DataFrame, x: str, y: str, output: str) -> None:
    """
    Plot the scatter plot of the data.

    :param df: Path statistics in pandas Dataframe
    :param x: Name of x variable
    :param y: Name of y variable
    :param output: Output file name
    :return: Plot
    """

    plt.figure(figsize = (5,4))
    plt.scatter(data=df, x=x, y=y, edgecolor = "black", color = "royalblue")

    #for x1 in df.iterrows():
    #    plt.annotate(x1[0], (x1[1][x] + 0.0005, x1[1][y]))
    plt.xlabel(x)
    plt.ylabel(y)
    plt.tight_layout()
    plt.savefig(output + ".path.scatter.pdf")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Path: Process TSV input and PDF output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PDF file', required=True)
    parser.add_argument('-x', '--x', type=str, help='X-axis', required=True)
    parser.add_argument('-y', '--y', type=str, help='Y-axis', required=True)
    args = parser.parse_args()

    logging.info("Reading data from %s", args.input)
    df = read_path_stats(args.input)
    if not check_col(df, args.x):
        print("Column not found", file = sys.stderr)
        sys.exit(1)
    if not check_col(df, args.y):
        print("Column not found", file = sys.stderr)
        sys.exit(1)

    logging.info("Plotting the scatter plot")
    plotter(df, args.x, args.y, args.output)
