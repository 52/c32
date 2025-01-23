// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[path = "macro.rs"]
mod macros;

gen_c32_samples_test!(test_c32_m_32b, "samples/c32_m_100x32b.in");
gen_c32_samples_test!(test_c32_m_64b, "samples/c32_m_100x64b.in");
gen_c32_samples_test!(test_c32_m_128b, "samples/c32_m_100x128b.in");
gen_c32_samples_test!(test_c32_m_256b, "samples/c32_m_100x256b.in");
gen_c32_samples_test!(test_c32_m_512b, "samples/c32_m_100x512b.in");
gen_c32_samples_test!(test_c32_m_1k, "samples/c32_m_100x1k.in");
gen_c32_samples_test!(test_c32_m_2k, "samples/c32_m_100x2k.in");
gen_c32_samples_test!(test_c32_m_4k, "samples/c32_m_100x4k.in");

gen_c32_samples_test!(test_c32_s_32k, "samples/c32_s_32k.in");
gen_c32_samples_test!(test_c32_s_64k, "samples/c32_s_64k.in");
gen_c32_samples_test!(test_c32_s_128k, "samples/c32_s_128k.in");
gen_c32_samples_test!(test_c32_s_256k, "samples/c32_s_256k.in");
gen_c32_samples_test!(test_c32_s_512k, "samples/c32_s_512k.in");
gen_c32_samples_test!(test_c32_s_1m, "samples/c32_s_1m.in");
gen_c32_samples_test!(test_c32_s_2m, "samples/c32_s_2m.in");
gen_c32_samples_test!(test_c32_s_4m, "samples/c32_s_4m.in");
