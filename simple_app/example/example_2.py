import numpy as np

import simple_app
from simple_app import Inputs, IntSlider, Outputs, Plot, Slider


def main():

    elements = [
        (master_slider := IntSlider("Number of sliders", 1, 5, 10)),
    ]

    def callback_1() -> Inputs:

        order = int(master_slider.value)  # TODO: IntSlider
        print(f"Polynomial order: {order}")

        sliders = [Slider(f"coef_{i}", -10.0, 0.5, 10.0) for i in range(order + 1)]

        def callback_2() -> Outputs:
            xs = np.linspace(-10.0, 10.0, 100)
            ys = np.zeros_like(xs)
            for k in range(order + 1):
                ys += sliders[k].value * xs**k
            return Outputs(
                Plot(xs, ys, y_limits=(-10, +10)),
            )

        return Inputs(*sliders, callback=callback_2)

    simple_app.run(elements, callback_1)


if __name__ == "__main__":
    main()
