use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Employee {
    pub name: String,
    pub duration: f32,
    pub task_name: String,
    pub date: NaiveDateTime,
    pub description: String,
}
