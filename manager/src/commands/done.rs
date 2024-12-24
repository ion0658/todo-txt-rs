use crate::TodoManager;
use todo_txt_model::prelude::*;

impl TodoManager {
    #[tracing::instrument(parent = None, skip(self))]
    pub fn update_state(&self, task_id: usize, done: bool) -> Result<(usize, Task)> {
        let tasks = self.list(
            super::list::Filter::default(),
            super::list::Order::default(),
        )?;

        let ((task_id, mut task), tasks) = self.take_task_by_id(task_id, tasks)?;

        if done {
            task.state = TaskState::Done;
            task.completed_date = Some(chrono::Utc::now().date_naive());
        } else {
            task.state = TaskState::Todo;
            if task.created_date.is_none() {
                task.created_date = task.completed_date;
            }
            task.completed_date = None;
        }

        let tasks = self.set_task_at(task_id, task.clone(), tasks)?;
        let (todos, dones) = self.split_tasks_todo_and_done(tasks)?;
        crate::commands::write_tasks(&self.todo_file, &todos)?;
        crate::commands::write_tasks(&self.done_file, &dones)?;
        Ok((task_id, task))
    }

    #[cfg(any(feature = "rt_tokio", feature = "rt_async_std", feature = "rt_smol"))]
    #[tracing::instrument(parent = None, skip(self))]
    pub async fn update_state_async(&self, task_id: usize, done: bool) -> Result<(usize, Task)> {
        let tasks = self
            .list_async(
                super::list::Filter::default(),
                super::list::Order::default(),
            )
            .await?;
        let ((task_id, mut task), tasks) = self.take_task_by_id(task_id, tasks)?;

        if done {
            task.state = TaskState::Done;
            task.completed_date = Some(chrono::Utc::now().date_naive());
        } else {
            task.state = TaskState::Todo;
            if task.created_date.is_none() {
                task.created_date = task.completed_date;
            }
            task.completed_date = None;
        }

        let tasks = self.set_task_at(task_id, task.clone(), tasks)?;
        let (todos, dones) = self.split_tasks_todo_and_done(tasks)?;
        crate::commands::write_tasks_async(&self.todo_file, &todos).await?;
        crate::commands::write_tasks_async(&self.done_file, &dones).await?;
        Ok((task_id, task))
    }
}
