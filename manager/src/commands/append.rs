use crate::TodoManager;
use todo_txt_model::prelude::*;

impl TodoManager {
    #[tracing::instrument(parent = None, skip(self))]
    pub fn append(&self, task_id: usize, description: TaskDescription) -> Result<(usize, Task)> {
        let tasks = self.list(
            super::list::Filter::default(),
            super::list::Order::default(),
        )?;

        let ((task_id, mut task), tasks) = self.take_task_by_id(task_id, tasks)?;

        if !description.value.is_empty() {
            task.description
                .value
                .push(todo_txt_serializer::TOKEN_SEPARATOR);
            task.description.value.push_str(&description.value);
        }
        task.description.project.extend(description.project);
        task.description.context.extend(description.context);

        let tasks = self.set_task_at(task_id, task.clone(), tasks)?;
        let (todos, dones) = self.split_tasks_todo_and_done(tasks)?;
        crate::commands::write_tasks(&self.todo_file, &todos)?;
        crate::commands::write_tasks(&self.done_file, &dones)?;
        Ok((task_id, task))
    }

    #[cfg(any(feature = "rt_tokio", feature = "rt_async_std", feature = "rt_smol"))]
    #[tracing::instrument(parent = None, skip(self))]
    pub async fn append_async(
        &self,
        task_id: usize,
        description: TaskDescription,
    ) -> Result<(usize, Task)> {
        let tasks = self
            .list_async(
                super::list::Filter::default(),
                super::list::Order::default(),
            )
            .await?;

        let ((task_id, mut task), tasks) = self.take_task_by_id(task_id, tasks)?;

        if !description.value.is_empty() {
            task.description
                .value
                .push(todo_txt_serializer::TOKEN_SEPARATOR);
            task.description.value.push_str(&description.value);
        }
        task.description.project.extend(description.project);
        task.description.context.extend(description.context);

        let tasks = self.set_task_at(task_id, task.clone(), tasks)?;
        let (todos, dones) = self.split_tasks_todo_and_done(tasks)?;
        crate::commands::write_tasks_async(&self.todo_file, &todos).await?;
        crate::commands::write_tasks_async(&self.done_file, &dones).await?;
        Ok((task_id, task))
    }
}
