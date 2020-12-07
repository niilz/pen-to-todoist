// Auth is done through JWT tokens!
// Those were the steps to enable Vision API:
// - Create account
// - Create project
// - Download credentials json
// - Set GOOGLE_APPLICATION_CREDENTIALS env var to access the json-file
// - Install gcloud client
// - Enable Vision API on google cloud platform
// - Activate Vision API for your project using the gcloud cli tool: "gcloud auth activate-service-account --key-file KEY_FILE"
// More infos under: https://cloud.google.com/vision/product-search/docs/auth
// - Create JWT and send it to GoogleCloud, requesting acces_token service_account
// - Use retrieved access_token as bearer to make requests to Vision API

pub mod auth;
pub mod jwt;
pub mod todoist;
pub mod types;
pub mod utils;
pub mod vision_api;

use todoist::make_shopping_list;
use vision_api::img_data_to_string;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn todoist_from_handwriting(
    img_data: String,
    todoist_token: String,
    credentials_json: String,
) -> JsValue {
    let list_as_string = img_data_to_string(img_data, &credentials_json).await;
    match list_as_string {
        Ok(list) => {
            let digital_list = list.split_terminator('\n');
            make_shopping_list(digital_list, &todoist_token).await
        }
        Err(e) => {
            utils::console_log("Error", &e);
            JsValue::null()
        }
    }
}
