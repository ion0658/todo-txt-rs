mod commands;

use std::str::FromStr;
use todo_txt_model::prelude::*;

pub use commands::list::*;

#[derive(Debug, Clone)]
pub struct TodoManager {
    pub(crate) todo_dir: std::path::PathBuf,
    pub(crate) todo_file: std::path::PathBuf,
    pub(crate) done_file: std::path::PathBuf,
}

pub(crate) type GetTaskResult = ((usize, Task), Vec<(usize, Task)>);

impl TodoManager {
    pub fn new() -> Result<Self> {
        let _ = dotenvy::dotenv();
        let todo_dir = if let Ok(dir) = std::env::var("TODO_DIR") {
            std::path::PathBuf::from(dir)
        } else if let Ok(home) = std::env::var("XDG_DATA_HOME") {
            let path = std::path::PathBuf::from_str(&home)?;
            path.join("todo")
        } else {
            let path = std::env::current_dir()?;
            path.join(".todo")
        };
        let todo_file = todo_dir.join("todo.txt");
        let done_file = todo_dir.join("done.txt");

        Ok(Self {
            todo_dir,
            todo_file,
            done_file,
        })
    }

    pub fn get_data_dir(&self) -> &std::path::Path {
        &self.todo_dir
    }

    /// Get Task by ID
    /// note: task_id is 1-based index
    /// note: tasks is 0-based index
    /// note: this function will remove the task from the tasks
    pub(self) fn take_task_by_id(
        &self,
        task_id: usize,
        mut tasks: Vec<(usize, Task)>,
    ) -> Result<GetTaskResult> {
        if task_id > tasks.len() {
            return Err(TodoTxtRsError::InvalidIndex);
        }
        Ok((tasks.remove(task_id - 1), tasks))
    }

    pub(self) fn set_task_at(
        &self,
        task_id: usize,
        task: Task,
        mut tasks: Vec<(usize, Task)>,
    ) -> Result<Vec<(usize, Task)>> {
        if task_id - 1 > tasks.len() {
            return Err(TodoTxtRsError::InvalidIndex);
        }
        tasks.insert(task_id - 1, (task_id, task));
        Ok(tasks)
    }

    /// Split tasks into two groups: todo and done
    pub(self) fn split_tasks_todo_and_done(
        &self,
        tasks: Vec<(usize, Task)>,
    ) -> Result<(Vec<Task>, Vec<Task>)> {
        {
            #[cfg(not(feature = "parallel"))]
            {
                let todos = tasks
                    .iter()
                    .cloned()
                    .filter_map(|(_, t)| {
                        if t.state == TaskState::Todo {
                            Some(t)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let dones = tasks
                    .into_iter()
                    .filter_map(|(_, t)| {
                        if t.state == TaskState::Done {
                            Some(t)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                Ok((todos, dones))
            }
            #[cfg(feature = "parallel")]
            {
                use rayon::prelude::*;
                let todos = tasks
                    .par_iter()
                    .cloned()
                    .filter_map(|(_, t)| {
                        if t.state == TaskState::Todo {
                            Some(t)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                let dones = tasks
                    .into_par_iter()
                    .filter_map(|(_, t)| {
                        if t.state == TaskState::Done {
                            Some(t)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                Ok((todos, dones))
            }
        }
    }
}
