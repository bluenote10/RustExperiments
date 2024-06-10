from . import _simple_app
from .types import Callback, Slider


def run(sliders: list[Slider], callback: Callback):
    _simple_app.run(sliders, callback)
