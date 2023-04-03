# forte

`forte` provides basic functions for interacting with and manipulating pitch class sets

## Normal Form

A set can be converted into its normal form using the `to_normal_form` function

```rust
use forte::{Set, to_normal_form};

let set: Set = vec![0, 2, 11];
let normal = to_normal_form(&set);
println!("{normal:?}"); // => [11, 0, 2]
```

## Transposition

`forte` provides two methods for transposing a set.

First, via `transpose`, which takes a reference to a set and an integer
(mod 12) specifying the transposition level

```rust
use forte::{Set, transpose};

let set: Set = vec![0, 2, 5];
let transposed = transpose(&set, 3);
println!("{transposed:?}"); // => [3, 5, 8]
```

Second, by using `forte`'s `tX` functions, which tranpose a given set by `X`

```rust
use forte::Set;

let set: Set = vec![0, 2, 5];
let transposed = forte::t4(&set);
println!("{transposed:?}"); // => [4, 6, 9]
```

The available functions of this form are `t0(&set) - t11(&set)`
