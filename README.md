[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/to-offset)
[![crates.io](https://img.shields.io/crates/v/to-offset.svg)](https://crates.io/crates/to-offset)
[![docs.rs](https://docs.rs/to-offset/badge.svg)](https://docs.rs/to-offset)

# ToOffset and FromOffset

## Convert Signed and Unsigned Integers to Valid Indices

This crate introduces two traits to simplify working with a range of integer types or to constrain generic parameters for safe casting to `usize`, following these rules:

- **Negative Offsets**: Count backwards from the given length. For example, `-2` with an array length of 10 refers to index `8` (the last but one).
- **Overflow/Underflow**:
  - If the offset underflows, it returns `0`.
  - If it overflows, it returns the last index for arrays or the length for integer types.

### ToOffset

The `to_offset(length: usize)` method is implemented for `i32`, `i64`, `u8`, `u32`, `u64`, and `usize`. Here, `length` represents the maximum offset, akin to the end of a slice range.

```rust
let sample_array = [1, 2, 3, 4, 5, 6];

// Example with negative offset
let relative_index_1 = -2;
let result_1 = relative_index_1.to_offset(sample_array.len());
// result_1 is 4, the index of the penultimate element

// Example with overflow
let relative_index_2 = 100;
let result_2 = relative_index_2.to_offset(sample_array.len());
// result_2 is 6, the length of the array

// Example with underflow
let relative_index_3 = -100;
let result_3 = relative_index_3.to_offset(sample_array.len());
// result_3 is 0, the first index
```

### FromOffset

The `from_offset(offset: T) -> Option<&T>` method works with arrays or vectors, accepting any integer type that implements `ToOffset`.

```rust
let sample_array = [1, 2, 3, 4, 5, 6];

// Accessing the penultimate element
let penultimate_element = sample_array.from_offset(-2);
// equals Some(&5), reference to the last but one element

// Negative offset beyond bounds
let end_minus_10 = sample_array.from_offset(-10);
// equals Some(1), the first element as there are only 6 and it would otherwise underflow

let start_plus_10 = sample_array.from_offset(10);
// equals Some(6), the last element as there are only 6
```
