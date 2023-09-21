#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::{println, unreachable};

use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
wasm_bindgen_test_configure!(run_in_browser);

use pen_to_todoist::utils;
use pen_to_todoist::vision_api;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
async fn fails_without_token() {
    let response =
        vision_api::ask_google_vision_api("picture-data".to_string(), "token".to_string()).await;
    assert!(response.is_err());
    let Err(e) = response else {
        unreachable!("we checked it's an error")
    };
    match &e {
        v @ JsValue => {
            let has_correct_error = v
                .as_string()
                .expect("test fails: JsValue cannot be converted to string")
                .contains("missing field `responses`");
            assert!(has_correct_error);
        }
        _ => panic!("test fails: unexpected response error-type"),
    }
}
