use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mult(m: u64, n: u64) -> u64 {
    return m * n;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
