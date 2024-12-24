use crate::TodoManager;
use todo_txt_model::prelude::*;

impl TodoManager {
    #[tracing::instrument(parent = None, skip(self))]
    pub fn delete(&self, task_id: usize) -> Result<Task> {
        let tasks = self.list(
            super::list::Filter::default(),
            super::list::Order::default(),
        )?;
        let ((_, deleted), tasks) = self.take_task_by_id(task_id, tasks)?;

        let (todos, dones) = self.split_tasks_todo_and_done(tasks)?;
        crate::commands::write_tasks(&self.todo_file, &todos)?;
        crate::commands::write_tasks(&self.done_file, &dones)?;
        Ok(deleted)
    }

    #[cfg(any(feature = "rt_tokio", feature = "rt_async_std", feature = "rt_smol"))]
    #[tracing::instrument(parent = None, skip(self))]
    pub async fn delete_async(&self, task_id: usize) -> Result<Task> {
        let tasks = self
            .list_async(
                super::list::Filter::default(),
                super::list::Order::default(),
            )
            .await?;

        let ((_, deleted), tasks) = self.take_task_by_id(task_id, tasks)?;

        let (todos, dones) = self.split_tasks_todo_and_done(tasks)?;
        crate::commands::write_tasks_async(&self.todo_file, &todos).await?;
        crate::commands::write_tasks_async(&self.done_file, &dones).await?;
        Ok(deleted)
    }
}
