// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]

/// A type alias for static byte arrays.
pub type Bytes<const N: usize> = &'static [u8; N];

/// Multi-sample: 100 iterations of 32-byte data.
pub const M_100X32B: Bytes<3300> = include_bytes!("../../samples/c32_m_100x32b.in");

/// Multi-sample: 100 iterations of 64-byte data.
pub const M_100X64B: Bytes<6_500> = include_bytes!("../../samples/c32_m_100x64b.in");

/// Multi-sample: 100 iterations of 128-byte data.
pub const M_100X128B: Bytes<12_900> = include_bytes!("../../samples/c32_m_100x128b.in");

/// Multi-sample: 100 iterations of 256-byte data.
pub const M_100X256B: Bytes<25_700> = include_bytes!("../../samples/c32_m_100x256b.in");

/// Multi-sample: 100 iterations of 512-byte data.
pub const M_100X512B: Bytes<51_300> = include_bytes!("../../samples/c32_m_100x512b.in");

/// Multi-sample: 100 iterations of 1KB data.
pub const M_100X1K: Bytes<102_500> = include_bytes!("../../samples/c32_m_100x1k.in");

/// Multi-sample: 100 iterations of 2KB data.
pub const M_100X2K: Bytes<204_900> = include_bytes!("../../samples/c32_m_100x2k.in");

/// Multi-sample: 100 iterations of 4KB data.
pub const M_100X4K: Bytes<409_700> = include_bytes!("../../samples/c32_m_100x4k.in");

/// Single 32KB sample.
pub const S_32K: Bytes<32_768> = include_bytes!("../../samples/c32_s_32k.in");

/// Single 64KB sample.
pub const S_64K: Bytes<65_536> = include_bytes!("../../samples/c32_s_64k.in");

/// Single 128KB sample.
pub const S_128K: Bytes<131_072> = include_bytes!("../../samples/c32_s_128k.in");

/// Single 256KB sample.
pub const S_256K: Bytes<262_144> = include_bytes!("../../samples/c32_s_256k.in");

/// Single 512KB sample.
pub const S_512K: Bytes<524_288> = include_bytes!("../../samples/c32_s_512k.in");

/// Single 1MB sample.
pub const S_1M: Bytes<1_048_576> = include_bytes!("../../samples/c32_s_1m.in");

/// Single 2MB sample.
pub const S_2M: Bytes<2_097_152> = include_bytes!("../../samples/c32_s_2m.in");

/// Single 4MB sample.
pub const S_4M: Bytes<4_194_304> = include_bytes!("../../samples/c32_s_4m.in");

/// All available benchmark samples.
pub const ALL: [(&str, &[u8]); 16] = [
    ("m_100x32b", M_100X32B),
    ("m_100x64b", M_100X64B),
    ("m_100x128b", M_100X128B),
    ("m_100x256b", M_100X256B),
    ("m_100x512b", M_100X512B),
    ("m_100x1k", M_100X1K),
    ("m_100x2k", M_100X2K),
    ("m_100x4k", M_100X4K),
    ("s_32k", S_32K),
    ("s_64k", S_64K),
    ("s_128k", S_128K),
    ("s_256k", S_256K),
    ("s_512k", S_512K),
    ("s_1m", S_1M),
    ("s_2m", S_2M),
    ("s_4m", S_4M),
];
