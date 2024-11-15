#!/usr/bin/python3
#!python

#Import
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import sys
import argparse


def read_data(filename: str) -> pd.DataFrame:
    """
    Read the path similarity output in as DataFrame

    :param filename: Input file name (or stdin of "-")
    :return: Data in pandas DataFrame
    """
    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t")

        return df
    else:
        df = pd.read_csv(filename, sep = "\t")

        return df

def index_df(df: pd.DataFrame) -> (int, int):
    """
    Identify the index of the start and end of the sequence node part of the output

    :param df: Data in pandas DataFrame
    :return:
    """

    # Sequence starts here
    seq_index = [i for i, x in enumerate(list(df.columns)) if x.startswith("Seq")][0]

    # Data starts here
    start_index = [i for i, x in enumerate(list(df.columns)) if x.startswith("Node")][0]+1

    return seq_index, start_index


def extract_node_df(df: pd.DataFrame, seq_index: int, start_index: int) -> pd.DataFrame:
    """
    Extract the node statistics from the DataFrame

    :param df:
    :param seq_index:
    :param start_index:
    :return:
    """

    node_soft = df.apply(lambda x: sum([x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][1:-1]), axis = 1)
    node_core = df.apply(lambda x: [x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][-1], axis = 1)
    node_priv = df.apply(lambda x: [x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][0], axis = 1)
    node_pan = df.apply(lambda x: sum([x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][:]), axis = 1)

    # Create a DataFrame
    df2 = pd.DataFrame([node_core, node_soft, node_priv]).T

    # Change index and column name
    df2.columns = ["Core", "Soft", "Private"]
    df2.index = df["Accession"]

    return df2

def plot_node_stats(df2: pd.DataFrame, outfile: str) -> None:
    """
    Plot the node core, soft and private statistics

    :param df2: Core, shell and private data in pandas DataFrame
    :param outfile: Output file name
    :return: Plot (PDF)
    """

    #Plot the nodes
    (df2/1000).plot(kind = "barh", stacked = True, figsize = (6,5), width = 0.75, legend = True,
                    edgecolor = "black",
                    linewidth = 0.5,
                    color = ["royalblue", "tomato", "gold"])
    plt.xlabel("#Nodes [x1000]")
    plt.ylabel("Accession")
    plt.tight_layout()
    plt.savefig(outfile + ".ps.node.pdf")


# Sequence
def sub_df2(df: pd.DataFrame, seq_index: int) -> pd.DataFrame:
    """
    Extract the sequence statistics from the DataFrame

    :param df: Data in pandas DataFrame
    :param seq_index: Index of the sequence part of the output
    :return: Extracted data in pandas DataFrame
    """

    seq_soft = df.apply(lambda x: sum([x for x in x.iloc[seq_index:].values if not np.isnan(x)][2:-1]), axis = 1)
    seq_core = df.apply(lambda x: [x for x in x.iloc[seq_index:].values if not np.isnan(x)][-1], axis = 1)
    seq_priv = df.apply(lambda x: [x for x in x.iloc[seq_index:].values if not np.isnan(x)][1], axis = 1)

    # Read in DataFrame
    df2 = pd.DataFrame([seq_core, seq_soft, seq_priv]).T

    # Change index and column name
    df2.columns = ["Core", "Soft", "Private"]
    df2.index = df["Accession"]
    return df2

def plot_seq_stats(df2, outfile):
    """
    Plot the node core, soft and private statistics

    :param df2: Core, shell and private data in pandas DataFrame
    :param outfile: Output file name
    :return: Plot (PDF)
    """
    (df2/1000).plot(kind = "barh", stacked = True, figsize = (6,5), width = 0.75, legend = True,
                    edgecolor = "black",
                    linewidth = 0.5,
                    color = ["royalblue", "tomato", "gold"])
    plt.xlabel("Sequence [kbp]")
    plt.ylabel("Accession")
    plt.tight_layout()
    plt.savefig(outfile + ".ps.sequence.pdf")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Path: Process TSV input and PDF output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PDF file', required=True)
    args = parser.parse_args()

    df = read_data(args.input)
    seq_index, start_index = index_df(df)
    df2 = extract_node_df(df, seq_index, start_index)
    plot_node_stats(df2, args.output)
    df2 = sub_df2(df, seq_index)
    plot_seq_stats(df2, args.output)
