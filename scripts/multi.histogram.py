#!/usr/bin/env python3

import argparse
import pandas as pd
import matplotlib.pyplot as plt
import os
import logging
import sys

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s    [%(levelname)s] - %(filename)s: %(message)s',
    datefmt='%d/%m/%Y %H:%M:%S',  # 24-hour format
    handlers=[logging.StreamHandler(stream=sys.stderr)]
)

def read_stats(filename: str) -> list:
    """
    Parse the input file and return a list of files to read.

    :param filename: File with the absolute path of the graphs to read
    :return: A list of files to read
    """
    files = []
    with open(filename, "r") as f:
        for line in f.readlines():
            files.append(line.split())
    return files

def read_data(files: list) -> pd.DataFrame:
    """
    Read all the files in the list and return a combined dataframe.

    :param files: List of all files to read
    :return: A combined dataframe of all the files
    """
    combined = pd.read_csv(files[0][1], sep = "\t")

    for x in files[1:]:
        df = pd.read_csv(x[1], sep = "\t")
        combined = pd.concat([combined, df], ignore_index= True, axis = 0)
    combined.index = [x[0] for x in files]
    return combined

def plot_histo(df: pd.DataFrame, output: str) -> None :
    """
    Plot a histogram for each column in the dataframe

    :param df: Multiple graphs statistics in pandas Dataframe
    :param output: Output file name prefix
    :return:
    """
    for x in df.columns:
        fileout = output + "multi.histogram." + "".join(x.split()).replace("/", "") + ".histo.pdf"
        plt.figure(figsize = (5,4))
        plt.hist(df[x], edgecolor = "black", color = "royalblue")
        plt.xlabel(x)
        plt.ylabel("Frequency")
        plt.tight_layout()
        plt.savefig(fileout)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Multi: Process TSV input and PNG output.")
    parser.add_argument('-i', '--input', type=str, help='Path to the input file', required=True)
    parser.add_argument('-o', '--output', type=str, help='Path to the output file', required=True)
    args = parser.parse_args()


    logging.info("Reading data from %s", args.input)
    files = read_stats(args.input)

    logging.info("Really read the data (+combine)")
    df = read_data(files)


    # We create a new folder since it is going to be a lot of new plots
    os.makedirs(args.output, exist_ok=True)

    logging.info("Plotting histograms")
    plot_histo(df, args.output + "/" )

