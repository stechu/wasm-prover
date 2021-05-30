#![feature(test)]

mod bench;
use wasm_bindgen::prelude::*;

//#[wasm_bindgen]
//extern {
//    pub fn alert(s: &str);
//}

#[wasm_bindgen]
pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        x => fibonacci(x - 1) + fibonacci(x - 2),
    }
}
