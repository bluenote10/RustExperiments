import simple_app
from simple_app import Slider

elements = [
    (slider1 := Slider(0.0, 0.5, 1.0)),
    (slider2 := Slider(0.0, 20.0, 100.0)),
]


def callback() -> None:
    print(f"{slider1.value=}")
    print(f"{slider2.value=}")


simple_app.run(elements, callback)
