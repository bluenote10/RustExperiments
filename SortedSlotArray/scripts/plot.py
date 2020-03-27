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

    files = glob.glob("results/*.json")

    data = [
        json.load(open(f))
        for f in files
    ]

    fig, axes = plt.subplots(2, 1, figsize=(16, 12))

    for entry in data:
        name = entry["name"]
        times = np.array(entry["times"])
        xs = (np.arange(len(times)) + 1) * 10
        axes[0].plot(xs, times, "-o", ms=0.5, alpha=0.5, label=name)
        axes[1].plot(xs[1:], times[1:] - times[:-1], "o", ms=0.4, alpha=0.8, label=name)

    axes[0].legend(loc="best")
    axes[1].legend(loc="best")
    plt.show()
    #import IPython; IPython.embed()


if __name__ == "__main__":
    main()

