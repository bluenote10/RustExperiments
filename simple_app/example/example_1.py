import simple_app
from simple_app import Slider


def callback(*args, **kwargs) -> None:
    print(f"Callback called with: {args} / {kwargs}")


simple_app.run(
    [
        Slider(0.0, 0.5, 1.0),
        Slider(0.0, 20.0, 100.0),
    ],
    callback,
)
