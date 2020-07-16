# bitmatch
bitwise pattern matching and extracting

[![Actions Status](https://github.com/dalance/bitmatch/workflows/Regression/badge.svg)](https://github.com/dalance/bitmatch/actions)
[![Crates.io](https://img.shields.io/crates/v/bitmatch.svg)](https://crates.io/crates/bitmatch)
[![Docs.rs](https://docs.rs/bitmatch/badge.svg)](https://docs.rs/bitmatch)

## Usage

```Cargo.toml
[dependencies]
bitmatch = "0.1.0"
```

## Example

```rust
use bitmatch::bitmatch;

let x = 0xacu8; // 10101100

// '0' means the bit must be 0.
// '1' means the bit must be 1.
// ' ' can be uses as separator.
assert_eq!(bitmatch!("1010 1100", x), Some(()));
assert_eq!(bitmatch!("1010 0100", x), None);

// '_' means the bit can be 0 or 1.
assert_eq!(bitmatch!("1_10 1_00", x), Some(()));

// Other charactors can be used for extracting.
// 'a' extracts a single bit.
assert_eq!(bitmatch!("1a10 1100", x), Some(0));
assert_eq!(bitmatch!("10a0 1100", x), Some(1));

// Multi-bit extracting by continuous charactors.
assert_eq!(bitmatch!("1aaa a100", x), Some(5));

// Multiple extracting.
assert_eq!(bitmatch!("1aa0 aa00", x), Some((1, 3)));

// If the extracting fields are adjacent, the different charactors can be used.
assert_eq!(bitmatch!("1aab bccc", x), Some((1, 1, 4)));
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
