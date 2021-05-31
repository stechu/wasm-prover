#![feature(test)]

mod bench;
pub mod groth_api;
pub mod pedersen;
pub mod r1cs;
pub mod prover;
pub mod zk_params;
pub mod pedersen_params;

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

#[wasm_bindgen]
pub fn zk_prove() {
    prover::prover();
}