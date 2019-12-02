# Vose Algorithm

*by Kenny Peluso*

*last updated: 12/2/19*

## Description

This implements the Vose Algorithm (an improvement upon the Alias Algorithm, though they sometimes erroneously share the name, "Alias") created by Vose, 1991. This is used to sample from arbitrary categorical distributions, though it is most applicable for non-symmetric categorical distributions with more than 2 outcomes.

**Inspiration**: http://datagenetics.com/blog/november52019/

**Source**: [A Linear Algorithm For Generating Random Numbers With a Given Distribution](https://web.archive.org/web/20131029203736/http://web.eecs.utk.edu/~vose/Publications/random.pdf)

## TODO

1. Implement random oralce for test input generation - Line 107 of `src/vose.rs`
2. Throughout, switch to generic types or `Float` and `Integer` from `num` crate
3. Normalize generic Vec with `sm()` upon `vose()` input
4. Normalize generic Vec with `sm()` upon `vose_n()` input
