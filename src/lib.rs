#![feature(portable_simd)]

mod utils;

use core_simd::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn _fibonacci(n: u32) -> u32 {
    if n == 1 || n == 2 {
        1
    } else {
        _fibonacci(n - 1) + _fibonacci(n - 2)
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, the first webassembly project!");
}

#[wasm_bindgen]
pub fn run_fibonacci(iter: u32, len: u32) -> u32 {
    let mut total = 0;
    for _ in 0..iter {
        total = 0;
        for i in 1..len {
            total = total + _fibonacci(i);
        }
    }
    return total;
}

#[wasm_bindgen]
pub fn plus_ten(num: u32) {
    let mut array = vec![0, 0, 0, 0];
    let mut count = 0;
    loop {
        count += 1;
        for i in &mut array {
            *i += 1;
        }
        if count >= num { break; }
    }
}

#[wasm_bindgen]
pub fn plus_ten_simd(num: u32) {
    let mut a = i32x4::splat(1);
    let mut b = i32x4::from_array([0, 0, 0, 0]);
    let mut count = 0;
    loop {
        count += 1;
        b += a;
        if count >= num { break; }
    }
}
