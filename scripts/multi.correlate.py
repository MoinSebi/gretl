#!/home/svorbrugg/miniconda3/bin/python
#!python3

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns


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


def correlate(df: pd.DataFrame) -> pd.DataFrame:
    """
    Correlate the input statistics of multiple graphs

    :param df: Input statistics of multiple graphs
    :return: Correlation matrix of the input statistics
    """
    dfcorr = df.corr()
    df2 = dfcorr.dropna(how = "all", axis = 1)
    df2 = df2.dropna(how = "all", axis = 0)

    return df2


def plot_heatmap(df: pd.DataFrame, output: str) -> None:
    """
    Plot heatmap of the correlating matrix

    :param df: Correlation matrix in a pandas DataFrame
    :param output: Output file name
    :return: Plots
    """
    sns.clustermap(df.T)
    plt.xticks([])  # Remove x-axis tick labels
    plt.savefig(output + ".multi.corr.heatmap.pdf")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Multi: Process TSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output file', required=True)
    args = parser.parse_args()

    files = read_stats(args.input)
    df = read_data(files)
    df = correlate(df)
    plot_heatmap(df, args.output)