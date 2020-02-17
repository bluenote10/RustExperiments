#!/usr/bin/env python

from __future__ import print_function

import numpy as np
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
    "fast1",
    "fast2",
    "soe",
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
    plt.legend()


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
    print(json.dumps(
        {k: v for k, v in row.to_dict().items() if k != "grid" and not k.startswith("i_")},
        indent=2, sort_keys=True,
    ))
    fig, axes = plt.subplots(1, 2, figsize=(16, 8))

    plot_intersection_grid(axes[0], row)
    plot_intersection_points(axes[1], row)

    plt.legend()
    plt.tight_layout()
    plt.subplots_adjust(top=0.90)

    plt.show()


def plot_distributions(data):
    points_exact = pd.DataFrame(
        [row["i_exact"]["p"] for row in data],
        columns=["x", "y"],
    )

    names = [n for n in point_names if n != "exact"]
    fig1, axes = plt.subplots(1, len(names), figsize=(16, 8))
    fig2, ax_all = plt.subplots(1, 2, figsize=(16, 8))
    fig3, ax_dists = plt.subplots(1, 2, figsize=(16, 8))

    for ax, name in zip(axes, names):
        points = pd.DataFrame(
            [row["i_{}".format(name)]["p"] for row in data],
            columns=["x", "y"],
        )
        deltas = points - points_exact
        ulp_deltas = pd.DataFrame(
            [row["i_{}".format(name)]["ulp_dist"] for row in data],
            columns=["x", "y"],
        )
        ax.plot(ulp_deltas["x"], ulp_deltas["y"], "o", ms=2, alpha=0.5)
        ax.set_title(name)

        if name not in ["exact", "min"]:
            ax_all[0].plot(deltas["x"], deltas["y"], "o", ms=4, alpha=0.5, label=name)
            ax_all[1].plot(ulp_deltas["x"], ulp_deltas["y"], "o", ms=4, alpha=0.5, label=name)

            deltas_err = np.sqrt(deltas["x"]**2 + deltas["y"]**2)
            ulp_deltas_err = np.sqrt(ulp_deltas["x"] ** 2 + ulp_deltas["y"] ** 2)

            deltas_err = sorted(deltas_err)
            ulp_deltas_err = sorted(ulp_deltas_err)

            ax_dists[0].plot(np.arange(len(deltas_err)), deltas_err, "-", label=name)
            ax_dists[1].plot(np.arange(len(deltas_err)), ulp_deltas_err, "-", label=name)

        print(name)
        print("Deltas:     {}    {}    {}".format(
            deltas["x"].abs().mean(),
            deltas["y"].abs().mean(),
            (deltas["x"].abs() + deltas["y"].abs()).mean()
        ))
        print("ULPs:       {}    {}    {}".format(
            ulp_deltas["x"].abs().mean(),
            ulp_deltas["y"].abs().mean(),
            (ulp_deltas["x"].abs() + ulp_deltas["y"].abs()).mean(),
        ))

    ax_dists[0].set_yscale('log')
    ax_dists[1].set_yscale('log')

    plt.legend()
    plt.tight_layout()
    plt.subplots_adjust(top=0.90)
    plt.show()


