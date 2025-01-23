mod calculator;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate(input: String) -> Result<String, JsValue> {
    match calculator::parser::calculate(&input) {
        Ok(result) => Ok(result.to_string()), // 成功时返回结果
        Err(err) => Err(JsValue::from_str(&err.to_string())), // 错误时返回错误信息
    }
}
