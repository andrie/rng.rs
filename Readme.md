## rng.rs

This is a simple Rust program that demonstrates random number generation that is identical to the default behavior of the R programming language.

This code is still very experimental, and it's on my to-do list to convert the embedded `rnorm.rs` module to a standalone crate.

The library exposes these functions:

- `runif` for a uniform distribution.
- `rnorm` for a normal distribution.
- `r_set_seed()` to set the random seed.


In Rust:

```rust
r_set_seed(42);
println!("rnorm: {:.5?}", rnorm(Some(5), 0.0, 1.0));
```

```
rnorm: [1.37096, -0.56470, 0.36313, 0.63286, 0.40427]
```

The equivalent code in R:

``` R
set.seed(42); rnorm(5)
```

```
[1]  1.3709584 -0.5646982  0.3631284  0.6328626  0.4042683
```