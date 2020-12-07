use crate::types::todoist::{Project, ProjectResponse, Task, TaskResponse};
use crate::utils;
use utils::fetch;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, Request, RequestInit, Response};

const PROJECTS_URL: &str = "https://api.todoist.com/rest/v1/projects";
const TASKS_URL: &str = "https://api.todoist.com/rest/v1/tasks";
const SHOPPING_LIST: &str = "Einkaufsliste";

pub(crate) async fn make_shopping_list<'a, I>(items: I, token: &str) -> JsValue
where
    I: Iterator<Item = &'a str>,
{
    match get_shopping_list_id(token).await {
        Some(shopping_list_id) => {
            for item in items {
                create_task(Task::new(&item, shopping_list_id), token)
                    .await
                    .expect("Could not create item. sorryyyy");
            }
            JsValue::from(shopping_list_id as f64)
        }
        None => JsValue::NULL,
    }
}

async fn get_shopping_list_id(token: &str) -> Option<u64> {
    let projects: Vec<ProjectResponse> = fetch_all_projects(token)
        .await
        .expect("Could not get all Projects");
    let maybe_shopping_list = projects.iter().find(|proj| proj.name == SHOPPING_LIST);
    match maybe_shopping_list {
        Some(list) => Some(list.id),
        None => {
            let new_project = create_project(Project::new(SHOPPING_LIST), token)
                .await
                .expect("Could not create ShoppingList-Project");
            Some(new_project.id)
        }
    }
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
    let task_json = serde_json::to_string(&task).expect("Could not convert task to json");
    let request = init_request("POST", TASKS_URL, Some(&task_json), token);

    let json = fetch(request).await;
    match json {
        Ok(json) => json.into_serde::<TaskResponse>().ok(),
        Err(_) => None,
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

async fn fetch_all_projects(token: &str) -> Option<Vec<ProjectResponse>> {
    utils::set_panic_hook();
    let request = init_request("GET", PROJECTS_URL, None, token);

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
    match resp_value {
        Ok(resp_val) => {
            let resp: Response = resp_val.dyn_into().unwrap();
            let json = JsFuture::from(resp.json().unwrap())
                .await
                .expect("Could not transform Json");
            let list_of_projects: Vec<ProjectResponse> = json.into_serde().unwrap();
            Some(list_of_projects)
        }
        Err(_) => None,
    }
}
