#!/usr/bin/env python3

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import sys
import logging

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s    [%(levelname)s] - %(filename)s: %(message)s',
    datefmt='%d/%m/%Y %H:%M:%S',  # 24-hour format
    handlers=[logging.StreamHandler(stream=sys.stderr)]
)

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


def table_sep(df: pd.DataFrame, what: str) -> pd.DataFrame:

    df = df.loc[df["Type"] == what]
    return df

def plot_node_stats(df: pd.DataFrame, outfile: str, label1: str, what: str) -> None:
    """
    Plot the saturation/bootstrapping statistics for the nodes (scatterplot)

    :param df: Gretl bootstrap output in pandas DataFrame
    :param outfile: Output file name
    :return: Plot (PDF)
    """

    collen = len(df.columns)

    node_soft = df.apply(lambda x: sum([x for x in x.iloc[3:]][1:-1]), axis = 1)
    node_core = df.apply(lambda x: x.iloc[2+x["Size"]], axis = 1)
    node_private = df.apply(lambda x: x.iloc[3], axis = 1)
    node_pan = df.apply(lambda x: sum(x.iloc[3:]), axis = 1)
    # Plot figure (nodes)
    plt.figure(figsize = (6,4))
    plt.scatter(df["Size"], node_private, edgecolor = "black", label = "Private")
    plt.scatter(df["Size"], node_soft, edgecolor = "black", label = "Soft")
    plt.scatter(df["Size"],  node_core, edgecolor = "black", label = "Core")
    plt.scatter(df["Size"],  node_pan, edgecolor = "black", label = "Pan")
    plt.xlabel("Number of genomes")
    plt.ylabel(label1)
    plt.legend()
    plt.tight_layout()
    plt.savefig(outfile + ".bootstrap." + what + ".pdf")




if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Bootstrap: Process CSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PNG file', required=True)
    args = parser.parse_args()



    # Load the CSV file
    logging.info("Reading data from %s", args.input)
    data = read_data(args.input)

    # Separate the data
    logging.info("Separate tables")
    df_node = table_sep(data, "N")
    df_seq = table_sep(data, "S")

    # Plot the data
    logging.info("Plotting data")
    plot_node_stats(df_node, args.output, "#Nodes", "N")
    plot_node_stats(df_seq, args.output, "#Sequence [bp]", "S")
