from typing import Callable, Sequence

from . import _simple_app
from .types import Plot, Slider


def run(sliders: list[Slider], callback: Callable[[], Sequence[Plot]]):
    _simple_app.run(sliders, callback)
