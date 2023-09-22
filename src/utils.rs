use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub async fn fetch(request: Request) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
    match resp_value {
        Ok(resp_value) => {
            let resp: Response = resp_value.dyn_into().unwrap();
            JsFuture::from(resp.json().unwrap()).await
        }
        Err(e) => Err(e),
    }
}

pub fn console_log<JS>(ident: &str, value: &JS)
where
    JS: std::fmt::Debug,
{
    web_sys::console::log_1(&JsValue::from(&format!("{}: {:?}", ident, value)));
}
