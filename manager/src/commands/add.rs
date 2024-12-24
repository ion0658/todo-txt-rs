use crate::TodoManager;
use todo_txt_model::prelude::*;

impl TodoManager {
    #[tracing::instrument(parent = None, skip(self))]
    pub fn add(&self, mut new_task: Task) -> Result<Task> {
        let file = match new_task.state {
            TaskState::Todo => &self.todo_file,
            TaskState::Done => &self.done_file,
        };

        if new_task.created_date.is_none() {
            new_task.created_date = Some(chrono::Utc::now().date_naive());
        }
        if new_task.state == TaskState::Done && new_task.completed_date.is_none() {
            new_task.completed_date = Some(chrono::Utc::now().date_naive());
        }

        let tasks = {
            let mut tasks = crate::commands::read_tasks_from_file(file)?;
            tasks.push(new_task.clone());
            tasks
        };
        tracing::debug!("file:{:?}, all tasks: {:?}", file, tasks);
        crate::commands::write_tasks(file, &tasks)?;
        Ok(new_task)
    }

    #[cfg(any(feature = "rt_tokio", feature = "rt_async_std", feature = "rt_smol"))]
    #[tracing::instrument(parent = None, skip(self))]
    pub async fn add_async(&self, mut new_task: Task) -> Result<Task> {
        let file = match new_task.state {
            TaskState::Todo => &self.todo_file,
            TaskState::Done => &self.done_file,
        };

        if new_task.created_date.is_none() {
            new_task.created_date = Some(chrono::Utc::now().date_naive());
        }
        if new_task.state == TaskState::Done && new_task.completed_date.is_none() {
            new_task.completed_date = Some(chrono::Utc::now().date_naive());
        }

        let tasks = {
            let mut tasks = crate::commands::read_tasks_from_file_async(file).await?;
            tasks.push(new_task.clone());
            tasks
        };
        tracing::debug!("file:{:?}, all tasks: {:?}", file, tasks);
        crate::commands::write_tasks_async(file, &tasks).await?;
        Ok(new_task)
    }
}
