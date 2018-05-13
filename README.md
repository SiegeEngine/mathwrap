# mathwrap

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

*mathwrap* is a math library with a focus on compatibility.

This is a work in progress, and is not ready for general use.

For unknown reasons, neither the author of
[cgmath](https://crates.io/crates/cgmath)
nor the author of
[nalgebra](https://crates.io/crates/nalgebra)
implemented compatibility (From/Into) with the other library.
This creates friction for downstream projects which must work with multiple
non-math libraries which depend on different math libraries.

This crate wraps types from
[cgmath](https://crates.io/crates/cgmath) and
[nalgebra](https://crates.io/crates/nalgebra).
In a few cases an entirely new type is created.
In all cases where it is possible
[From](https://doc.rust-lang.org/std/convert/trait.From.html)
and/or [Into](https://doc.rust-lang.org/std/convert/trait.Into.html)
are implemented allowing direct conversion between types from different
math libraries. Wherever possible, this conversion compiles away and has
zero runtime cost (but this is not always possible).

We attempt to eventually provide a superset of the features.  Additional features
not present in the aforementioned crates are implemented, as were required to
support the [Siege Engine](https://github.com/SiegeEngine) and in replacing
[siege-math](https://github.com/SiegeEngine/siege-math).

To convert foreign types into our types, use either the `From` or `Into` trait.
To convert back to the foreign type you must use the `Into` trait due to
coherence rule limitations in the Rust language.
