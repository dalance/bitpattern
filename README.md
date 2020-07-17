# bitpattern
bitwise pattern matching and extracting

[![Actions Status](https://github.com/dalance/bitpattern/workflows/Regression/badge.svg)](https://github.com/dalance/bitpattern/actions)
[![Crates.io](https://img.shields.io/crates/v/bitpattern.svg)](https://crates.io/crates/bitpattern)
[![Docs.rs](https://docs.rs/bitpattern/badge.svg)](https://docs.rs/bitpattern)

## Usage

```Cargo.toml
[dependencies]
bitpattern = "0.1.0"
```

## Example

```rust
    let x = 0xacu8; // 10101100

    // '0' means the bit must be 0.
    // '1' means the bit must be 1.
    // '_' can be uses as separator.
    assert_eq!(bitpattern!("1010_1100", x), Some(()));
    assert_eq!(bitpattern!("1010_0100", x), None);

    // '?' means the bit can be 0 or 1.
    assert_eq!(bitpattern!("1?10_1?00", x), Some(()));

    // Other charactors can be used for extracting.
    // 'a' extracts a single bit.
    assert_eq!(bitpattern!("1a10_1100", x), Some(0));
    assert_eq!(bitpattern!("10a0_1100", x), Some(1));

    // Multi-bit extracting by continuous charactors.
    assert_eq!(bitpattern!("1aaa_a100", x), Some(5));

    // Multiple extracting.
    assert_eq!(bitpattern!("1aa0_aa00", x), Some((1, 3)));

    // If the extracting fields are adjacent, the different charactors can be used.
    assert_eq!(bitpattern!("1aab_bccc", x), Some((1, 1, 4)));
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
