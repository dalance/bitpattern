use bitpattern::bitpattern;

#[test]
fn test_bit1() {
    let x = 0u8;
    assert_eq!(bitpattern!("0", x), Some(()));
    assert_eq!(bitpattern!("1", x), None);
    assert_eq!(bitpattern!("?", x), Some(()));
    assert_eq!(bitpattern!("a", x), Some(0));
    let x = 1;
    assert_eq!(bitpattern!("0", x), None);
    assert_eq!(bitpattern!("1", x), Some(()));
    assert_eq!(bitpattern!("?", x), Some(()));
    assert_eq!(bitpattern!("a", x), Some(1));
}

#[test]
fn test_bit8() {
    let x = 0xacu8;
    assert_eq!(bitpattern!("10101100", x), Some(()));
    assert_eq!(bitpattern!("10100100", x), None);
    assert_eq!(bitpattern!("1?101?00", x), Some(()));
    assert_eq!(bitpattern!("1a101100", x), Some(0));
    assert_eq!(bitpattern!("1ab011c0", x), Some((0, 1, 0)));
    assert_eq!(bitpattern!("1ab0?1c?", x), Some((0, 1, 0)));
    assert_eq!(bitpattern!("aaa01bb0", x), Some((5, 2)));
}

#[test]
fn test_bit16() {
    let x = 0xacf0u16;
    assert_eq!(bitpattern!("1010_1100_1111_0000", x), Some(()));
    assert_eq!(bitpattern!("1010_0100_1111_0000", x), None);
    assert_eq!(bitpattern!("1?10_1?00_1?11_0??0", x), Some(()));
    assert_eq!(bitpattern!("1a10_1100_1111_0000", x), Some(0));
    assert_eq!(bitpattern!("1ab0_11c0_1111_0000", x), Some((0, 1, 0)));
    assert_eq!(bitpattern!("1ab0_?1c?_1111_0000", x), Some((0, 1, 0)));
    assert_eq!(bitpattern!("aaa0_1bb0_111x_xxxx", x), Some((5, 2, 16)));
}

#[test]
fn test_bit32() {
    let x = 0xacf0acf0u32;
    assert_eq!(
        bitpattern!("1010_1100_1111_0000_1010_1100_1111_0000", x),
        Some(())
    );
    assert_eq!(
        bitpattern!("1010_0100_1111_0000_1010_0100_1111_0000", x),
        None
    );
    assert_eq!(
        bitpattern!("1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0", x),
        Some(())
    );
    assert_eq!(
        bitpattern!("1a10_1100_1111_0000_1a10_1100_1111_0000", x),
        Some((0, 0))
    );
    assert_eq!(
        bitpattern!("1ab0_11c0_1111_0000_1ab0_11c0_1111_0000", x),
        Some((0, 1, 0, 0, 1, 0))
    );
    assert_eq!(
        bitpattern!("1ab0_?1c?_1111_0000_1ab0_?1c?_1111_0000", x),
        Some((0, 1, 0, 0, 1, 0))
    );
    assert_eq!(
        bitpattern!("aaa0_1bb0_111x_xxxx_aaa0_1bb0_111x_xxxx", x),
        Some((5, 2, 16, 5, 2, 16))
    );
}

#[test]
fn test_bit64() {
    let x = 0xacf0acf0acf0acf0u64;
    assert_eq!(
        bitpattern!(
            "1010_1100_1111_0000_1010_1100_1111_0000_1010_1100_1111_0000_1010_1100_1111_0000",
            x
        ),
        Some(())
    );
    assert_eq!(
        bitpattern!(
            "1010_0100_1111_0000_1010_0100_1111_0000_1010_0100_1111_0000_1010_0100_1111_0000",
            x
        ),
        None
    );
    assert_eq!(
        bitpattern!(
            "1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0",
            x
        ),
        Some(())
    );
    assert_eq!(
        bitpattern!(
            "1a10_1100_1111_0000_1a10_1100_1111_0000_1a10_1100_1111_0000_1a10_1100_1111_0000",
            x
        ),
        Some((0, 0, 0, 0))
    );
    assert_eq!(
        bitpattern!(
            "1ab0_11c0_1111_0000_1ab0_11c0_1111_0000_1ab0_11c0_1111_0000_1ab0_11c0_1111_0000",
            x
        ),
        Some((0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0))
    );
    assert_eq!(
        bitpattern!(
            "1ab0_?1c?_1111_0000_1ab0_?1c?_1111_0000_1ab0_?1c?_1111_0000_1ab0_?1c?_1111_0000",
            x
        ),
        Some((0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0))
    );
    assert_eq!(
        bitpattern!(
            "aaa0_1bb0_111x_xxxx_aaa0_1bb0_111x_xxxx_aaa0_1bb0_111x_xxxx_aaa0_1bb0_111x_xxxx",
            x
        ),
        Some((5, 2, 16, 5, 2, 16, 5, 2, 16, 5, 2, 16))
    );
}

#[test]
fn test_bit128() {
    let x = 0xacf0acf0acf0acf0acf0acf0acf0acf0u128;
    assert_eq!( bitpattern!( "1010_1100_1111_0000_1010_1100_1111_0000_1010_1100_1111_0000_1010_1100_1111_0000_1010_1100_1111_0000_1010_1100_1111_0000_1010_1100_1111_0000_1010_1100_1111_0000", x), Some(()));
    assert_eq!( bitpattern!( "1010_0100_1111_0000_1010_0100_1111_0000_1010_0100_1111_0000_1010_0100_1111_0000_1010_0100_1111_0000_1010_0100_1111_0000_1010_0100_1111_0000_1010_0100_1111_0000", x), None);
    assert_eq!( bitpattern!( "1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0_1?10_1?00_1?11_0??0", x), Some(()));
    assert_eq!( bitpattern!( "1a10_1100_1111_0000_1a10_1100_1111_0000_1a10_1100_1111_0000_1a10_1100_1111_0000_1a10_1100_1111_0000_1a10_1100_1111_0000_1a10_1100_1111_0000_1a10_1100_1111_0000", x), Some((0, 0, 0, 0, 0, 0, 0, 0)));
}

#[test]
fn test_readme() {
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
}

#[test]
fn test_expr() {
    let x = 0xacu8;
    let y = 0x1;
    assert_eq!(bitpattern!("10101101", x + y), Some(()));
    assert_eq!(bitpattern!("10100101", x + y), None);
    assert_eq!(bitpattern!("1?101?01", x + y), Some(()));
    assert_eq!(bitpattern!("1a101101", x + y), Some(0));
    assert_eq!(bitpattern!("1ab011c1", x + y), Some((0, 1, 0)));
    assert_eq!(bitpattern!("1ab0?1c?", x + y), Some((0, 1, 0)));
    assert_eq!(bitpattern!("aaa01bb1", x + y), Some((5, 2)));

    assert_eq!(bitpattern!("10101101", 0xad), Some(()));
    assert_eq!(bitpattern!("10100101", 0xad), None);
    assert_eq!(bitpattern!("1?101?01", 0xad), Some(()));
    assert_eq!(bitpattern!("1a101101", 0xad), Some(0));
    assert_eq!(bitpattern!("1ab011c1", 0xad), Some((0, 1, 0)));
    assert_eq!(bitpattern!("1ab0?1c?", 0xad), Some((0, 1, 0)));
    assert_eq!(bitpattern!("aaa01bb1", 0xad), Some((5, 2)));
}
