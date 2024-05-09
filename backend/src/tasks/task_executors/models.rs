use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct TaskExecutor {
    pub task_id: Uuid,
    pub user_id: Uuid,
}
