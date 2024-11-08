# Imports
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np
import argparse
import sys


def read_gfa(graph_file):
    # Read the nodes of the GFA file
    # dict(nodeID -> len)
    node2len = dict()
    with open(graph_file) as file:
        for line in file.readlines():
            if line.startswith("S"):
                ls = line.split()
                node2len[int(ls[1])] = len(ls[2])

    # Check
    node2len

def read_nwindow_tsv(filename):
    # Read the nwindow.md output in as DataFrame
    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t")
    else:
        df = pd.read_csv(filename, sep = "\t")
    return df

def get_poslist(df, node2len):
    current_pos = 0
    pos_list = []
    for x in df.index:
        pos_list.append(current_pos + node2len[x+1])
        current_pos += node2len[x+1]
    return pos_list

def plotter1(df, pos_list, output, ylabel, xlabel, df_col, correction = True):
    # Plot #Nodes
    plt.figure(figsize = (8,3))
    cor1 = "correction"
    if correction:
        x = np.array(pos_list)/1000
    else:
        cor1 = "no_correction"
        x = df["nodeid"]


    plt.scatter(x, df[df_col], color = "black", s = 2)
    plt.ylabel(ylabel)
    plt.xlabel(xlabel)
    plt.tight_layout()
    plt.savefig(output + "." + df_col + "." + cor1 + ".pdf")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Bootstrap: Process CSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-g', '--graph', type=str, help='Graph', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PNG file', required=True)
    args = parser.parse_args()

    node2len = read_gfa(args.graph)
    df = read_nwindow_tsv(args.input)
    pos_list = get_poslist(df, node2len)
    plotter1(df, pos_list, args.output, "#Nodes (in one window.md)", "Node ID (length corrected) [x10³]", "node")
    plotter1(df, pos_list, args.output, "Sequence (in one window.md) [kbp]", "Node ID (length corrected) [x10³]", "sequence")
    plotter1(df, pos_list, args.output, "Sum of Jumps (in one window.md) [x10⁶]", "Node ID (length corrected) [x10³]", "jumps")
    plotter1(df, pos_list, args.output, "#Nodes (in one window.md)", "Node ID (length corrected) [x10³]", "node", correction=False)
    plotter1(df, pos_list, args.output, "Sequence (in one window.md) [kbp]", "Node ID (length corrected) [x10³]", "sequence", correction=False)
    plotter1(df, pos_list, args.output, "Sum of Jumps (in one window.md) [x10⁶]", "Node ID (length corrected) [x10³]", "jumps", correction=False)





