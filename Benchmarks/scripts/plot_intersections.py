#!/usr/bin/env python

from __future__ import print_function

import pandas as pd

import matplotlib.pyplot as plt
import matplotlib
from matplotlib.collections import PatchCollection
from matplotlib.backends.backend_pdf import PdfPages
from matplotlib.patches import PathPatch
from matplotlib.path import Path

import argparse
import json
import math
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


point_names = [
    "exact",
    "min",
    "fast",
    # "search",
]


def plot_intersection_grid(ax, row):
    grid = row["grid"]
    grid_df = pd.DataFrame(grid)

    circles = [
        plt.Rectangle((xi - 0.5, yi - 0.5), 1, 1, linewidth=0)
        for xi, yi in zip(grid_df["i"], grid_df["j"])
    ]
    c = PatchCollection(circles, cmap=matplotlib.cm.jet)
    c.set_array(grid_df["dist"])
    p = ax.add_collection(c)
    # plt.colorbar(p)
    ax.autoscale_view()

    for point_name in point_names:
        ax.plot(
            row["i_{}".format(point_name)]["ulp_dist"][0],
            row["i_{}".format(point_name)]["ulp_dist"][1],
            "o", label="{}".format(point_name),
        )

    def format_coord(x, y):
        i = int(math.floor(x + 0.5))
        j = int(math.floor(y + 0.5))
        grid_row = [row for row in grid if row["i"] == i and row["j"] == j]
        if grid_row:
            return '{} i={} j={} '.format(json.dumps(grid_row[0]["dist"]), i, j)
        else:
            return 'i={} j={} '.format(i, j)

    ax.format_coord = format_coord


def plot_intersection_points(ax, row):
    for point_name in point_names:
        ax.plot(
            row["i_{}".format(point_name)]["p"][0],
            row["i_{}".format(point_name)]["p"][1],
            "o", ms=4, label="{}".format(point_name),
        )

    for point_name in ["a1", "a2", "b1", "b2"]:
        ax.plot(
            row[point_name][0],
            row[point_name][1],
            "o", ms=4, label="{}".format(point_name),
        )

    ax.plot(
        [row["a1"][0], row["a2"][0]],
        [row["a1"][1], row["a2"][1]],
        "-", c="#333333", alpha=0.5,
    )
    ax.plot(
        [row["b1"][0], row["b2"][0]],
        [row["b1"][1], row["b2"][1]],
        "-", c="#333333", alpha=0.5,
    )


def plot_intersection(row):
    fig, axes = plt.subplots(1, 2, figsize=(16, 8))

    plot_intersection_grid(axes[0], row)
    plot_intersection_points(axes[1], row)

    plt.legend()
    plt.tight_layout()
    plt.subplots_adjust(top=0.90)

    plt.show()


def plot_distributions(data):
    names = [n for n in point_names if n != "exact"]
    fig, axes = plt.subplots(1, len(names), figsize=(16, 8))
    for ax, name in zip(axes, names):
        dists = pd.DataFrame(
            [row["i_{}".format(name)]["ulp_dist"] for row in data],
            columns=["x", "y"],
        )
        ax.plot(dists["x"], dists["y"], "o", ms=2, alpha=0.5)
        ax.set_title(name)

    plt.tight_layout()
    plt.subplots_adjust(top=0.90)
    plt.show()


def main():
    args = parse_args()
    filename = args.file
    interactive = True # args.interactive

    data = json.load(open(filename))
    df = pd.DataFrame(data)

    plot_distributions(data)

    for _, row in df.iterrows():
        plot_intersection(row)

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
