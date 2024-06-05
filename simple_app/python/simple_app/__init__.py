from typing import Callable

from . import _simple_app
from .types import Slider


def run(sliders: list[Slider], callback: Callable[[float], None]):
    _simple_app.run(sliders, callback)
