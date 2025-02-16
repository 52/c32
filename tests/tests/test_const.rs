// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

use c32::Buffer;

mod __internal {
    /// A test helper for default features.
    macro_rules! const_test {
        {
            $name:ident,
            ENC = Buffer<$n:literal>,
            DEC = Buffer<$m:literal>,
            INPUT = [$l:expr, $i:expr],
            EXPECTED = $s:expr
        } => {
            #[test]
            fn $name() {
                const INPUT: [u8; $l] = $i;
                const ENC: Buffer<$n> = Buffer::encode(&INPUT);
                assert_eq!(ENC.as_str(), $s);
                const DEC: Buffer<$m> = Buffer::decode(&ENC.as_bytes());
                assert_eq!(DEC.as_bytes(), INPUT);
            }
        };
    }

    /// A test helper for `[feature = "check"]`.
    macro_rules! const_test_check {
         {
             $name:ident,
             ENC = Buffer<$n:literal>,
             DEC = Buffer<$m:literal>,
             INPUT = [$l:expr, $i:expr],
             VERSION = $v:expr,
             EXPECTED = $s:expr
         } => {
             #[test]
             fn $name() {
                 const INPUT: [u8; $l] = $i;
                 const VERSION: u8 = $v;
                 const ENC: Buffer<$n> = Buffer::encode_check(&INPUT, VERSION);
                 assert_eq!(ENC.as_str(), $s);
                 const DEC: Buffer<$m> = Buffer::decode_check(&ENC.as_bytes());
                 assert_eq!(DEC.as_bytes(), INPUT);
             }
         };
     }

    /// A test helper for prefixed encoding/decoding.
    macro_rules! const_test_prefixed {
        {
            $name:ident,
            ENC = Buffer<$n:literal>,
            DEC = Buffer<$m:literal>,
            INPUT = [$l:expr, $i:expr],
            PREFIX = $p:expr,
            EXPECTED = $s:expr
        } => {
            #[test]
            fn $name() {
                const INPUT: [u8; $l] = $i;
                const PREFIX: char = $p;
                const ENC: Buffer<$n> = Buffer::encode_prefixed(&INPUT, PREFIX);
                assert_eq!(ENC.as_str(), $s);
                const DEC: Buffer<$m> = Buffer::decode_prefixed(ENC.as_bytes(), PREFIX);
                assert_eq!(DEC.as_bytes(), INPUT);
            }
        };
    }

    /// A test helper for `[feature = "check"]` prefixed encoding/decoding.
    macro_rules! const_test_check_prefixed {
        {
            $name:ident,
            ENC = Buffer<$n:literal>,
            DEC = Buffer<$m:literal>,
            INPUT = [$l:expr, $i:expr],
            PREFIX = $p:expr,
            VERSION = $v:expr,
            EXPECTED = $s:expr
        } => {
            #[test]
            fn $name() {
                const INPUT: [u8; $l] = $i;
                const PREFIX: char = $p;
                const VERSION: u8 = $v;
                const ENC: Buffer<$n> = Buffer::encode_check_prefixed(&INPUT, PREFIX, VERSION);
                assert_eq!(ENC.as_str(), $s);
                const DEC: Buffer<$m> = Buffer::decode_check_prefixed(ENC.as_bytes(), PREFIX);
                assert_eq!(DEC.as_bytes(), INPUT);
            }
        };
    }

    pub(crate) use const_test;
    pub(crate) use const_test_check;
    pub(crate) use const_test_check_prefixed;
    pub(crate) use const_test_prefixed;
}

__internal::const_test! {
    test_empty,
    ENC = Buffer<0>,
    DEC = Buffer<0>,
    INPUT = [0, []],
    EXPECTED = ""
}

__internal::const_test_check! {
    test_check_empty,
    ENC = Buffer<8>,
    DEC = Buffer<7>,
    INPUT = [0, []],
    VERSION = 0,
    EXPECTED = "0A0DR2R"
}

__internal::const_test_prefixed! {
    test_prefixed_empty,
    ENC = Buffer<1>,
    DEC = Buffer<0>,
    INPUT = [0, []],
    PREFIX = 'S',
    EXPECTED = "S"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_empty,
    ENC = Buffer<9>,
    DEC = Buffer<8>,
    INPUT = [0, []],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S0A0DR2R"
}

__internal::const_test! {
    test_ascending_one,
    ENC = Buffer<2>,
    DEC = Buffer<1>,
    INPUT = [1, [1]],
    EXPECTED = "1"
}

__internal::const_test_check! {
    test_check_ascending_one,
    ENC = Buffer<9>,
    DEC = Buffer<8>,
    INPUT = [1, [1]],
    VERSION = 0,
    EXPECTED = "04C407K6"
}

