from typing import Callable

from .types import Slider

def run(sliders: list[Slider], callback: Callable[[float], None]): ...
