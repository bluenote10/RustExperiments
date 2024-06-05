from dataclasses import dataclass


@dataclass
class Slider:
    min: float
    init: float
    max: float

    def __post_init__(self):
        assert self.min <= self.init <= self.max
