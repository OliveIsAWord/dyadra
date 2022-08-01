# Dyadra

Dyadra is an exploration in alternative arithmetic computation, perhaps most similar to [stochastic computing](https://en.wikipedia.org/wiki/Stochastic_computing). This repository is an implementation of an arithmetic type using Dyadra in Rust.

At its heart, Dyadra is a bijection between the natural numbers and the [Dyadic rationals](https://en.wikipedia.org/wiki/Dyadic_rational) in the interval `(0, 1]`. Operations like multiplication and comparison on this set of numbers seem to have reasonably efficient implementations in this unsigned integer space. For example, these could easily be microprocessor instructions.

Extensions of this system could include signed numbers and/or `0`, as well as exclude `1`.
