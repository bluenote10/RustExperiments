#!/usr/bin/env python

import argparse
import glob
import json

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


def main():
    args = parse_args()

    files = glob.glob("results/fill_avg_*.json")

    data = [
        json.load(open(f))
        for f in sorted(files)
    ]

    fig, axes = plt.subplots(2, 1, figsize=(12, 8), sharex=True)

    for i, entry in enumerate(data):
        name = entry["name"]
        iters = np.array(entry["iters"])
        times = np.array(entry["times"])
        axes[0].plot(iters, times, "-o", ms=0.5, alpha=0.5, label=name)
        axes[1].plot(iters[1:], times[1:] - times[:-1], "o", ms=0.4, alpha=0.8, label=name)

        fig.text(0.8, 0.9 - (0.03 * i), name, fontsize=9, family="monospace")
        fig.text(0.9, 0.9 - (0.03 * i), "{:5.1f}".format(times[-1] * 1000), fontsize=9, family="monospace")

    fig.text(0.8, 0.93, "Total times [ms]", fontsize=9, family="monospace", weight="bold")

    axes[0].legend(loc="best")
    axes[1].legend(loc="best")

    fig.tight_layout()
    plt.subplots_adjust(right=0.75)


    plt.show()
    #import IPython; IPython.embed()


if __name__ == "__main__":
    main()

