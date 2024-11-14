#!/usr/bin/python3
#!python

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import sys


def read_data(filename: str) -> (pd.DataFrame, pd.DataFrame):
    """
    Read the output of gretl core and split the data into private and public data.
    Return two dataframes

    :param filename: Input file name
    :return: Two dataframes, one for private and one for public data
    """

    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t", index_col=0)
    else:
        df = pd.read_csv(filename, sep = "\t", index_col=0)

    # Separate the two dataframes
    df_priv = df.loc[df["Name"].apply(lambda x: not x.isdigit())]
    df_priv = df_priv.sort_values("Sequence[bp]")
    df_pub = df.loc[df["Name"].apply(lambda x: x.isdigit())]
    return df_priv, df_pub


def plot_test(df: pd.DataFrame, df_seq_sorted: pd.DataFrame, output: str) -> None:
    """
    Plot the data from private and public data into a PDF file.
    Sequence and nodes are plotted in two separate plots.

    :param df: Public data (see above)
    :param df_seq_sorted: Private data (see above)
    :param output: Output file name
    :return:
    """

    plt.rcParams.update({
        'text.usetex': False,
        "svg.fonttype": 'none',
        'axes.spines.right': False,
        'axes.spines.top': False
    })
    plt.figure(figsize=(5,5))
    plt.bar([int(x) for x in df.index], df["Sequence[bp]"]/1000, edgecolor="black")
    plt.bar([int(x) for x in df.index][-1], df["Sequence[bp]"].values[-1]/1000, edgecolor="black")
    bottom1 = 0
    for x in df_seq_sorted.iterrows():
        plt.bar(1, (x[1]["Sequence[bp]"])/1000, color="yellow", bottom=bottom1, edgecolor="black")
        bottom1 += int(x[1]["Sequence[bp]"])/1000
    plt.ylabel("Sequence [kbp]")
    plt.xlabel("Similarity")
    plt.savefig(output + ".sequence.core.pdf")
    plt.close()

    # Plotting the node data
    plt.figure(figsize=(5,5))
    plt.bar([int(x) for x in df.index], df["#Node"]/1000, edgecolor="black")
    plt.bar([int(x) for x in df.index][-1], df["#Node"].values[-1]/1000, edgecolor="black")
    bottom1 = 0
    for x in df_seq_sorted.iterrows():
        plt.bar(1, (x[1]["#Node"])/1000, color="yellow", bottom=bottom1, edgecolor="black")
        bottom1 += int(x[1]["#Node"])/1000
    plt.ylabel("#Node [x1000]")
    plt.xlabel("Similarity")
    plt.savefig(output + ".node.core.pdf")
    plt.close()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Core: Process CSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PNG file', required=True)
    args = parser.parse_args()

    # Load the CSV file
    data_priv, data_pub = read_data(args.input)
    plot_test(data_pub, data_priv, args.output)