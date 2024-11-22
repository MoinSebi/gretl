#!/usr/bin/env python3

# Imports
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import argparse
import sys
import logging

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s    [%(levelname)s] - %(filename)s: %(message)s',
    datefmt='%d/%m/%Y %H:%M:%S',  # 24-hour format
    handlers=[logging.StreamHandler(stream=sys.stderr)]
)


def read_gfa(filename: str) -> dict:
    """
    Node to length dictionary

    :param filename: Input GFA file
    :return: Node to length dictionary
    """
    # Read the nodes of the GFA file
    # dict(nodeID -> len)
    node2len = dict()
    with open(filename) as file:
        for line in file.readlines():
            if line.startswith("S"):
                ls = line.split()
                node2len[int(ls[1])] = len(ls[2])

    # Check
    return node2len


def read_nwindow_tsv(filename: str) -> pd.DataFrame:
    """
    Read the gretl nwindow  output in as DataFrame

    :param filename: Input file name (or stdint of "-")
    :return: Pandas DataFrame of the data
    """
    # Read the nwindow output in as DataFrame
    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t")
    else:
        df = pd.read_csv(filename, sep = "\t")
    return df


def get_poslist(df: pd.DataFrame, node2len: dict) -> list:
    """
    Get the position list for the nodes

    :param df: Gretl nwindow output in pandas DataFrame
    :param node2len: Node to length dictionary
    :return: List of pangenomic positions
    """

    current_pos = 0
    pos_list = []
    for x in df.index:
        pos_list.append(current_pos + node2len[x+1])
        current_pos += node2len[x+1]
    return pos_list


def plot_scatter(df: pd.DataFrame, pos_list: list, output: str, ylabel: str, xlabel: str, df_col: str, correction = True):
    """
    Plot a scatter plot of the two columns in the dataframe
    Correct it by length of the nodes of wanted (default: off)

    :param df: Gretl nwindow output in pandas DataFrame
    :param pos_list: Positional list of the nodes
    :param output: Output file name
    :param ylabel: y label
    :param xlabel: x label
    :param df_col: Dataframe column to plot
    :param correction: Length correction of not
    :return: Plot
    """

    # Plot #Nodes
    plt.figure(figsize = (8,3))
    cor1 = "correction"
    if correction:
        x = np.array(pos_list)/1000
    else:
        cor1 = "no_correction"
        x = df["Node_id"]


    plt.scatter(x, df[df_col], color = "black", s = 2)
    plt.ylabel(ylabel)
    plt.xlabel(xlabel)
    plt.tight_layout()
    plt.savefig(output + ".nwindow." + df_col + "." + cor1 + ".pdf")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Bootstrap: Process CSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-g', '--graph', type=str, help='Graph', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PNG file', required=True)
    args = parser.parse_args()

    logging.info("Reading graph from %s", args.graph)
    node2len = read_gfa(args.graph)

    logging.info("Reading data from %s", args.input)
    df = read_nwindow_tsv(args.input)

    logging.info("Positional list")
    pos_list = get_poslist(df, node2len)

    logging.info("Plotting")
    plot_scatter(df, pos_list, args.output, "#Nodes (in one window)", "Node ID (length corrected) [x10³]", "Nodes")
    plot_scatter(df, pos_list, args.output, "Sequence (in one window) [kbp]", "Node ID (length corrected) [x10³]", "Sequences")
    plot_scatter(df, pos_list, args.output, "Sum of Jumps (in one window) [x10⁶]", "Node ID (length corrected) [x10³]", "Jumps")
    plot_scatter(df, pos_list, args.output, "#Nodes (in one window)", "Node ID (length corrected) [x10³]", "Nodes", correction=False)
    plot_scatter(df, pos_list, args.output, "Sequence (in one window) [kbp]", "Node ID (length corrected) [x10³]", "Sequences", correction=False)
    plot_scatter(df, pos_list, args.output, "Sum of Jumps (in one window) [x10⁶]", "Node ID (length corrected) [x10³]", "Jumps", correction=False)





