# Imports
import pandas as pd
import matplotlib.pyplot as plt
import sys
import argparse


def read_data(filename):
    if filename == "-": # Read from stdin
        df = pd.read_csv(sys.stdin, sep = "\t", header = True)
        return df
    else:
        df = pd.read_csv(filename, sep = "\t", header = True)
        return df

def check_data(df_list):
    a = df_list[0].index
    b = df_list[0].columns
    for x in df_list[1:]:
        if x.index != a or x.columns != b:
            return False

def get_cols(df_list):
    cols = list(df_list.columns)

def collect_col_val(df_list, cols):
    col_vals = []
    for x in df_list:
        col_vals.append(x[cols])
    return col_vals

def plot_histogram(col_vals, cols, output):
    for i, x in enumerate(col_vals):
        plt.hist(x, bins = 50, alpha = 0.5, label = cols[i])
        plt.savefig(output + "." + cols[i] + ".hist.pdf")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Histogram: Process TSV input and PDF output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input TSV file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output PDF file', required=True)
    args = parser.parse_args()

    df = read_data(args.input)
    cols = get_cols(df)
    col_vals = collect_col_val(df, cols)
    plot_histogram(col_vals, cols, args.output)