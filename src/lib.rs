#![feature(variant_count)]
#![feature(const_mut_refs)]
#![allow(non_snake_case, non_camel_case_types, clippy::upper_case_acronyms)]

pub mod riscv;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
  fn prompt(s: &str) -> String;
}

#[wasm_bindgen]
pub fn set_panic_hook() {
  console_error_panic_hook::set_once();
}
