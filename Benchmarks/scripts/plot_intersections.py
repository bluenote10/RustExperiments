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

from sklearn import tree
from sklearn.linear_model import LinearRegression

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

            ax_all[0].legend(loc="best")
            ax_all[1].legend(loc="best")
            ax_dists[0].legend(loc="best")
            ax_dists[1].legend(loc="best")
            ax_all[0].set_title("Delta (values) to exact solution")
            ax_all[1].set_title("Delta (ULPs) to exact solution")

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


def two_diff_err(a, b):
    x = a - b
    b_virt = a - x
    a_virt = x + b_virt
    b_roundoff = b_virt - b
    a_roundoff = a_virt - a
    y = a_roundoff + b_roundoff
    return y


def estimate_cross_product_error(df, ax_name, ay_name, bx_name, by_name):
    ax = df[ax_name]
    ay = df[ay_name]
    bx = df[bx_name]
    by = df[by_name]
    ax_err = np.abs(df["{}_err".format(ax_name)])
    ay_err = np.abs(df["{}_err".format(ay_name)])
    bx_err = np.abs(df["{}_err".format(bx_name)])
    by_err = np.abs(df["{}_err".format(by_name)])
    err = (
            ax_err * np.abs(by) +
            np.abs(ax) * by_err +
            ay_err * np.abs(bx) +
            np.abs(ay) * bx_err
    )
    val = ax * by - ay * bx
    return err / np.abs(val)


def estimate_uncertainty(df):
    a1_x = df["a1_x"]
    a1_y = df["a1_y"]
    a2_x = df["a2_x"]
    a2_y = df["a2_y"]
    b1_x = df["b1_x"]
    b1_y = df["b1_y"]
    b2_x = df["b2_x"]
    b2_y = df["b2_y"]

    va_x = Interval.from_sub(a2_x, a1_x)
    va_y = Interval.from_sub(a2_y, a1_y)
    vb_x = Interval.from_sub(b2_x, b1_x)
    vb_y = Interval.from_sub(b2_y, b1_y)
    e_y = Interval.from_sub(b1_y, a1_y)
    e_x = Interval.from_sub(b1_x, a1_x)

    cross_va_vb = va_x * vb_y - va_y * vb_x
    cross_e_vb = e_x * vb_y - e_y * vb_x

    s = cross_e_vb / cross_va_vb

    result_x = Interval.from_float(a1_x) + s * va_x
    result_y = Interval.from_float(a1_y) + s * va_y

    err = result_x.err + result_y.err
    df["uncertainty"] = err


class Interval(object):
    def __init__(self, lo, hi):
        assert (lo <= hi).all()
        self.lo = lo
        self.hi = hi

    @property
    def mid(self):
        return (self.hi + self.lo) / 2.0

    @property
    def err(self):
        return self.hi - self.lo

    def __add__(self, that):
        return self.operation(self, that, np.add)

    def __sub__(self, that):
        return self.operation(self, that, np.subtract)

    def __mul__(self, that):
        return self.operation(self, that, np.multiply)

    def __div__(self, that):
        return self.operation(self, that, np.divide)

    def __repr__(self):
        return "Interval({}, {})".format(self.lo, self.hi)

    @staticmethod
    def operation(a, b, op):
        c1 = op(a.lo, b.lo)
        c2 = op(a.lo, b.hi)
        c3 = op(a.hi, b.lo)
        c4 = op(a.hi, b.hi)
        hi = np.max([c1, c2, c3, c4], axis=0)
        lo = np.min([c1, c2, c3, c4], axis=0)
        hi = np.nextafter(hi, +np.inf)
        lo = np.nextafter(lo, -np.inf)
        return Interval(lo, hi)

    @staticmethod
    def from_float(x):
        return Interval(x, x)

    @staticmethod
    def from_sub(a, b):
        x = a - b
        y = two_diff_err(a, b)
        lo = x.copy()
        hi = x.copy()
        lo[y < 0] = np.nextafter(x, -np.inf)
        hi[y > 0] = np.nextafter(x, +np.inf)
        return Interval(lo, hi)


