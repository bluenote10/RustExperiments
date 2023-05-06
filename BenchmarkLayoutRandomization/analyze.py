#!/usr/bin/env python

import subprocess
from pathlib import Path
from typing import Callable

import matplotlib.pyplot as plt
import numpy as np


def measure(set_cwd: bool) -> float:
    exec_path = Path(__file__).parent / "micro_kernel"
    output = subprocess.check_output(
        [exec_path],
        text=True,
        cwd="/tmp" if set_cwd else None,
        env={},
    )
    return float(output)


def collect_sample(n: int, measure_func: Callable[[], float]) -> np.ndarray:
    return np.array([measure_func() for _ in range(n)])


def main():
    n = 200

    times = np.concatenate(
        [
            collect_sample(n, lambda: measure(set_cwd=True)),
            collect_sample(n, lambda: measure(set_cwd=False)),
        ]
    )

    mean = np.mean(times)
    std = np.std(times)

    print(f"{mean:.1f} ± {std:.3f}")

    fig, ax = plt.subplots(1, 1, figsize=(12, 8))
    ax.plot(times, "o", ms=2)
    fig.tight_layout()
    plt.show()


if __name__ == "__main__":
    main()
