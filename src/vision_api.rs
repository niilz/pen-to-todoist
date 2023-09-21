use crate::auth;
use crate::jwt;
use crate::types::vision_api as va;
use crate::utils::{console_log, fetch};
use wasm_bindgen::prelude::*;
use web_sys::{Request, RequestInit};

const VISION_API_URL: &str = "https://vision.googleapis.com/v1/images:annotate";

pub(crate) async fn img_data_to_string(
    img_data: String,
    credentials_json: &str,
) -> Result<String, JsValue> {
    let jwt = jwt::create_jwt(credentials_json).expect("Could not create jwt");
    let access_token = auth::get_access_token(&jwt).await?;

    let api_res_json = ask_google_vision_api(img_data, access_token.access_token).await?;
    console_log("WASM - vision_api.rs", &"google answered with token");

    let text_from_api = api_res_json.responses[0].full_text_annotation.text.clone();
    console_log("WASM - vision_api.rs", &text_from_api);
    Ok(text_from_api)
}

pub(crate) async fn ask_google_vision_api(
    img_data: String,
    access_token: String,
) -> Result<va::Responses, JsValue> {
    let requests_obj = va::Requests::from(img_data);
    let request_obj_json = serde_json::to_string(&requests_obj).unwrap();
    console_log(
        "WASM - vision_api.rs",
        &"constructed image-translate-request-object",
    );

    let request = init_request(&request_obj_json, &access_token);

    let res = fetch(request).await?;
    console_log(
        "WASM - vision_api.rs",
        &"Sent image and access-token to vision-API and got response",
    );

    match res.into_serde::<va::Responses>() {
        Ok(api_data_json) => Ok(api_data_json),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

fn init_request(request_obj: &str, access_token: &str) -> Request {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(request_obj)));
    let request =
        Request::new_with_str_and_init(VISION_API_URL, &opts).expect("Could not create response");
    request
        .headers()
        .set("Authorization", &format!("Bearer {}", access_token))
        .unwrap();
    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();
    request
}

#[cfg(test)]
mod test {

    extern crate wasm_bindgen_test;

    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use super::ask_google_vision_api;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    async fn fails_without_token() {
        let response = ask_google_vision_api("picture-data".to_string(), "token".to_string()).await;
        assert!(response.is_err());
        let Err(e) = response else {
            unreachable!("we checked it's an error")
        };
        let has_correct_error = e
            .as_string()
            .expect("test fails: JsValue cannot be converted to string")
            .contains("missing field `responses`");
        assert!(has_correct_error);
    }
}
