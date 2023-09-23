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

    #[derive(Deserialize, Debug)]
    pub struct Responses {
        pub responses: Vec<Response>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Response {
        pub text_annotations: Option<Vec<EntityAnnotation>>,
        pub full_text_annotation: Option<FullTextAnnotation>,
        pub error: Option<ApiError>,
    }

    #[derive(Deserialize, Debug)]
    pub struct FullTextAnnotation {
        // only interested in final text for now
        pub text: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct EntityAnnotation {
        pub description: String,
        pub bounding_poly: BoundingPoly,
    }

    #[derive(Deserialize, Debug)]
    pub struct BoundingPoly {
        pub vertices: Vertices,
    }

    #[derive(Deserialize, Debug)]
    pub struct Vertices {
        pub top_left: VerticeOption,
        pub top_right: VerticeOption,
        pub bottom_right: VerticeOption,
        pub bottom_left: VerticeOption,
    }

    #[derive(Deserialize, Debug)]
    pub struct VerticeOption {
        pub x: Option<u32>,
        pub y: Option<u32>,
    }

    #[derive(Debug)]
    pub struct Vertice {
        pub x: u32,
        pub y: u32,
    }
    impl std::ops::Sub for VerticeOption {
        type Output = Vertice;
        fn sub(self, other: Self) -> Self::Output {
            Vertice {
                x: self.x.unwrap_or(0).saturating_sub(other.x.unwrap_or(0)),
                y: self.y.unwrap_or(0).saturating_sub(other.y.unwrap_or(0)),
            }
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct ApiError {
        // only interested in final text for now
        pub code: u32,
        pub message: String,
    }
}

pub mod todoist {
    use serde::{Deserialize, Serialize};

    // ###### todoist types #######
    #[derive(Debug, Serialize)]
    pub struct Project {
        name: String,
    }
    impl Project {
        pub fn new(name: &str) -> Self {
            Project {
                name: name.to_string(),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct ProjectResponse {
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

    #[derive(Debug, Deserialize, Serialize, Default)]
    pub struct Task {
        content: String,
        project_id: Option<u64>,
        label_ids: Vec<u32>,
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
    pub(crate) struct TaskResponse {
        id: String,
        assigner_id: Option<String>,
        assignee_id: Option<String>,
        project_id: String,
        section_id: Option<String>,
        parent_id: Option<String>,
        order: u32,
        content: String,
        description: String,
        is_completed: bool,
        labels: Vec<String>,
        priority: u8,
        comment_count: u32,
        creator_id: Option<String>,
        created_at: String,
        due: Option<Due>,
        url: String,
    }

    #[derive(Debug, Deserialize, Default)]
    pub struct Due {
        string: String,
        date: String,
        is_recurring: bool,
        datetime: String,
        timezone: String,
    }
}
