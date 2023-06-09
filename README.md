# tinymath

[![crates.io](https://img.shields.io/crates/v/tinymath)](https://crates.io/crates/tinymath)

Experimental routines for performing arithmetic on small data types.

This focus of this crate is currently on functions to perform operations
of the form `a * b / 2^c`. Normally this operation requires using the
integer one size larger than the size of `a` and `b`, but this crate
performs these operations without resorting to a larger integer type.

The included Criterion benchmarks indicate that, individually, these
operations are slightly faster than the equivalent operations utilizing
the larger integer type.

The motivation to write this crate was to experiment with vectorization;
in theory, 8 ops in i8 could take as long as 4 ops in i16. This has not
been definitively measured.

This is not an officially supported Google product.
