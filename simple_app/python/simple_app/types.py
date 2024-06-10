from __future__ import annotations

from dataclasses import dataclass
from typing import Callable, Sequence

import numpy as np


@dataclass
class Slider:
    name: str
    min: float
    value: float
    max: float

    def __post_init__(self) -> None:
        assert self.min <= self.value <= self.max


Input = Slider  # Will eventually be a union type / base type of all supported outputs.


class Inputs:
    def __init__(self, *inputs: Input, callback: Callback):
        self.inputs = inputs
        self.callback = callback


Data = np.ndarray | list[float]


class Plot:
    def __init__(
        self,
        xs: Data,
        ys: Data,
        *,
        x_limits: tuple[float, float] | None = None,
        y_limits: tuple[float, float] | None = None,
    ):
        self.xs = xs
        self.ys = ys
        self.x_limits = _use_or_infer(x_limits, xs)
        self.y_limits = _use_or_infer(y_limits, ys)


def _use_or_infer(
    limits: tuple[float, float] | None, data: Data
) -> tuple[float, float]:
    if limits is None:
        return float(np.min(data)), float(np.max(data))
    else:
        return limits


Output = Plot  # Will eventually be a union type / base type of all supported outputs.


class Outputs:
    def __init__(self, *outputs: Output):
        self.outputs = outputs


Callback = Callable[[], Outputs | Inputs]