def plot_correlations(df):
    df["a1_x"] = df["a1"].apply(lambda x: x[0])
    df["a1_y"] = df["a1"].apply(lambda x: x[1])
    df["a2_x"] = df["a2"].apply(lambda x: x[0])
    df["a2_y"] = df["a2"].apply(lambda x: x[1])
    df["b1_x"] = df["b1"].apply(lambda x: x[0])
    df["b1_y"] = df["b1"].apply(lambda x: x[1])
    df["b2_x"] = df["b2"].apply(lambda x: x[0])
    df["b2_y"] = df["b2"].apply(lambda x: x[1])
    df["va_x"] = df["a2_x"] - df["a1_x"]
    df["va_y"] = df["a2_y"] - df["a1_y"]
    df["vb_x"] = df["b2_x"] - df["b1_x"]
    df["vb_y"] = df["b2_y"] - df["b1_y"]
    df["e_x"] = df["b1_x"] - df["a1_x"]
    df["e_y"] = df["b1_y"] - df["a1_y"]
    import IPython; IPython.embed()

    df["len_va"] = np.sqrt(df["va_x"] ** 2 + df["va_y"] ** 2)
    df["len_vb"] = np.sqrt(df["vb_x"] ** 2 + df["vb_y"] ** 2)
    df["len_e"] = np.sqrt(df["e_x"] ** 2 + df["e_y"] ** 2)

    df["diff_len_va_vb"] = np.abs(df["len_va"] - df["len_vb"])
    df["ratio_len_va_vb"] = df["len_va"] / df["len_vb"]

    df["angle_va_vb"] = np.arccos(
        ((df["va_x"] * df["vb_x"]) + (df["va_y"] * df["vb_y"])) / (df["len_va"] * df["len_vb"])
    )
    df["angle_e_va"] = np.arccos(
        ((df["e_x"] * df["va_x"]) + (df["e_y"] * df["va_y"])) / (df["len_e"] * df["len_va"])
    )
    df["angle_e_vb"] = np.arccos(
        ((df["e_x"] * df["vb_x"]) + (df["e_y"] * df["vb_y"])) / (df["len_e"] * df["len_vb"])
    )

    df["va_x_vb_y"] = df["va_x"] * df["vb_y"]
    df["va_y_vb_x"] = df["va_y"] * df["vb_x"]
    df["diff_va_x_vb_y_va_y_vb_x"] = df["va_x_vb_y"] - df["va_y_vb_x"]
    df["ratio_va_x_vb_y_va_y_vb_x"] = (
        np.abs(df["diff_va_x_vb_y_va_y_vb_x"]) / np.max([np.abs(df["va_x_vb_y"]), np.abs(df["va_y_vb_x"])], axis=0)
    )

    df["e_x_vb_y"] = df["e_x"] * df["vb_y"]
    df["e_y_vb_x"] = df["e_y"] * df["vb_x"]
    df["diff_e_x_vb_y_e_y_vb_x"] = df["e_x_vb_y"] - df["e_y_vb_x"]
    df["ratio_e_x_vb_y_e_y_vb_x"] = (
        np.abs(df["diff_e_x_vb_y_e_y_vb_x"]) / np.max([np.abs(df["e_x_vb_y"]), np.abs(df["e_y_vb_x"])], axis=0)
    )

    df["e_x_va_y"] = df["e_x"] * df["va_y"]
    df["e_y_va_x"] = df["e_y"] * df["va_x"]
    df["diff_e_x_va_y_e_y_va_x"] = df["e_x_va_y"] - df["e_y_va_x"]
    df["ratio_e_x_va_y_e_y_va_x"] = (
        np.abs(df["diff_e_x_va_y_e_y_va_x"]) / np.max([np.abs(df["e_x_va_y"]), np.abs(df["e_y_va_x"])], axis=0)
    )

    df["all_ratios"] = df["ratio_va_x_vb_y_va_y_vb_x"] * df["ratio_e_x_vb_y_e_y_vb_x"] * df["ratio_e_x_va_y_e_y_va_x"]

    fig, ax = plt.subplots(1, 1, figsize=(16, 8))
    #ax.scatter(df["angle_va_vb"], df["angle_e_vb"], c=df["delta_fast1"])
    ax.scatter(df["ratio_va_x_vb_y_va_y_vb_x"], df["ratio_e_x_vb_y_e_y_vb_x"], c=df["delta_fast1"])
    plt.show()


def add_delta_col(df, name):
    df["delta_{}".format(name)] = (
        df["i_{}".format(name)].apply(lambda row: np.array(row["p"])) -
        df["i_exact"].apply(lambda row: np.array(row["p"]))
    ).apply(lambda row: np.sqrt((row ** 2).mean()))


def main():
    args = parse_args()
    filename = args.file

    data = json.load(open(filename))
    df = pd.DataFrame(data)
    add_delta_col(df, "soe")
    add_delta_col(df, "fast1")

    plot_correlations(df)
    plot_distributions(data)

    # sort df by delta
    df = df.sort_values("delta_fast1", ascending=False).reset_index(drop=True)
    for _, row in df.iterrows():
        plot_intersection(row)


if __name__ == "__main__":
    main()
