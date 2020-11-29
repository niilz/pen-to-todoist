use crate::auth;
use crate::jwt;
use crate::types::vision_api as va;
use crate::utils::console_log;
use std::io;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn img_data_to_string(img_data: String) -> Result<JsValue, JsValue> {
    let jwt = jwt::create_jwt().expect("Could not create jwt");
    let access_token_res = auth::get_access_token(&jwt).await;
    match access_token_res {
        Ok(token) => console_log("YES TOKEN", &token),
        Err(e) => console_log("schade", &e),
    }

    /*
    let api_res = ask_google_vision_api(img_data, access_token_res.access_token, &client)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    // println!("Google-api-res: {}", api_res);
    let res_as_json = serde_json::from_str::<va::Responses>(&api_res)?;
    let text_from_api = res_as_json.responses[0].full_text_annotation.text.clone();
    println!(
        "vision_api.rs/img_data_to_string(): Retrieving digitized list-items from JSON-response."
    );
    println!(
        "vision_api.rs/img_data_to_string(): The list-items are: {}",
        text_from_api
    );
    Ok(text_from_api)
    */
    Ok(JsValue::UNDEFINED)
}

/*
pub async fn ask_google_vision_api(
    img_data: String,
    access_token: String,
    client: &Client,
) -> Result<String, reqwest::Error> {
    //let image_64 = base64::encode(img_data);
    let requests = va::Requests::from(img_data);

    let res = client
        .post("https://vision.googleapis.com/v1/images:annotate")
        .header(header::CONTENT_TYPE, "application/json")
        .bearer_auth(access_token)
        .body(serde_json::to_string(&requests).unwrap())
        .send()
        .await?;
    println!("vision_api.rs/ask_google_vision_api(): Sent image and access-token to vision-API and got response: {}", res.status());
    // println!("Api-res: {:#?}", res);
    let res_body = res.text().await;
    // println!("Res-Body-Text: {:#?}", res_body);
    res_body
}
*/
