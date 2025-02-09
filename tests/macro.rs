// © 2025 Max Karou. All Rights Reserved.
// Licensed under Apache Version 2.0, or MIT License, at your discretion.
//
// Apache License: http://www.apache.org/licenses/LICENSE-2.0
// MIT License: http://opensource.org/licenses/MIT
//
// Usage of this file is permitted solely under a sanctioned license.

#[macro_export]
macro_rules! test_cases {
   ($name:ident, fn($($param:ident: $ty:ty),*) $body:block, $($test_name:literal: ($($arg:expr),*)),* $(,)?) => {
       paste::paste! {
           mod $name {
               #[allow(unused_imports)]
               use super::*;
               fn test_fn($($param: $ty),*) $body
               $(#[test] fn [<$test_name>]() { test_fn($($arg),*); })*
           }
       }
   };
}
