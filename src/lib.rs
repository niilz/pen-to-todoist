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

use todoist::{fetch_all_projects, make_or_update_project};
use vision_api::{image_to_list_items, image_to_single_item, TodoItem};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn list_from_handwriting(
    project_id: u32,
    img_data: String,
    todoist_token: String,
    credentials_json: String,
) -> JsValue {
    todoist_from_handwriting(project_id, img_data, todoist_token, credentials_json, false).await
}

#[wasm_bindgen]
pub async fn largest_item_from_handwriting(
    project_id: u32,
    img_data: String,
    todoist_token: String,
    credentials_json: String,
) -> JsValue {
    todoist_from_handwriting(project_id, img_data, todoist_token, credentials_json, true).await
}

async fn todoist_from_handwriting(
    project_id: u32,
    img_data: String,
    todoist_token: String,
    credentials_json: String,
    single_todo: bool,
) -> JsValue {
    utils::console_log("project_id u32", &project_id);
    let image_data = if single_todo {
        image_to_single_item(img_data, &credentials_json).await
    } else {
        image_to_list_items(img_data, &credentials_json).await
    };

    match image_data {
        Ok(TodoItem::List(list)) => {
            make_or_update_project(project_id, list.iter().map(|s| s.as_str()), &todoist_token)
                .await
        }
        Ok(TodoItem::Single(item)) => {
            make_or_update_project(project_id, std::iter::once(item.as_str()), &todoist_token).await
        }
        Err(e) => {
            utils::console_log("Error", &e);
            JsValue::null()
        }
    }
}

#[wasm_bindgen]
pub async fn get_all_projects(todoist_token: String) -> JsValue {
    match fetch_all_projects(&todoist_token).await {
        Some(projects) => projects,
        None => JsValue::NULL,
    }
}
