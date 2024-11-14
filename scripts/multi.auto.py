#!/usr/bin/python3
#!python3

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import os
import itertools
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
            ls = line.split()
            if os.path.exists(ls[1]):
                files.append(line.split())
            else:
                print(ls[1] + " does not exist", file = sys.stderr)
    return files

def read_data(files: list) -> pd.DataFrame:
    """
    Read all the files in the list and return a combined dataframe.

    :param files: List of all files to read
    :return: A combined dataframe of all the files
    """
    if len(files) == 0:
        raise ValueError("No files to read")
    combined = pd.read_csv(files[0][1], sep = "\t")

    for x in files[1:]:
        df = pd.read_csv(x[1], sep = "\t")
        combined = pd.concat([combined, df], ignore_index= True, axis = 0)
    combined.index = [x[0] for x in files]
    return combined

def relative_deviation(vector: np.array) -> float:
    """
    Calculate the relative deviation of a vector.

    :param vector: Vector of values (numeric)
    :return: The relative deviation of the vector
    """
    mean = np.mean(vector)
    if mean == 0:
        return 0  # to handle division by zero if mean is zero
    deviations = np.abs(vector - mean) / mean
    return np.sum(deviations)

def cal_dev(df: pd.DataFrame) -> pd.DataFrame:
    """

    :param df: Dataframe of all the statistics
    :return: Dataframe with the relative deviation of each column
    """
    dev = []
    for x in df.columns:
        dev.append(relative_deviation(df[x]))
    df = pd.concat([df, pd.DataFrame([dev], index = ["dev"], columns = df.columns)], axis = 0)
    return df

def get_topX(df: pd.DataFrame, number: int = 10) -> pd.DataFrame:
    """
    Get the top 10 values of the relative deviation.
    :param df: Input data (with dev)
    :param number: Number of (top) values to return
    :return: Dataframe of the top 10 values of the relative deviation
    """
    df = df.T
    df = df.sort_values("dev", ascending = False)
    df = df[:10]
    return df

def plot_scatter(df: pd.DataFrame, output: str):
    """
    Plot of each combination of the top X values (see above) in a scatter plot.

    :param df: Dataframe of the top X values
    :param output: Output file name
    :return:
    """
    df = df.T
    combi = list(itertools.combinations(df.columns, 2))
    print(len(combi))
    for x in combi:
        plt.figure(figsize = (5,5))
        plt.scatter(df[x[0]], df[x[1]], color = "royalblue")
        plt.xlabel(x[0])
        plt.ylabel(x[1])
        plt.savefig(output + "." + "".join(x[0].split()).replace("/", "") + "--" + "".join(x[1].split()).replace("/", "") + ".scatter.png")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Multi: Process TSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output file', required=True)
    parser.add_argument('-n', '--number', type=int, help='Number of statistics (scaling is O(n^2))', required=False)
    args = parser.parse_args()

    files = read_stats(args.input)
    df = read_data(files)
    df = cal_dev(df)
    df = get_topX(df, args.number)
    os.makedirs(args.output, exist_ok=True)

    plot_scatter(df, args.output + "/")