__internal::const_test_prefixed! {
    test_prefixed_ascending_one,
    ENC = Buffer<3>,
    DEC = Buffer<1>,
    INPUT = [1, [1]],
    PREFIX = 'S',
    EXPECTED = "S1"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_ascending_one,
    ENC = Buffer<10>,
    DEC = Buffer<8>,
    INPUT = [1, [1]],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S04C407K6"
}

__internal::const_test! {
    test_ascending_two,
    ENC = Buffer<5>,
    DEC = Buffer<2>,
    INPUT = [2, [1, 2]],
    EXPECTED = "82"
}

__internal::const_test_check! {
    test_check_ascending_two,
    ENC = Buffer<11>,
    DEC = Buffer<10>,
    INPUT = [2, [1, 2]],
    VERSION = 0,
    EXPECTED = "0108TZKWMK"
}

__internal::const_test_prefixed! {
    test_prefixed_ascending_two,
    ENC = Buffer<5>,
    DEC = Buffer<2>,
    INPUT = [2, [1, 2]],
    PREFIX = 'S',
    EXPECTED = "S82"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_ascending_two,
    ENC = Buffer<12>,
    DEC = Buffer<10>,
    INPUT = [2, [1, 2]],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S0108TZKWMK"
}

__internal::const_test! {
    test_ascending_three,
    ENC = Buffer<5>,
    DEC = Buffer<4>,
    INPUT = [3, [1, 2, 3]],
    EXPECTED = "20G3"
}

__internal::const_test_check! {
    test_check_ascending_three,
    ENC = Buffer<13>,
    DEC = Buffer<11>,
    INPUT = [3, [1, 2, 3]],
    VERSION = 0,
    EXPECTED = "0820FVT6NE0"
}

__internal::const_test_prefixed! {
    test_prefixed_ascending_three,
    ENC = Buffer<6>,
    DEC = Buffer<4>,
    INPUT = [3, [1, 2, 3]],
    PREFIX = 'S',
    EXPECTED = "S20G3"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_ascending_three,
    ENC = Buffer<14>,
    DEC = Buffer<11>,
    INPUT = [3, [1, 2, 3]],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S0820FVT6NE0"
}

__internal::const_test! {
    test_ascending_four,
    ENC = Buffer<7>,
    DEC = Buffer<5>,
    INPUT = [4, [1, 2, 3, 4]],
    EXPECTED = "G40R4"
}

__internal::const_test_check! {
    test_check_ascending_four,
    ENC = Buffer<14>,
    DEC = Buffer<13>,
    INPUT = [4, [1, 2, 3, 4]],
    VERSION = 0,
    EXPECTED = "020G30HV8M1Y1"
}

__internal::const_test_prefixed! {
    test_prefixed_ascending_four,
    ENC = Buffer<8>,
    DEC = Buffer<5>,
    INPUT = [4, [1, 2, 3, 4]],
    PREFIX = 'S',
    EXPECTED = "SG40R4"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_ascending_four,
    ENC = Buffer<15>,
    DEC = Buffer<13>,
    INPUT = [4, [1, 2, 3, 4]],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S020G30HV8M1Y1"
}

__internal::const_test! {
    test_ascending_five,
    ENC = Buffer<8>,
    DEC = Buffer<7>,
    INPUT = [5, [1, 2, 3, 4, 5]],
    EXPECTED = "4106105"
}

__internal::const_test_check! {
    test_check_ascending_five,
    ENC = Buffer<16>,
    DEC = Buffer<14>,
    INPUT = [5, [1, 2, 3, 4, 5]],
    VERSION = 0,
    EXPECTED = "0G40R40QP9HXK8"
}

__internal::const_test_prefixed! {
    test_prefixed_ascending_five,
    ENC = Buffer<9>,
    DEC = Buffer<7>,
    INPUT = [5, [1, 2, 3, 4, 5]],
    PREFIX = 'S',
    EXPECTED = "S4106105"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_ascending_five,
    ENC = Buffer<17>,
    DEC = Buffer<14>,
    INPUT = [5, [1, 2, 3, 4, 5]],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S0G40R40QP9HXK8"
}

__internal::const_test! {
    test_zero_all,
    ENC = Buffer<8>,
    DEC = Buffer<5>,
    INPUT = [5, [0, 0, 0, 0, 0]],
    EXPECTED = "00000"
}

__internal::const_test_check! {
    test_check_zero_all,
    ENC = Buffer<16>,
    DEC = Buffer<13>,
    INPUT = [5, [0, 0, 0, 0, 0]],
    VERSION = 0,
    EXPECTED = "0000001CKYF1A"
}

