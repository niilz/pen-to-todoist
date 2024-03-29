use crate::types::todoist::{Project, ProjectResponse, Task, TaskResponse};
use crate::utils;
use utils::fetch;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, Request, RequestInit, Response};

const PROJECTS_URL: &str = "https://api.todoist.com/rest/v2/projects";
const TASKS_URL: &str = "https://api.todoist.com/rest/v2/tasks";
const SHOPPING_LIST: &str = "Einkaufsliste";

pub(crate) async fn make_or_update_project<'a, I>(list_id: u32, items: I, token: &str) -> JsValue
where
    I: Iterator<Item = &'a str>,
{
    let id = if list_id == 0 {
        create_shopping_list(token).await.unwrap()
    } else {
        list_id
    };

    for item in items {
        utils::console_log("WASM - creating Task for item:", &item);
        create_task(Task::new(&item, id as u64), token)
            .await
            .expect("Could not create item. sorryyyy");
    }
    JsValue::from(id)
}

async fn create_shopping_list(token: &str) -> Option<u32> {
    let new_project = create_project(Project::new(SHOPPING_LIST), token)
        .await
        .expect("Could not create ShoppingList-Project");
    Some(new_project.id as u32)
}

async fn create_project(project: Project, token: &str) -> Option<ProjectResponse> {
    let project_json = serde_json::to_string(&project).expect("Could not convert project to json");
    let request = init_request("POST", PROJECTS_URL, Some(&project_json), token);

    let json = fetch(request).await;
    match json {
        Ok(json) => json.into_serde::<ProjectResponse>().ok(),
        Err(_) => None,
    }
}

async fn create_task(task: Task, token: &str) -> Option<TaskResponse> {
    utils::console_log("WASM - creating json for task", &task);
    let task_json = serde_json::to_string(&task).expect("Could not convert task to json");
    utils::console_log("WASM - created json", &"");
    let request = init_request("POST", TASKS_URL, Some(&task_json), token);
    utils::console_log("WASM - created-request", &"");

    let json = fetch(request).await;
    utils::console_log("WASM - sent task-request to todois-api", &"");
    match json {
        Ok(json) => match json.into_serde::<TaskResponse>() {
            Ok(task_response) => {
                utils::console_log(
                    "WASM - todoist-response to TaskResponse was successful",
                    &"",
                );
                Some(task_response)
            }
            Err(e) => {
                utils::console_log("WASM - conversion todoist-response got error:", &e);
                None
            }
        },
        Err(e) => {
            utils::console_log("WASM - error", &e);
            None
        }
    }
}

fn init_request(mode: &str, url: &str, body: Option<&str>, token: &str) -> Request {
    let mut opts = RequestInit::new();
    opts.method(mode);
    if body.is_some() {
        opts.body(Some(&JsValue::from_str(body.unwrap())));
    }
    let request = Request::new_with_str_and_init(url, &opts).expect("Could not create response");
    request
        .headers()
        .set("Authorization", &format!("Bearer {}", token))
        .unwrap();

    if mode == "POST" {
        request
            .headers()
            .set("X-Request-Id", &js_sys::Date::now().to_string())
            .unwrap();
        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();
    }
    request
}

pub(crate) async fn fetch_all_projects(token: &str) -> Option<JsValue> {
    utils::set_panic_hook();
    let request = init_request("GET", PROJECTS_URL, None, token);

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
    match resp_value {
        Ok(resp_val) => {
            let resp: Response = resp_val.dyn_into().unwrap();
            // Vec<ProjectResponse> as Json
            JsFuture::from(resp.json().unwrap()).await.ok()
        }
        Err(_) => None,
    }
}
