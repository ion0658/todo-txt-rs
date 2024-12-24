use crate::TodoManager;
use todo_txt_model::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub state: Option<TaskState>,
    pub priority: Option<Vec<TaskPriority>>,
    pub project: Option<Vec<String>>,
    pub context: Option<Vec<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[clap(rename_all = "snake_case")]
pub enum OrderOption {
    State,
    Priority,
    CompletedDate,
    CreatedDate,
}
#[derive(Debug, Clone, Default)]
pub struct Order {
    pub by: Option<Vec<OrderOption>>,
    pub reverse: bool,
}

impl TodoManager {
    #[tracing::instrument(parent = None, skip(self))]
    pub fn list(&self, filter: Filter, order: Order) -> Result<Vec<(usize, Task)>> {
        let tasks = {
            let mut todos = crate::commands::read_tasks_from_file(&self.todo_file)?;
            let dones = crate::commands::read_tasks_from_file(&self.done_file)?;
            todos.extend(dones);
            todos
        };
        tracing::debug!("tasks before filter: {:?}", tasks);
        let task_enum = {
            #[cfg(feature = "parallel")]
            {
                use rayon::prelude::*;
                tasks
                    .into_par_iter()
                    .enumerate()
                    .map(|(idx, t)| (idx + 1, t))
                    .collect()
            }
            #[cfg(not(feature = "parallel"))]
            {
                tasks
                    .into_iter()
                    .enumerate()
                    .map(|(idx, t)| (idx + 1, t))
                    .collect()
            }
        };
        Ok(self.sort_tasks(self.filter_tasks(task_enum, filter), order))
    }

    #[cfg(any(feature = "rt_async_std", feature = "rt_tokio", feature = "rt_smol"))]
    #[tracing::instrument(parent = None, skip(self))]
    pub async fn list_async(&self, filter: Filter, order: Order) -> Result<Vec<(usize, Task)>> {
        let tasks = {
            let mut todos = crate::commands::read_tasks_from_file_async(&self.todo_file).await?;
            let dones = crate::commands::read_tasks_from_file_async(&self.done_file).await?;
            todos.extend(dones);
            todos
        };
        tracing::debug!("tasks before filter: {:?}", tasks);
        let task_enum = {
            #[cfg(feature = "parallel")]
            {
                use rayon::prelude::*;
                tasks
                    .into_par_iter()
                    .enumerate()
                    .map(|(idx, t)| (idx + 1, t))
                    .collect()
            }
            #[cfg(not(feature = "parallel"))]
            {
                tasks
                    .into_iter()
                    .enumerate()
                    .map(|(idx, t)| (idx + 1, t))
                    .collect()
            }
        };
        Ok(self.sort_tasks(self.filter_tasks(task_enum, filter), order))
    }
}

impl TodoManager {
    #[tracing::instrument(parent = None, skip(self, tasks))]
    fn filter_tasks(&self, tasks: Vec<(usize, Task)>, filter: Filter) -> Vec<(usize, Task)> {
        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;
            tasks
                .into_par_iter()
                .filter(|(_, t)| {
                    filter.state.is_none_or(|f| t.state == f)
                        && filter
                            .priority
                            .as_ref()
                            .is_none_or(|filter| t.priority.is_some_and(|t| filter.contains(&t)))
                        && filter.project.as_ref().is_none_or(|filter| {
                            filter.par_iter().all(|p| t.description.project.contains(p))
                        })
                        && filter.context.as_ref().is_none_or(|filter| {
                            filter.par_iter().all(|c| t.description.context.contains(c))
                        })
                })
                .collect()
        }
        #[cfg(not(feature = "parallel"))]
        {
            tasks
                .into_iter()
                .filter(|(_, t)| {
                    filter.state.is_none_or(|f| t.state == f)
                        && filter.project.as_ref().is_none_or(|filter| {
                            filter.iter().all(|p| t.description.project.contains(p))
                        })
                        && filter.context.as_ref().is_none_or(|filter| {
                            filter.iter().all(|c| t.description.context.contains(c))
                        })
                })
                .collect()
        }
    }

    #[tracing::instrument(parent = None, skip(self, tasks))]
    fn sort_tasks(&self, mut tasks: Vec<(usize, Task)>, order: Order) -> Vec<(usize, Task)> {
        #[cfg(feature = "parallel")]
        use rayon::prelude::*;
        if let Some(mut orderlist) = order.by {
            orderlist.reverse();
            fn compare_by(
                (_, a): &(usize, Task),
                (_, b): &(usize, Task),
                by: OrderOption,
            ) -> std::cmp::Ordering {
                match by {
                    OrderOption::State => a.state.cmp(&b.state),
                    OrderOption::Priority => a.priority.cmp(&b.priority),
                    OrderOption::CompletedDate => b.completed_date.cmp(&a.completed_date),
                    OrderOption::CreatedDate => b.created_date.cmp(&a.created_date),
                }
            }
            for by in orderlist {
                #[cfg(feature = "parallel")]
                {
                    tasks.par_sort_by(|a, b| compare_by(a, b, by));
                }
                #[cfg(not(feature = "parallel"))]
                {
                    tasks.sort_by(|a, b| compare_by(a, b, by));
                }
            }
            if order.reverse {
                tasks.reverse();
            }
            tasks
        } else {
            tasks
        }
    }
}
