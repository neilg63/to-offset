[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/to-offset)
[![crates.io](https://img.shields.io/crates/v/to-offset.svg)](https://crates.io/crates/to-offset)
[![docs.rs](https://docs.rs/to-offset/badge.svg)](https://docs.rs/to-offset)

# ToOffset and FromOfsset

## Cast offsets, as any of the core signed and unsigned integer types, to valid unsigned indices with a defined length

This crate adds provides two simple traits that make it easier to work a range of common signed and unsigned integer types or to constrain generic parameters to integer type that may be safely cast to usize with the following rules:

Negative offsets work backwards from the projected length, e.g -2 with an array length of 10 would refer the last but one index of 8, the last being 9.

- If the offset underflows, it will return 0.
- If the offset overflows, it will return the last index with arrays and the length with integer types.

### ToOfset to_offset(length: usize)

The `to_offset(length: usize)` method will work with i32, i64, u8, u32, u64 and usize. The length parameters is the maximum offset, compatible with slice ranges where the rightmost value is the next index.

```rust
let sample_array = [1, 2, 3, 4, 5, 6];
let relative_index_1 = -2;
let result_1 = relative_index_1.to_offset(sample_array.len());
// result_1 is the penultimate index

let relative_index_2 = 100;
let result_2 = relative_index_2.to_offset(sample_array.len());
// result_2 is 6, i.e. the array size

let relative_index_3 = -100;
let result_2 = relative_index_3.to_offset(sample_array.len());
// result_3 is 0, i.e. the first index

```

### FromOfset from_offset(offset: T) -> Option<&T>

This method works with arrays or vectors. Unlike get, it accepts all integers types that implement ToOffset.

```rust
let sample_array = [1, 2, 3, 4, 5, 6];

let penultimate_element = sample_array.from_offset(-2);
// equauls 5, the last but one array value
```
