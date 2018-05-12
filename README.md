# mathwrap

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

*mathwrap* is a math library with a focus on compatibility.

This is a work in progress, and is not ready for general use.

It uses types from [cgmath](https://crates.io/crates/cgmath) and
[nalgebra](https://crates.io/crates/nalgebra) directly, or else it
wraps them.  In a few cases a new type is created. In all cases where
it is possible [From](https://doc.rust-lang.org/std/convert/trait.From.html)
and [Into](https://doc.rust-lang.org/std/convert/trait.Into.html) are
implemented allowing direct conversion between types from different
math libraries (for unknown reasons, the neither author of the aforementioned
crates implemented compatibility with the other crate).

We attempt to eventually provide a superset of the features.  Additional features
not present in the aforementioned crates are implemented, as were required to
support the [Siege Engine](https://github.com/SiegeEngine) and in replacing
[siege-math](https://github.com/SiegeEngine/siege-math).
