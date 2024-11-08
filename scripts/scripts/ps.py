#Import
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np
import sys
import argparse


def read_data(filename):
    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t")
        df.set_index(0, inplace=True)

        return df
    else:
        df = pd.read_csv(filename, sep = "\t")
        df.set_index(0, inplace=True)

        return df

def index_df(df):
    # Sequence starts here
    seq_index = [i for i, x in enumerate(list(df.columns)) if x.startswith("Seq")][0]

    # Data starts here
    start_index = [i for i, x in enumerate(list(df.columns)) if x.startswith("Node")][0]+1

    return seq_index, start_index

def sub_df(df, seq_index, start_index):

# Node

# Node stats
    node_soft = df.apply(lambda x: sum([x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][1:-1]), axis = 1)
    node_core = df.apply(lambda x: [x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][-1], axis = 1)
    node_priv = df.apply(lambda x: [x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][0], axis = 1)
    node_pan = df.apply(lambda x: sum([x for x in x.iloc[start_index:seq_index].values if not np.isnan(x)][:]), axis = 1)

    # Create a DataFrame
    df2 = pd.DataFrame([node_core, node_soft, node_priv]).T

    # Change index and column name
    df2.columns = ["Core", "Soft", "Private"]
    df2.index = [x.split("_")[0] for x in df["Accession"]]
    df2.head()

# Update the index (remove the haplotype and contig/scaffold name)
    df2.index = [x.split("#")[0] for x in df2.index]

    return df2

def plot_node_stats(df2, outfile):
#Plot the nodes
    (df2/1000).plot(kind = "barh", stacked = True, figsize = (6,5), width = 0.75, legend = True,
                    edgecolor = "black",
                    linewidth = 0.5,
                    color = ["royalblue", "tomato", "gold"])
    plt.xlabel("#Nodes [x1000]")
    plt.ylabel("Accession")
    plt.tight_layout()
    #plt.savefig("plots/ps.similarity_path.400.png", dpi = 400)
#plt.savefig("plots/ps.similarity_path.pdf")


# Sequence
def sub_df2(df, seq_index):
# Do the same with sequence
    seq_soft = df.apply(lambda x: sum([x for x in x.iloc[seq_index:].values if not np.isnan(x)][2:-1]), axis = 1)
    seq_core = df.apply(lambda x: [x for x in x.iloc[seq_index:].values if not np.isnan(x)][-1], axis = 1)
    seq_priv = df.apply(lambda x: [x for x in x.iloc[seq_index:].values if not np.isnan(x)][1], axis = 1)

    # Read in DataFrame
    df2 = pd.DataFrame([seq_core, seq_soft, seq_priv]).T
    df2.head()

    # Change index and column name
    df2.columns = ["Core", "Soft", "Private"]
    df2.index = [x.split("_")[0] for x in df["Accession"]]
    df2.head()

    # Update index (s. above)
    df2.index = [x.split("#")[0] for x in df2.index]

def plot_seq_stats(df2, outfile):
    # Plot figure
    # Divide by 1000 -> kbp
    (df2/1000).plot(kind = "barh", stacked = True, figsize = (6,5), width = 0.75, legend = True,
                    edgecolor = "black",
                    linewidth = 0.5,
                    color = ["royalblue", "tomato", "gold"])
    plt.xlabel("Sequence [kbp]")
    plt.ylabel("Accession")
    plt.tight_layout()
    plt.savefig(outfile + ".bootstrap.seq.pdf")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Path: Process TSV input and PDF output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PDF file', required=True)
    args = parser.parse_args()

    df = read_data(args.input)
    seq_index, start_index = index_df(df)
    df2 = sub_df(df, seq_index, start_index)
    plot_node_stats(df2, args.output)
    df2 = sub_df2(df, seq_index)
    plot_seq_stats(df2, args.output)