def plot_correlations(df):

    df["a1_x"] = df["a1"].apply(lambda x: x[0])
    df["a1_y"] = df["a1"].apply(lambda x: x[1])
    df["a2_x"] = df["a2"].apply(lambda x: x[0])
    df["a2_y"] = df["a2"].apply(lambda x: x[1])
    df["b1_x"] = df["b1"].apply(lambda x: x[0])
    df["b1_y"] = df["b1"].apply(lambda x: x[1])
    df["b2_x"] = df["b2"].apply(lambda x: x[0])
    df["b2_y"] = df["b2"].apply(lambda x: x[1])

    non_train_columns = set(df.columns)

    df["va_x"] = df["a2_x"] - df["a1_x"]
    df["va_y"] = df["a2_y"] - df["a1_y"]
    df["vb_x"] = df["b2_x"] - df["b1_x"]
    df["vb_y"] = df["b2_y"] - df["b1_y"]
    df["e_x"] = df["b1_x"] - df["a1_x"]
    df["e_y"] = df["b1_y"] - df["a1_y"]

    df["va_x_err"] = two_diff_err(df["a2_x"], df["a1_x"])
    df["va_y_err"] = two_diff_err(df["a2_y"], df["a1_y"])
    df["vb_x_err"] = two_diff_err(df["b2_x"], df["b1_x"])
    df["vb_y_err"] = two_diff_err(df["b2_y"], df["b1_y"])
    df["e_x_err"] = two_diff_err(df["b1_x"], df["a1_x"])
    df["e_y_err"] = two_diff_err(df["b1_y"], df["a1_y"])
    df["va_x_err_rel"] = df["va_x_err"] / df["va_x"]
    df["va_y_err_rel"] = df["va_y_err"] / df["va_y"]
    df["vb_x_err_rel"] = df["vb_x_err"] / df["vb_x"]
    df["vb_y_err_rel"] = df["vb_y_err"] / df["vb_y"]
    df["e_x_err_rel"] = df["e_x_err"] / df["e_x"]
    df["e_y_err_rel"] = df["e_y_err"] / df["e_y"]

    for c in df:
        if c.endswith("_err_rel"):
            df[c + "_abs"] = np.abs(df[c])

    df["err_va_vb"] = estimate_cross_product_error(df, "va_x", "va_y", "vb_x", "vb_y")
    df["err_e_va"] = estimate_cross_product_error(df, "e_x", "e_y", "va_x", "va_y")
    df["err_e_vb"] = estimate_cross_product_error(df, "e_x", "e_y", "vb_x", "vb_y")

    estimate_uncertainty(df)

    df["va_ratio"] = df["va_x"] / df["va_y"]
    df["vb_ratio"] = df["vb_x"] / df["vb_y"]
    df["e_ratio"] = df["e_x"] / df["e_y"]
    df["va_vb_ratio"] = df["va_ratio"] / df["vb_ratio"]

    df["len_va"] = np.sqrt(df["va_x"] ** 2 + df["va_y"] ** 2)
    df["len_vb"] = np.sqrt(df["vb_x"] ** 2 + df["vb_y"] ** 2)
    df["len_e"] = np.sqrt(df["e_x"] ** 2 + df["e_y"] ** 2)

    df["diff_len_va_vb"] = np.abs(df["len_va"] - df["len_vb"])
    df["ratio_len_va_vb"] = df["len_va"] / df["len_vb"]

    df["angle_va_vb"] = (np.arccos(
        ((df["va_x"] * df["vb_x"]) + (df["va_y"] * df["vb_y"])) / (df["len_va"] * df["len_vb"])
    ) + np.pi) % (2 * np.pi) - np.pi
    df["angle_e_va"] = (np.arccos(
        ((df["e_x"] * df["va_x"]) + (df["e_y"] * df["va_y"])) / (df["len_e"] * df["len_va"])
    ) + np.pi) % (2 * np.pi) - np.pi
    df["angle_e_vb"] = (np.arccos(
        ((df["e_x"] * df["vb_x"]) + (df["e_y"] * df["vb_y"])) / (df["len_e"] * df["len_vb"])
    ) + np.pi) % (2 * np.pi) - np.pi

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

    feature_names = [c for c in df.columns if c not in non_train_columns]
    #feature_names = [c for c in df.columns if c not in non_train_columns and "err" in c]
    X = np.transpose([df[feature].values for feature in feature_names])

    Y = np.zeros(len(df))
    Y[df["delta_fast1"] > 0] = 1
    Y[df["delta_fast1"] > 1e-14] = 2
    Y[df["delta_fast1"] > 1e-13] = 3
    Y[df["delta_fast1"] > 1e-12] = 4
    Y[df["delta_fast1"] > 1e-11] = 5
    Y[df["delta_fast1"] > 1e-10] = 6

    def fit_lin_reg_features(features):
        feature_indices = [feature_names.index(feature) for feature in features]
        X_tmp = X[:, feature_indices]
        reg = LinearRegression().fit(X_tmp, Y)
        score = reg.score(X_tmp, Y)
        return score

    results = []
    for i in range(len(feature_names) - 1):
        for j in range(i + 1, len(feature_names)):
            features = [feature_names[i], feature_names[j]]
            score = fit_lin_reg_features(features)
            results.append((score, features))

    for score, features in list(reversed(sorted(results)))[:20]:
        print("{} {}".format(score, features))

        fig, ax = plt.subplots(1, 1, figsize=(16, 8))
        p = ax.scatter(df[features[0]], df[features[1]], c=Y)   # c=np.log(df["delta_fast1"]
        plt.colorbar(p)
        ax.set_xlabel(features[0])
        ax.set_ylabel(features[1])
        plt.show()

    import IPython; IPython.embed()
    fig, ax = plt.subplots(1, 1, figsize=(16, 8))
    p = ax.scatter(df["angle_va_vb"], df["va_y"], c=Y)   # c=np.log(df["delta_fast1"]
    plt.colorbar(p)
    plt.show()

    # Y = df["delta_fast1"] / df["delta_fast1"].max()
    Y = df["delta_fast1"] > 1e-12
    #clf = tree.DecisionTreeRegressor(max_depth=3, min_samples_leaf=3)
    clf = tree.DecisionTreeClassifier(max_depth=3, min_samples_leaf=3)
    clf = clf.fit(X, Y)
    #tree.plot_tree(clf)
    tree.export_graphviz(
        clf, "tree.dot",
        feature_names=feature_names, filled=True, rounded=True, special_characters=True,
    )

    import IPython; IPython.embed()


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

    # plot_correlations(df)

    plot_distributions(data)

    # sort df by delta
    df = df.sort_values("delta_fast1", ascending=False).reset_index(drop=True)
    for _, row in df.iterrows():
        plot_intersection(row)


if __name__ == "__main__":
    main()
