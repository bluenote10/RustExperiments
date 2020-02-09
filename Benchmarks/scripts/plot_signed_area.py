#!/usr/bin/env python

from __future__ import print_function

import pandas as pd

import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
from matplotlib.patches import PathPatch
from matplotlib.path import Path

import argparse
import json
import os


def plot(ax, df, col_a, col_b):
    ax.plot(df[col_a], df[col_b], "o", ms=2)
    ax.set_xlabel(col_a)
    ax.set_ylabel(col_b)


def parse_args():
    parser = argparse.ArgumentParser("Tool to plot signed area differences.")
    """
    parser.add_argument(
        "-i", "--interactive",
        action="store_true",
        help="whether to show interactive plot windows"
    )
    """
    parser.add_argument(
        "file",
        help="data file",
    )
    args = parser.parse_args()
    return args


def main():
    args = parse_args()
    filename = args.file
    interactive = True # args.interactive

    data = json.load(open(filename))
    df = pd.DataFrame(data)

    df["rel_err_robust"] = df["sa_robust"] / df["sa_exact"]
    df["rel_err_fast"] = df["sa_fast"] / df["sa_exact"]

    fig, axes = plt.subplots(1, 3, figsize=(15, 7), sharex=True, sharey=True)
    plot(axes[0], df, "sa_exact", "sa_fast")
    plot(axes[1], df, "sa_exact", "sa_robust")
    plot(axes[2], df, "sa_robust", "sa_fast")

    fig, axes = plt.subplots(1, 2, figsize=(15, 7))
    axes[0].plot(df["sa_exact"], df["rel_err_robust"], "o", ms=2)
    axes[1].plot(df["sa_exact"], df["rel_err_fast"], "o", ms=2)

    plt.tight_layout()
    plt.subplots_adjust(top=0.90)

    if interactive:
        plt.show()

    plt.close(fig)


if __name__ == "__main__":
    main()
