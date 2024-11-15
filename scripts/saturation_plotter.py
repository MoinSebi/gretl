#!/usr/bin/env python3

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import sys



def read_data(filename: str) -> pd.DataFrame:
    """

    :param filename: Gretl bootstrap output in TSV format
    :return: Data in pandas DataFrame
    """

    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t")
        return df
    else:
        return pd.read_csv(filename, sep = "\t")



def get_index(df: pd.DataFrame) -> (int, int):
    """
    Get the start index (node start) and sequence index (sequence start)
    This is needed to split the DataFrame into two parts

    :param df: Data in pandas DataFrame
    :return: Start index and sequence index
    """


    # Get the index when sequence starts (it goes till end)
    seq_index = [i for i, x in enumerate(list(df.columns)) if x.startswith("Seq")][0]

    # Get the index when nodes starts (till seq index)
    start_index = [i for i, x in enumerate(list(df.columns)) if x.startswith("Run")][0] +1
    return start_index, seq_index

def plot_node_stats(df: pd.DataFrame, seq_index: int, start_index: int, outfile: str) -> None:
    """
    Plot the saturation/bootstrapping statistics for the nodes (scatterplot)

    :param df: Gretl bootstrap output in pandas DataFrame
    :param seq_index: Index of start of sequence
    :param start_index: Index of start of nodes
    :param outfile: Output file name
    :return: Plot (PDF)
    """

    node_soft = df.apply(lambda x: sum([x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][1:-1]), axis = 1)
    node_core = df.apply(lambda x: [x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][-1], axis = 1)
    node_private = df.apply(lambda x: [x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][0], axis = 1)
    node_pan = df.apply(lambda x: sum([x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][:]), axis = 1)
    # Plot figure (nodes)
    plt.figure(figsize = (6,4))
    plt.scatter(df["Size"], node_private/1000, edgecolor = "black")
    plt.scatter(df["Size"], node_soft/1000, edgecolor = "black")
    plt.scatter(df["Size"],  node_core/1000, edgecolor = "black")
    plt.scatter(df["Size"],  node_pan/1000, edgecolor = "black")
    plt.xlabel("Number of genomes")
    plt.ylabel("#Nodes [x10³]")
    plt.tight_layout()
    plt.savefig(outfile + ".bootstrap.node.pdf")

def plot_seq_stats(df: pd.DataFrame, seq_index: int, outfile: str) -> None:
    """
    Plot the saturation/bootstrapping statistics for the sequence (scatterplot)

    :param df: Gretl bootstrap output in pandas DataFrame
    :param seq_index: Index of start of sequence
    :param outfile: Output file name
    :return: Plot (PDF)
    """

    seq_soft = df.apply(lambda x: sum([x for x in x.iloc[seq_index:].values if not np.isnan(x)][1:-1]), axis = 1)
    seq_core = df.apply(lambda x: [x for x in x.iloc[seq_index:].values if not np.isnan(x)][-1], axis = 1)
    seq_private = df.apply(lambda x: [x for x in x.iloc[seq_index:].values if not np.isnan(x)][0], axis = 1)
    seq_pan = df.apply(lambda x: sum([x for x in x.iloc[seq_index:].values if not np.isnan(x)][:]), axis = 1)
    plt.figure(figsize = (6,4))
    plt.scatter(df["Size"], seq_private/1000, edgecolor = "black", label = "Private")
    plt.scatter(df["Size"], seq_soft/1000, edgecolor = "black", label = "Soft")
    plt.scatter(df["Size"],  seq_core/1000, edgecolor = "black", label = "Core")
    plt.scatter(df["Size"],  seq_pan/1000, edgecolor = "black", label = "Pan")
    plt.xlabel("Number of genomes")
    plt.ylabel("Sequence [kbp]")
    plt.legend()
    plt.tight_layout()
    plt.savefig(outfile + ".bootstrap.seq.pdf")



if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Bootstrap: Process CSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PNG file', required=True)
    args = parser.parse_args()

    # Load the CSV file
    data = read_data(args.input)
    start_index, seq_index = get_index(data)
    plot_seq_stats(data, seq_index, args.output)
    plot_node_stats(data, seq_index, start_index, args.output)