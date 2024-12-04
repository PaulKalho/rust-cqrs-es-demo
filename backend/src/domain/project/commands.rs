use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum ProjectCommand {
    CreateProject {
        project_id: uuid::Uuid,
        project_name: String,
        project_start: DateTime<Utc>,
        project_end: DateTime<Utc>,
        participants_name: Vec<String>,
    },
}
