from dataclasses import dataclass


@dataclass
class Slider:
    min: float
    value: float
    max: float

    def __post_init__(self):
        assert self.min <= self.value <= self.max
