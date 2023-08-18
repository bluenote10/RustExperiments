
The main functions of the Shewchuk technique were originally coming from:
https://github.com/georust/robust

And I had opened this issue related to it, which contains a few more notes:
https://github.com/georust/robust/issues/7

My original implementation was within the geometry benchmarks in `../Benchmarks/src/robust_alt.rs`.
This version here is a slight cleanup, mainly just removing the actualy `orient2d` and `incircle`
functions, and all their dependencies, keeping just what is needed for second order expansions.

Note that nowadays, a web search for "rust robust" actually also reveals another implementation:
- https://docs.rs/robust-geo/latest/robust_geo/
- https://github.com/josh65536/robust-geo
