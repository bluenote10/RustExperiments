from typing import Any, Callable, Sequence

from .types import Input

def run(inputs: Sequence[Input], callback: Callable[[], Any]): ...
