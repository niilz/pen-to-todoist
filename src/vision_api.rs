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

    let response = &api_res_json.responses[0];

    let text_from_api = match (
        &response.text_annotations,
        &response.full_text_annotation,
        &response.error,
    ) {
        (Some(_ta), Some(fta), None) => fta.text.clone(),
        (None, None, Some(error)) => return Err(JsValue::from_str(&error.message)),
        _ => return Err(JsValue::from_str("unexpected structure")),
    };
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

    #[cfg(test)]
    console_log("WASM - vision_api.rs", &format!("got response: ${res:?}"));

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

    use crate::{auth, jwt, types::vision_api::Response, utils};

    use super::ask_google_vision_api;
    use wasm_bindgen_test::wasm_bindgen_test;
    const GOOGLE_VISION_API_KEY: &str = include_str!("../vision-api-key.json");

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

    #[wasm_bindgen_test]
    async fn bad_image_data_gives_error_response() {
        let jwt = jwt::create_jwt(GOOGLE_VISION_API_KEY)
            .expect("test fails: could not create jwt from credentials");
        let access_token = auth::get_access_token(&jwt)
            .await
            .expect("test fails: could not load an access_token");

        let mock_picture_data = base64::encode("12345");

        let response = ask_google_vision_api(mock_picture_data, access_token.access_token).await;
        utils::console_log("zero_bytes_test", &format!("${response:?}"));
        assert!(!response.is_err());
        let Ok(err) = response else {
            unreachable!("we checked it's not an error")
        };
        let Response {
            error: Some(error), ..
        } = err
            .responses
            .get(0)
            .expect("test fails: 0th element should be present")
        else {
            panic!("test fails: Error type was expected but");
        };
        assert_eq!(3, error.code);
        assert_eq!("Bad image data.", error.message);
    }

    #[wasm_bindgen_test]
    async fn valid_picture_gives_valid_response() {
        let jwt = jwt::create_jwt(GOOGLE_VISION_API_KEY)
            .expect("test fails: could not create jwt from credentials");
        let access_token = auth::get_access_token(&jwt)
            .await
            .expect("test fails: could not load an access_token");

        let mock_picture_data = include_bytes!("../handwritten-list.png");
        let mock_picture_data = base64::encode(mock_picture_data);

        let response = ask_google_vision_api(mock_picture_data, access_token.access_token).await;
        utils::console_log("valid_picture_test", &format!("${response:?}"));

        assert!(!response.is_err());

        let Ok(response) = response else {
            unreachable!("we checked it's not an error")
        };
        let Response {
            text_annotations: Some(_ta),
            full_text_annotation: Some(fta),
            ..
        } = response
            .responses
            .get(0)
            .expect("test fails: 0th element should be present")
        else {
            panic!("test fails: Error type was expected but");
        };
        let expected_data = "Аму Thomas\nChelsea Cook\nJoel Nylund\nKIM TAYLOR";
        assert_eq!(expected_data, fta.text);
    }
}
