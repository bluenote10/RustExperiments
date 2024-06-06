import numpy as np

import simple_app
from simple_app import Plot, Slider

elements = [
    (slider_a := Slider("a", -10.0, 0.5, 10.0)),
    (slider_b := Slider("b", -10.0, 0.5, 10.0)),
    (slider_c := Slider("c", -10.0, 0.5, 10.0)),
]


def callback() -> list[Plot]:
    print(f"{slider_a.value=} {slider_b.value=} {slider_c.value=}")
    a = slider_a.value
    b = slider_b.value
    c = slider_c.value
    xs = np.linspace(-10.0, 10.0, 100)
    ys = a * xs**2 + b * xs + c
    return [Plot(xs, ys, x_limits=(-10, +10), y_limits=(-10, +10))]


simple_app.run(elements, callback)
