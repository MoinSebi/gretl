# Imports
import seaborn as sns
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib as mpl
import sys
import argparse

def read_data(filename):
    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t")
        return df
    else:
        df = pd.read_csv(filename, sep = "\t")
        df.index = df["Path"]
        df.index = [x.split(".")[0] for x in df.index]
        return df

def plotter(df, output):
    plt.figure(figsize = (5,4))
    plt.scatter(data=df, x="Edges", y="Jumps total", edgecolor = "black", color = "royalblue")
    for x in df.iterrows():
        plt.annotate(x[0], (x[1]["Edges"] + 0.0005, x[1]["Jumps total"]))
    plt.xlabel("Edges")
    plt.ylabel("Jumps total")
    plt.tight_layout()
    plt.savefig(output + "path.scatter.pdf")

# Plot the example using "Egdes" and "Jumpts total"
    plt.figure(figsize = (5,4))
    plt.scatter(data=df, x="Edges", y="Depth average", edgecolor = "black", color = "royalblue")
    for x in df.iterrows():
        plt.annotate(x[0], (x[1]["Edges"] + 0.0005, x[1]["Depth average"]))
    plt.xlabel("Edges")
    plt.ylabel("Depth average")
    plt.tight_layout()
    plt.savefig(output + "path.scatter.pdf")


    df
