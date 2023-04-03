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

## Prime Form

A set can be converted to the prime form of its set class using the `to_prime_form` function

```rust
use forte::{to_prime_form, Set};

let set: Set = vec![1, 5, 6, 7];
let prime = to_prime_form(&set);
println!("{prime:?}") // => [0, 1, 2, 6]
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

## Inversion

Similar to transposition, `forte` provides two methods for inverting a set.

First, via `invert`, which takes a reference to a set and an integer
(mod 12) specifying the inversion level

```rust
use forte::{Set, invert};

let set: Set = vec![0, 2, 5];
let inverted = invert(&set, 4);
println!("{inverted:?}") // => [11, 2, 4];
```

Second, by using `forte`'s `iX` functions, which invert a given set by `X`

```rust
use forte::Set;

let set: Set = vec![0, 2, 5];
let inverted = forte::i3(&set);
println!("{inverted:?}") // => [10, 1, 3];
```

`forte` also provides the `invert_by_pair` function, which takes a reference
to a set, and a 2-element tuple of integers (mod 12) that defines one of the
desired inversion level's pitch class mappings.

```rust
use forte::{invert_by_pair, Set};

let set: Set = vec![0, 2, 5];
let inverted = invert_by_pair(&set, (1, 5));
println!("{inverted:?}") // => [1, 4, 6];
```
