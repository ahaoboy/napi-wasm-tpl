#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use data_to_esm;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  data_to_esm::sum(a, b)
}
