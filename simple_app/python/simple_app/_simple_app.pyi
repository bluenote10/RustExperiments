from typing import Any, Callable

from .types import Slider

def run(sliders: list[Slider], callback: Callable[[], Any]): ...
