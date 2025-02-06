// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[path = "macro.rs"]
mod macros;

c32_rand_test!(test_c32_rand_lo, 10_000, 10);
c32_rand_test!(test_c32_rand_mi, 1_000, 100);
c32_rand_test!(test_c32_rand_hi, 100, 1_000);
