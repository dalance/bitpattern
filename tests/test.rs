use bitmatch::bitmatch;

#[test]
fn test_bit1() {
    let x = 0u8;
    assert_eq!(bitmatch!("0", x), Some(()));
    assert_eq!(bitmatch!("1", x), None);
    assert_eq!(bitmatch!("_", x), Some(()));
    assert_eq!(bitmatch!("a", x), Some(0));
    let x = 1;
    assert_eq!(bitmatch!("0", x), None);
    assert_eq!(bitmatch!("1", x), Some(()));
    assert_eq!(bitmatch!("_", x), Some(()));
    assert_eq!(bitmatch!("a", x), Some(1));
}

#[test]
fn test_bit8() {
    let x = 0xacu8;
    assert_eq!(bitmatch!("10101100", x), Some(()));
    assert_eq!(bitmatch!("10100100", x), None);
    assert_eq!(bitmatch!("1_101_00", x), Some(()));
    assert_eq!(bitmatch!("1a101100", x), Some(0));
    assert_eq!(bitmatch!("1ab011c0", x), Some((0, 1, 0)));
    assert_eq!(bitmatch!("1ab0_1c_", x), Some((0, 1, 0)));
    assert_eq!(bitmatch!("aaa01bb0", x), Some((5, 2)));
}

#[test]
fn test_bit16() {
    let x = 0xacf0u16;
    assert_eq!(bitmatch!("1010 1100 1111 0000", x), Some(()));
    assert_eq!(bitmatch!("1010 0100 1111 0000", x), None);
    assert_eq!(bitmatch!("1_10 1_00 1_11 0__0", x), Some(()));
    assert_eq!(bitmatch!("1a10 1100 1111 0000", x), Some(0));
    assert_eq!(bitmatch!("1ab0 11c0 1111 0000", x), Some((0, 1, 0)));
    assert_eq!(bitmatch!("1ab0 _1c_ 1111 0000", x), Some((0, 1, 0)));
    assert_eq!(bitmatch!("aaa0 1bb0 111x xxxx", x), Some((5, 2, 16)));
}

#[test]
fn test_bit32() {
    let x = 0xacf0acf0u32;
    assert_eq!(
        bitmatch!("1010 1100 1111 0000 1010 1100 1111 0000", x),
        Some(())
    );
    assert_eq!(
        bitmatch!("1010 0100 1111 0000 1010 0100 1111 0000", x),
        None
    );
    assert_eq!(
        bitmatch!("1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0", x),
        Some(())
    );
    assert_eq!(
        bitmatch!("1a10 1100 1111 0000 1a10 1100 1111 0000", x),
        Some((0, 0))
    );
    assert_eq!(
        bitmatch!("1ab0 11c0 1111 0000 1ab0 11c0 1111 0000", x),
        Some((0, 1, 0, 0, 1, 0))
    );
    assert_eq!(
        bitmatch!("1ab0 _1c_ 1111 0000 1ab0 _1c_ 1111 0000", x),
        Some((0, 1, 0, 0, 1, 0))
    );
    assert_eq!(
        bitmatch!("aaa0 1bb0 111x xxxx aaa0 1bb0 111x xxxx", x),
        Some((5, 2, 16, 5, 2, 16))
    );
}

#[test]
fn test_bit64() {
    let x = 0xacf0acf0acf0acf0u64;
    assert_eq!(
        bitmatch!(
            "1010 1100 1111 0000 1010 1100 1111 0000 1010 1100 1111 0000 1010 1100 1111 0000",
            x
        ),
        Some(())
    );
    assert_eq!(
        bitmatch!(
            "1010 0100 1111 0000 1010 0100 1111 0000 1010 0100 1111 0000 1010 0100 1111 0000",
            x
        ),
        None
    );
    assert_eq!(
        bitmatch!(
            "1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0",
            x
        ),
        Some(())
    );
    assert_eq!(
        bitmatch!(
            "1a10 1100 1111 0000 1a10 1100 1111 0000 1a10 1100 1111 0000 1a10 1100 1111 0000",
            x
        ),
        Some((0, 0, 0, 0))
    );
    assert_eq!(
        bitmatch!(
            "1ab0 11c0 1111 0000 1ab0 11c0 1111 0000 1ab0 11c0 1111 0000 1ab0 11c0 1111 0000",
            x
        ),
        Some((0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0))
    );
    assert_eq!(
        bitmatch!(
            "1ab0 _1c_ 1111 0000 1ab0 _1c_ 1111 0000 1ab0 _1c_ 1111 0000 1ab0 _1c_ 1111 0000",
            x
        ),
        Some((0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0))
    );
    assert_eq!(
        bitmatch!(
            "aaa0 1bb0 111x xxxx aaa0 1bb0 111x xxxx aaa0 1bb0 111x xxxx aaa0 1bb0 111x xxxx",
            x
        ),
        Some((5, 2, 16, 5, 2, 16, 5, 2, 16, 5, 2, 16))
    );
}

#[test]
fn test_bit128() {
    let x = 0xacf0acf0acf0acf0acf0acf0acf0acf0u128;
    assert_eq!(
        bitmatch!(
            "1010 1100 1111 0000 1010 1100 1111 0000 1010 1100 1111 0000 1010 1100 1111 0000 1010 1100 1111 0000 1010 1100 1111 0000 1010 1100 1111 0000 1010 1100 1111 0000",
            x
        ),
        Some(())
    );
    assert_eq!(
        bitmatch!(
            "1010 0100 1111 0000 1010 0100 1111 0000 1010 0100 1111 0000 1010 0100 1111 0000 1010 0100 1111 0000 1010 0100 1111 0000 1010 0100 1111 0000 1010 0100 1111 0000",
            x
        ),
        None
    );
    assert_eq!(
        bitmatch!(
            "1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0 1_10 1_00 1_11 0__0",
            x
        ),
        Some(())
    );
    assert_eq!(
        bitmatch!(
            "1a10 1100 1111 0000 1a10 1100 1111 0000 1a10 1100 1111 0000 1a10 1100 1111 0000 1a10 1100 1111 0000 1a10 1100 1111 0000 1a10 1100 1111 0000 1a10 1100 1111 0000",
            x
        ),
        Some((0, 0, 0, 0, 0, 0, 0, 0))
    );
}

#[test]
fn test_readme() {
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
}
