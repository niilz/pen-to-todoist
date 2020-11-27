use crate::types;
use crate::utils;
use chrono::Utc;
use js_sys::Array;
use serde::{Deserialize, Serialize};
use types::todoist::{Project, Task, TaskResponse};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, Headers, Request, RequestInit, Response};

static PROJECTS_URL: &str = "https://api.todoist.com/rest/v1/projects";
static TASKS_URL: &str = "https://api.todoist.com/rest/v1/tasks";
static SHOPPING_LIST: &str = "Einkaufsliste";

#[wasm_bindgen]
#[derive(PartialEq)]
pub struct Todoist {
    client: (),
    token: String,
    list_of_projects: Option<Vec<Project>>,
}

#[derive(Serialize, Debug, Deserialize)]
pub enum TodoistItem {
    Project(Project),
    Task(Task),
}
impl TodoistItem {
    fn to_json_body(&self) -> String {
        match self {
            Self::Project(proj) => serde_json::to_string(&proj).unwrap(),
            Self::Task(task) => serde_json::to_string(&task).unwrap(),
        }
    }
    fn get_url(&self) -> &str {
        match self {
            Self::Project(_) => PROJECTS_URL,
            Self::Task(_) => TASKS_URL,
        }
    }
}

enum TodoistItemResponse {
    A(Project),
    B(TaskResponse),
}

#[wasm_bindgen]
impl Todoist {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // let token = dotenv::var("TODOIST_TOKEN").unwrap();
        let token = "3d3698a47222e41791894ab11a71c8c912aa1b90".to_string();
        /*
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        let client = Client::builder().default_headers(headers).build().unwrap();
        */
        Todoist {
            client: (),
            token,
            list_of_projects: None,
        }
    }

    pub async fn make_shopping_list(self, items: Array) -> JsValue {
        match self.get_shopping_list_id().await {
            (Some(new_self), Some(shopping_list_id)) => {
                println!("todoist.rs/make_shopping_list(): Creating tasks.");
                let mut loop_self = new_self;
                for item in items.iter() {
                    let (ls, item_res) = loop_self
                        .create_item(TodoistItem::Task(Task::new(
                            &item.as_string().unwrap(),
                            shopping_list_id,
                        )))
                        .await
                        .expect("Could not create item. sorryyyy");
                    loop_self = ls;
                }
                JsValue::from(shopping_list_id as f64)
            }
            (Some(_), None) => JsValue::from("some-none"),
            (None, Some(_)) => JsValue::from("none-some"),
            (None, None) => JsValue::from("none-none"),
        }
    }

    async fn get_shopping_list_id(self) -> (Option<Todoist>, Option<u64>) {
        let self_with_projects = self
            .fetch_all_projects()
            .await
            .expect("Could not get all Projects");
        let mut self_to_return = None;
        let shopping_list_id = match self_with_projects.list_of_projects {
            Some(ref projects) => {
                let maybe_shopping_list = projects.iter().find(|proj| proj.name == SHOPPING_LIST);
                match maybe_shopping_list {
                    Some(list) => Some(list.id),
                    None => {
                        let (self_with_projects, new_list) = self_with_projects
                            .create_item(TodoistItem::Project(Project::new(SHOPPING_LIST)))
                            .await
                            .expect("Could not create ShoppingList");
                        match new_list {
                            TodoistItemResponse::A(proj) => {
                                self_to_return = Some(self_with_projects);
                                Some(proj.id)
                            }
                            _ => Some(42),
                        }
                    }
                }
            }
            None => None,
        };
        (self_to_return, shopping_list_id)
    }

    pub async fn fetch_all_projects(mut self) -> Option<Todoist> {
        utils::set_panic_hook();
        let mut opts = RequestInit::new();
        opts.method("GET");
        let maybe_request = Request::new_with_str_and_init(PROJECTS_URL, &opts);
        let request_with_headers = match maybe_request {
            Ok(request) => {
                request
                    .headers()
                    .set("Authorization", &format!("Bearer {}", self.token));
                request
            }
            Err(e) => return None,
        };

        let window = web_sys::window().unwrap();
        match request_with_headers {
            request => {
                let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
                match resp_value {
                    Ok(resp_val) => {
                        let resp: Response = resp_val.dyn_into().unwrap();
                        let json = JsFuture::from(resp.json().unwrap())
                            .await
                            .expect("Could not transform Json");
                        let list_of_projects: Vec<Project> = json.into_serde().unwrap();
                        self.list_of_projects = Some(list_of_projects);
                    }
                    Err(e) => return None,
                }
            }
            _ => return None,
        }
        Some(self)
    }

    async fn create_item(self, item: TodoistItem) -> Option<(Todoist, TodoistItemResponse)> {
        let mut opts = RequestInit::new();
        opts.method("POST");
        console_log("item", &item);
        let name = match &item {
            TodoistItem::Project(proj) => &proj.name,
            TodoistItem::Task(task) => &task.content,
        };
        console_log("name", &name);
        let name_json = r#"{"name": "#.to_string() + r#"""# + name + r#"""# + "}";
        console_log("name_json", &name_json);
        opts.body(Some(&JsValue::from_str(&name_json)));
        console_log("opts", &opts);

        let request = Request::new_with_str_and_init(item.get_url(), &opts)
            .expect("Could not create request");
        console_log("request before headers", &request);
        request
            .headers()
            .set("X-Request-Id", &js_sys::Date::now().to_string())
            .unwrap();
        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();
        let token = format!("Bearer {}", self.token);
        request.headers().set("Authorization", &token).unwrap();

        let window = web_sys::window().unwrap();
        console_log("request", &request);
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
        console_log("resp_value", &resp_value);
        match resp_value {
            Ok(resp_val) => {
                let resp: Response = resp_val.dyn_into().unwrap();
                console_log("json", &resp);
                let json = JsFuture::from(resp.json().unwrap())
                    .await
                    .expect("Could not transform Json");
                match item {
                    TodoistItem::Project(_) => Some((
                        self,
                        TodoistItemResponse::A(json.into_serde::<Project>().unwrap()),
                    )),
                    TodoistItem::Task(_) => Some((
                        self,
                        TodoistItemResponse::B(json.into_serde::<TaskResponse>().unwrap()),
                    )),
                }
            }
            Err(e) => return None,
        }
    }
}

fn console_log<JS>(ident: &str, value: &JS)
where
    JS: std::fmt::Debug,
{
    web_sys::console::log_1(&JsValue::from(&format!("{}: {:?}", ident, value)));
}

#[wasm_bindgen]
pub fn bla() -> String {
    "WORKS from RUST".to_string()
}
