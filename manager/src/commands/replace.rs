use crate::TodoManager;
use todo_txt_model::prelude::*;

impl TodoManager {
    #[tracing::instrument(parent = None, skip(self))]
    pub fn replace(&self, task_id: usize, mut new_task: Task) -> Result<(usize, Task, Task)> {
        let tasks = self.list(
            super::list::Filter::default(),
            super::list::Order::default(),
        )?;

        if new_task.created_date.is_none() {
            new_task.created_date = Some(chrono::Utc::now().date_naive());
        }
        if new_task.state == TaskState::Done && new_task.completed_date.is_none() {
            new_task.completed_date = Some(chrono::Utc::now().date_naive());
        }

        let ((task_id, old), tasks) = self.take_task_by_id(task_id, tasks)?;

        let tasks = self.set_task_at(task_id, new_task.clone(), tasks)?;
        let (todos, dones) = self.split_tasks_todo_and_done(tasks)?;
        crate::commands::write_tasks(&self.todo_file, &todos)?;
        crate::commands::write_tasks(&self.done_file, &dones)?;
        Ok((task_id, old, new_task))
    }

    #[cfg(any(feature = "rt_tokio", feature = "rt_async_std", feature = "rt_smol"))]
    #[tracing::instrument(parent = None, skip(self))]
    pub async fn replace_async(
        &self,
        task_id: usize,
        mut new_task: Task,
    ) -> Result<(usize, Task, Task)> {
        let tasks = self
            .list_async(
                super::list::Filter::default(),
                super::list::Order::default(),
            )
            .await?;

        if new_task.created_date.is_none() {
            new_task.created_date = Some(chrono::Utc::now().date_naive());
        }
        if new_task.state == TaskState::Done && new_task.completed_date.is_none() {
            new_task.completed_date = Some(chrono::Utc::now().date_naive());
        }

        let ((task_id, old), tasks) = self.take_task_by_id(task_id, tasks)?;

        let tasks = self.set_task_at(task_id, new_task.clone(), tasks)?;
        let (todos, dones) = self.split_tasks_todo_and_done(tasks)?;
        crate::commands::write_tasks_async(&self.todo_file, &todos).await?;
        crate::commands::write_tasks_async(&self.done_file, &dones).await?;
        Ok((task_id, old, new_task))
    }
}