__internal::const_test_prefixed! {
    test_prefixed_zero_all,
    ENC = Buffer<9>,
    DEC = Buffer<5>,
    INPUT = [5, [0, 0, 0, 0, 0]],
    PREFIX = 'S',
    EXPECTED = "S00000"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_zero_all,
    ENC = Buffer<17>,
    DEC = Buffer<13>,
    INPUT = [5, [0, 0, 0, 0, 0]],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S0000001CKYF1A"
}

__internal::const_test! {
    test_zero_alternating,
    ENC = Buffer<8>,
    DEC = Buffer<7>,
    INPUT = [5, [0, 1, 0, 1, 0]],
    EXPECTED = "0G0080"
}

__internal::const_test_check! {
    test_check_zero_alternating,
    ENC = Buffer<16>,
    DEC = Buffer<14>,
    INPUT = [5, [0, 1, 0, 1, 0]],
    VERSION = 0,
    EXPECTED = "00200100M9A7ZE"
}

__internal::const_test_prefixed! {
    test_prefixed_zero_alternating,
    ENC = Buffer<9>,
    DEC = Buffer<7>,
    INPUT = [5, [0, 1, 0, 1, 0]],
    PREFIX = 'S',
    EXPECTED = "S0G0080"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_zero_alternating,
    ENC = Buffer<17>,
    DEC = Buffer<14>,
    INPUT = [5, [0, 1, 0, 1, 0]],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S00200100M9A7ZE"
}

__internal::const_test! {
    test_zero_middle,
    ENC = Buffer<8>,
    DEC = Buffer<6>,
    INPUT = [5, [0, 0, 1, 0, 0]],
    EXPECTED = "002000"
}

__internal::const_test_check! {
    test_check_zero_middle,
    ENC = Buffer<16>,
    DEC = Buffer<13>,
    INPUT = [5, [0, 0, 1, 0, 0]],
    VERSION = 0,
    EXPECTED = "0008003VTB5H8"
}

__internal::const_test_prefixed! {
    test_prefixed_zero_middle,
    ENC = Buffer<9>,
    DEC = Buffer<6>,
    INPUT = [5, [0, 0, 1, 0, 0]],
    PREFIX = 'S',
    EXPECTED = "S002000"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_zero_middle,
    ENC = Buffer<17>,
    DEC = Buffer<13>,
    INPUT = [5, [0, 0, 1, 0, 0]],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S0008003VTB5H8"
}

__internal::const_test! {
    test_zero_second,
    ENC = Buffer<8>,
    DEC = Buffer<6>,
    INPUT = [5, [0, 1, 0, 0, 0]],
    EXPECTED = "0G0000"
}

__internal::const_test_check! {
    test_check_zero_second,
    ENC = Buffer<16>,
    DEC = Buffer<14>,
    INPUT = [5, [0, 1, 0, 0, 0]],
    VERSION = 0,
    EXPECTED = "00200003Y4CA34"
}

__internal::const_test_prefixed! {
    test_prefixed_zero_second,
    ENC = Buffer<9>,
    DEC = Buffer<6>,
    INPUT = [5, [0, 1, 0, 0, 0]],
    PREFIX = 'S',
    EXPECTED = "S0G0000"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_zero_second,
    ENC = Buffer<17>,
    DEC = Buffer<14>,
    INPUT = [5, [0, 1, 0, 0, 0]],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S00200003Y4CA34"
}

__internal::const_test! {
    test_zero_two_ones_end,
    ENC = Buffer<8>,
    DEC = Buffer<6>,
    INPUT = [5, [0, 1, 0, 0, 1]],
    EXPECTED = "0G0001"
}

__internal::const_test_check! {
    test_check_zero_two_ones_end,
    ENC = Buffer<16>,
    DEC = Buffer<14>,
    INPUT = [5, [0, 1, 0, 0, 1]],
    VERSION = 0,
    EXPECTED = "00200005F1PN5G"
}

__internal::const_test_prefixed! {
    test_prefixed_zero_two_ones_end,
    ENC = Buffer<9>,
    DEC = Buffer<6>,
    INPUT = [5, [0, 1, 0, 0, 1]],
    PREFIX = 'S',
    EXPECTED = "S0G0001"
}

__internal::const_test_check_prefixed! {
    test_check_prefixed_zero_two_ones_end,
    ENC = Buffer<17>,
    DEC = Buffer<14>,
    INPUT = [5, [0, 1, 0, 0, 1]],
    PREFIX = 'S',
    VERSION = 0,
    EXPECTED = "S00200005F1PN5G"
}
