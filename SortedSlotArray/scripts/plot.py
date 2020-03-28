#!/usr/bin/env python

import argparse
import glob
import json

from itertools import cycle

import numpy as np
import matplotlib.pyplot as plt

def parse_args():
    parser = argparse.ArgumentParser("Plot tool")
    """
    parser.add_argument(
        "files",
        nargs="+",
        help="JSON plot files"
    )
    """
    args = parser.parse_args()
    return args


def construct_color_map(keys):
    prop_cycle = plt.rcParams['axes.prop_cycle']
    color_cycle = cycle(prop_cycle.by_key()['color'])

    colors = {}
    for key in keys:
        color = next(color_cycle)
        colors[key] = color

    return colors


def compute_stats(keys, data):
    stats = {}
    for key in keys:
        entries = [entry for entry in data if entry["name"] == key]
        final_values = np.array([entry["times"][-1] for entry in entries])
        stats[key] = final_values

    return stats



def main():
    args = parse_args()

    files = glob.glob("results/fill_avg_*.json")

    data = [
        json.load(open(f))
        for f in sorted(files)
    ]

    keys = [entry["name"] for entry in data if entry["run"] == 1]

    color_map = construct_color_map(keys)
    stats = compute_stats(keys, data)

    # import IPython; IPython.embed()

    fig, axes = plt.subplots(2, 1, figsize=(12, 8), sharex=True)

    fig.text(0.8, 0.93, "Total times [ms]", fontsize=9, family="monospace", weight="bold")
    y_text = 0.9

    for i, entry in enumerate(data):
        name = entry["name"]
        iters = np.array(entry["iters"])
        times = np.array(entry["times"])

        color = color_map[name]
        is_primary = entry["run"] == 1

        if is_primary:
            label = name

            mean = stats[name].mean() * 1000
            std = stats[name].std() * 1000
            fig.text(0.80, y_text, name, fontsize=9, family="monospace")
            fig.text(0.88, y_text, "{:5.1f}".format(mean), fontsize=9, family="monospace")
            fig.text(0.93, y_text, "+/- {:5.3f}".format(std), fontsize=9, family="monospace")
            y_text -= 0.03
        else:
            label = None

        axes[0].plot(
            iters, times, "-",
            c=color, alpha=0.5, label=label,
        )
        axes[1].plot(
            iters[1:], times[1:] - times[:-1],
            "o", c=color, ms=0.4, alpha=0.8, label=label,
        )

    axes[0].legend(loc="best")
    axes[1].legend(loc="best")

    axes[1].set_yscale('log')

    fig.tight_layout()
    plt.subplots_adjust(right=0.75)

    plt.show()
    #import IPython; IPython.embed()


if __name__ == "__main__":
    main()

