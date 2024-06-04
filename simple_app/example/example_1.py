import simple_app

print(simple_app.__path__)
print(dir(simple_app))


def callback(*args, **kwargs) -> None:
    print(f"Callback called with: {args} / {kwargs}")


simple_app.regular_function()
simple_app.run(["foo", "bar", "baz"], callback)
