use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct GameInfo {
    field: Vec<u8>,
    next: Vec<u8>,
    scores: Vec<i32>,
}

#[wasm_bindgen]
pub struct Hoge {
    a: i32,
}

#[wasm_bindgen]
impl Hoge {
    pub fn new() -> Hoge {
        Hoge {
            a: 123
        }
    }

    pub fn get_a(&self) -> i32 {
        self.a
    }
}

#[wasm_bindgen]
pub fn greet() {
    "Hello, wasm-game-of-life!";
}
