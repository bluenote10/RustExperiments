from typing import Sequence

from . import _simple_app
from .types import Callback, Input


def run(inputs: Sequence[Input], callback: Callback):
    _simple_app.run(inputs, callback)
