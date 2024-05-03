use uuid::Uuid;

pub struct TaskDocument {
    pub task_id: Uuid,
    pub uri: String,
    pub name: Option<String>,
    pub description: Option<String>,
}
