use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

/// bitwise pattern matching and extracting.
///
/// # Example
///
///```rust
/// use bitpattern::bitpattern;
///
/// let x = 0xacu8; // 10101100
///
/// // '0' means the bit must be 0.
/// // '1' means the bit must be 1.
/// // '_' can be uses as separator.
/// assert_eq!(bitpattern!("1010_1100", x), Some(()));
/// assert_eq!(bitpattern!("1010_0100", x), None);
///
/// // '?' means the bit can be 0 or 1.
/// assert_eq!(bitpattern!("1?10_1?00", x), Some(()));
///
/// // Other charactors can be used for extracting.
/// // 'a' extracts a single bit.
/// assert_eq!(bitpattern!("1a10_1100", x), Some(0));
/// assert_eq!(bitpattern!("10a0_1100", x), Some(1));
///
/// // Multi-bit extracting by continuous charactors.
/// assert_eq!(bitpattern!("1aaa_a100", x), Some(5));
///
/// // Multiple extracting.
/// assert_eq!(bitpattern!("1aa0_aa00", x), Some((1, 3)));
///
/// // If the extracting fields are adjacent, the different charactors can be used.
/// assert_eq!(bitpattern!("1aab_bccc", x), Some((1, 1, 4)));
///```
#[proc_macro]
pub fn bitpattern(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let mut input = input.into_iter();
    let pattern = input.next().expect("too less arguments");
    let comma = input.next().expect("too less arguments");
    let expr: TokenStream = input.collect();

    let pattern = match pattern {
        TokenTree::Literal(x) => x.to_string(),
        _ => {
            panic!("1st argument must be string literal");
        }
    };
    let pattern = if pattern.starts_with('\"') & pattern.ends_with('\"') {
        String::from(&pattern[1..pattern.len() - 1]).replace("_", "")
    } else {
        panic!("1st argument must be string literal");
    };

    match comma {
        TokenTree::Punct(x) => {
            if x.as_char() != ',' {
                panic!("',' is required");
            }
        }
        _ => {
            panic!("',' is required");
        }
    }

    match pattern.len() {
        1..=8 => gen_code_u8(pattern, expr),
        9..=16 => gen_code_u16(pattern, expr),
        17..=32 => gen_code_u32(pattern, expr),
        33..=64 => gen_code_u64(pattern, expr),
        65..=128 => gen_code_u128(pattern, expr),
        _ => {
            panic!("unsupported pattern length: {}", pattern.len());
        }
    }
}

macro_rules! gen_code {
    ($x:ty) => {
        paste::item! {
            fn [<gen_code_$x>](pattern: String, expr: TokenStream) -> proc_macro::TokenStream {
                let mut bit_mask: $x = 0;
                let mut bit_pattern: $x = 0;

                let mut args_pos = Vec::new();
                let mut args_mask = Vec::new();

                let mut prev = None;
                let mut count = 0;

                for (i, bit) in pattern.chars().enumerate() {
                    bit_mask <<= 1;
                    bit_pattern <<= 1;
                    match bit {
                        '0' => {
                            bit_mask |= 1;
                            bit_pattern |= 0;
                        }
                        '1' => {
                            bit_mask |= 1;
                            bit_pattern |= 1;
                        }
                        '_' => {
                            bit_mask |= 0;
                            bit_pattern |= 0;
                        }
                        _ => {
                            bit_mask |= 0;
                            bit_pattern |= 0;
                        }
                    }
                    if let Some(x) = prev {
                        if x != bit && x != '0' && x != '1' && x != '?' {
                            let pos = (pattern.len() - i) as $x;
                            let mut mask: $x = 0;
                            for _ in 0..count {
                                mask <<= 1;
                                mask |= 1;
                            }
                            args_pos.push(pos);
                            args_mask.push(mask);

                            count = 0;
                        } else if x != bit {
                            count = 0;
                        }
                    }
                    count += 1;
                    prev = Some(bit);
                }
                if let Some(x) = prev {
                    if x != '0' && x != '1' && x != '?' {
                        let pos = 0 as $x;
                        let mut mask: $x = 0;
                        for _ in 0..count {
                            mask <<= 1;
                            mask |= 1;
                        }
                        args_pos.push(pos);
                        args_mask.push(mask);
                    }
                }

                let gen = quote! {
                    {
                        let value = (#expr) as $x;
                        if value & #bit_mask == #bit_pattern {
                            Some((
                                    #(
                                        (value >> #args_pos) & #args_mask
                                    ),*
                                ))
                        } else {
                            None
                        }
                    }
                };

                gen.into()
            }
        }
    };
}

gen_code!(u8);
gen_code!(u16);
gen_code!(u32);
gen_code!(u64);
gen_code!(u128);
