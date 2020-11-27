pub mod vision_api {

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Default)]
    pub struct Requests {
        requests: Vec<Request>,
    }
    impl From<String> for Requests {
        fn from(data: String) -> Self {
            Requests {
                requests: vec![Request::from(data)],
            }
        }
    }

    #[derive(Serialize, Default)]
    pub struct Request {
        image: Content,
        features: Vec<Item>,
    }
    impl From<String> for Request {
        fn from(data: String) -> Self {
            Request {
                image: Content::new(data),
                features: vec![Item::default()],
            }
        }
    }

    #[derive(Serialize, Default)]
    pub struct Content {
        content: String,
    }
    impl Content {
        fn new(data: String) -> Self {
            Content { content: data }
        }
    }

    #[derive(Serialize)]
    pub struct Item {
        r#type: String,
    }
    impl Default for Item {
        fn default() -> Self {
            Item {
                r#type: "TEXT_DETECTION".to_string(),
            }
        }
    }

    #[derive(Deserialize)]
    pub struct Responses {
        pub responses: Vec<Response>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub full_text_annotation: FullTextAnnotation,
    }

    #[derive(Deserialize)]
    pub struct FullTextAnnotation {
        // only interested in final text for now
        pub text: String,
    }
}

pub mod todoist {
    use serde::{Deserialize, Serialize};
    // ###### todoist types #######
    #[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
    pub struct Project {
        pub id: u64,
        pub name: String,
        pub comment_count: Option<u32>,
        pub order: Option<u32>,
        pub color: Option<u32>,
        pub shared: Option<bool>,
        pub sync_id: Option<u32>,
        pub favorite: Option<bool>,
        pub inbox_project: Option<bool>,
    }
    impl Project {
        pub fn new(name: &str) -> Self {
            let mut proj = Project::default();
            proj.name = name.to_string();
            proj
        }
    }

    #[derive(Debug, Deserialize, Serialize, Default)]
    pub struct Task {
        pub content: String,
        project_id: Option<u64>,
        label_ids: Option<Vec<u32>>,
        // only one due_* can be used
        due_string: Option<String>,
        due_date: Option<String>,
        due_datetime: Option<String>,
    }
    impl Task {
        pub fn new(content: &str, project_id: u64) -> Self {
            let mut task = Task::default();
            task.content = content.to_string();
            task.project_id = Some(project_id);
            task
        }
    }

    #[derive(Debug, Deserialize, Default)]
    pub struct TaskResponse {
        comment_count: u32,
        completed: bool,
        content: String,
        due: Option<Due>,
        id: u64,
        order: u32,
        priority: u32,
        project_id: u32,
        section_id: u32,
        parent_id: Option<u32>,
        url: String,
    }

    #[derive(Debug, Deserialize, Default)]
    pub struct Due {
        date: String,
        datetime: String,
        string: String,
        timezone: String,
    }
}
