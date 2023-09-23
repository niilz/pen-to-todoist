use crate::auth;
use crate::jwt;
use crate::types::vision_api as va;
use crate::types::vision_api::EntityAnnotation;
use crate::utils::{console_log, fetch};
use wasm_bindgen::prelude::*;
use web_sys::{Request, RequestInit};

const VISION_API_URL: &str = "https://vision.googleapis.com/v1/images:annotate";

pub(crate) async fn image_to_list_items(
    img_data: String,
    credentials_json: &str,
) -> Result<TodoItem, JsValue> {
    image_to_text(img_data, credentials_json, false).await
}

pub(crate) async fn image_to_single_item(
    img_data: String,
    credentials_json: &str,
) -> Result<TodoItem, JsValue> {
    image_to_text(img_data, credentials_json, true).await
}

#[derive(Debug)]
pub(crate) enum TodoItem {
    Single(String),
    List(Vec<String>),
}

async fn image_to_text(
    img_data: String,
    credentials_json: &str,
    single_todo: bool,
) -> Result<TodoItem, JsValue> {
    let jwt = jwt::create_jwt(credentials_json).expect("Could not create jwt");
    let access_token = auth::get_access_token(&jwt).await?;

    let api_res_json = ask_google_vision_api(img_data, access_token.access_token).await?;
    console_log("WASM - vision_api.rs", &"google answered with token");

    let response = api_res_json
        .responses
        .into_iter()
        .next()
        .expect("ok-response must have one element");

    let text_from_api = match (
        response.text_annotations,
        response.full_text_annotation,
        response.error,
    ) {
        (Some(text_annotations), Some(full_text_annotation), None) => {
            if single_todo {
                TodoItem::Single(find_largest_item(text_annotations))
            } else {
                TodoItem::List(
                    full_text_annotation
                        .text
                        .split_terminator('\n')
                        .map(str::to_string)
                        .collect(),
                )
            }
        }
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

fn find_largest_item(text_annotations: Vec<EntityAnnotation>) -> String {
    text_annotations
        .into_iter()
        // do not consider the entire picture (spanning over multiple lines)
        .filter(|e| !e.description.contains('\n'))
        .map(|e| (e.description, e.bounding_poly.vertices))
        .map(|(e, vs)| (e, (vs.bottom_left - vs.top_left)))
        .max_by_key(|(e, sizes)| {
            #[cfg(test)]
            console_log("finding-largest", &format!("item: {e}, sizes: {sizes:?}"));
            sizes.y
        })
        .expect("one must be the largest")
        .0
}
#[cfg(test)]
mod test {

    extern crate wasm_bindgen_test;

    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    wasm_bindgen_test_configure!(run_in_browser);

    use crate::{
        auth, jwt,
        types::vision_api::{EntityAnnotation, FullTextAnnotation, Response},
        utils,
        vision_api::find_largest_item,
    };

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
        let response = make_authenticated_test_request(b"12345").await;

        assert!(!response.is_err());
        let Ok(err) = response else {
            unreachable!("we checked it's not an error")
        };
        let Response {
            error: Some(error), ..
        } = err
        else {
            panic!("test fails: Error type was expected but");
        };
        assert_eq!(3, error.code);
        assert_eq!("Bad image data.", error.message);
    }

    #[wasm_bindgen_test]
    async fn valid_picture_gives_valid_response() {
        let mock_picture_data = include_bytes!("../test-assets/handwritten-list.png");

        let response = make_authenticated_test_request(mock_picture_data).await;

        assert!(!response.is_err());

        let (_, full_text_annotation) = extract_test_response(response);

        let expected_data = "Аму Thomas\nChelsea Cook\nJoel Nylund\nKIM TAYLOR";
        assert_eq!(expected_data, full_text_annotation.text);
    }

    #[wasm_bindgen_test]
    async fn finds_largest_item() {
        let mock_picture_data = include_bytes!("../test-assets/mythos-label.jpeg");

        let response = make_authenticated_test_request(mock_picture_data).await;

        assert!(!response.is_err());

        let (text_annotations, _) = extract_test_response(response);

        let largest_item = find_largest_item(text_annotations);
        let expected_data = "Mythos";
        assert_eq!(expected_data, largest_item);
    }

    async fn make_authenticated_test_request(mock_data: &[u8]) -> Result<Response, JsValue> {
        let jwt = jwt::create_jwt(GOOGLE_VISION_API_KEY)
            .expect("test fails: could not create jwt from credentials");
        let access_token = auth::get_access_token(&jwt)
            .await
            .expect("test fails: could not load an access_token");

        let mock_picture_data = base64::encode(mock_data);

        let response = ask_google_vision_api(mock_picture_data, access_token.access_token).await;
        utils::console_log("zero_bytes_test", &format!("${response:?}"));
        match response {
            Ok(res) => {
                let res = res
                    .responses
                    .into_iter()
                    .next()
                    .expect("test fails: 0th element should be present");
                Ok(res)
            }
            Err(e) => Err(e),
        }
    }

    fn extract_test_response(
        api_response: Result<Response, JsValue>,
    ) -> (Vec<EntityAnnotation>, FullTextAnnotation) {
        let response = api_response.expect("we checked it's not an error");
        let Response {
            text_annotations: Some(text_annotations),
            full_text_annotation: Some(full_text_annotation),
            ..
        } = response
        else {
            panic!("test fails: Error type was expected but");
        };

        (text_annotations, full_text_annotation)
    }
}
