
# Maturin

Useful command line snippets:

```sh
maturin develop --uv && python example/example_1.py
```

Note that I initially thought that I have to add `pip` to the `requirements.in`, because maturin errors when it doesn't find pip.
However, it looks like it actually can internally use `uv` as well:
- https://github.com/PyO3/maturin/issues/1959
- https://github.com/PyO3/maturin/pull/2015
- https://github.com/PyO3/maturin/pull/2015/files


# PyO3

General resources:
- https://pyo3.rs/v0.21.2/types

To get around the limitation of using a Python callback from a `Send + 'static` Rust closure:
- https://github.com/PyO3/pyo3/discussions/3788#discussioncomment-8325882
- https://docs.rs/pyo3/0.21.2/pyo3/marker/struct.Python.html#method.with_gil