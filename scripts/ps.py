#!/usr/bin/env python3

#Import
import pandas as pd
import matplotlib.pyplot as plt
import sys
import argparse

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
    node_core = df.apply(lambda x: [x for x in x.iloc[3:]][-1], axis = 1)
    node_private = df.apply(lambda x: x.iloc[3], axis = 1)
    df2 = pd.DataFrame([node_core, node_soft, node_private]).T
    df2.columns = ["Core", "Soft", "Private"]
    df2.index = df["Accession"]
    (df2).plot(kind = "barh", stacked = True, figsize = (6,5), width = 0.75, legend = True,
                    edgecolor = "black",
                    linewidth = 0.5,
                    color = ["royalblue", "tomato", "gold"])
    plt.xlabel(label1)
    plt.ylabel("Accession")
    plt.tight_layout()
    plt.savefig(outfile + ".ps." + what + ".pdf")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Path: Process TSV input and PDF output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PDF file', required=True)
    args = parser.parse_args()

    data = read_data(args.input)
    df_node = table_sep(data, "N")
    df_seq = table_sep(data, "S")
    plot_node_stats(df_node, args.output, "#Nodes", "N")
    plot_node_stats(df_seq, args.output, "#Sequence [bp]", "S")
