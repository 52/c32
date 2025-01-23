// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[path = "macro.rs"]
mod macros;

gen_c32_rand_test!(test_c32_rand_dist_lo, 10_000, 10);
gen_c32_rand_test!(test_c32_rand_dist_mi, 1_000, 100);
gen_c32_rand_test!(test_c32_rand_dist_hi, 100, 1_000);
