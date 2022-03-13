mod utils;

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
pub fn plus_ten() -> u32 {
    let mut arr: Vec<Vec<u32>> = vec![];
    for i in 0..1e7 as u32 {
        let mut _arr = vec![];
        for j in 0..4 {
            _arr.push(j)
        }
        arr.push(_arr)
    }

    for i in &mut arr {
        for j in i {
            *j += 10;
        }
    }

    return 1;
}
