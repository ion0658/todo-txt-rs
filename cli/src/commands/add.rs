use anyhow::Result;
use todo_txt_manager::TodoManager;
use todo_txt_model::prelude::*;

#[derive(Debug, Default, clap::Parser)]
pub(crate) struct AddArgs {
    /// Task string to add.
    destenations: Vec<String>,
    /// Task status. If task string contains status, it will be ignored.
    #[clap(short, long)]
    state: Option<TaskState>,
    /// Task priority. If task string contains priority, it will be ignored.
    #[clap(short, long)]
    priority: Option<TaskPriority>,
    /// Task Projects. If task string contains project, it will be added to the task.
    #[clap(long, value_delimiter(','))]
    project: Option<Vec<String>>,
    /// Task contexts. If task string contains context, it will be added to the task.
    #[clap(long, value_delimiter(','))]
    context: Option<Vec<String>>,
}

#[tracing::instrument(parent = None, skip(manager))]
pub(crate) async fn cmd_add(manager: &TodoManager, options: AddArgs) -> Result<()> {
    let destenation = options.destenations.join(" ");
    tracing::debug!("Adding todo: {}", destenation);
    let result = todo_txt_serializer::from_str(&destenation);
    let mut task = match result {
        Ok(task) => task,
        Err(e) => {
            tracing::error!("Failed to parse task: {}", e);
            if let TodoTxtRsError::Syntax = e {
                eprintln!("Invalid task syntax: {}", destenation)
            }
            return Err(e.into());
        }
    };
    tracing::debug!("Parsed task: {:?}", task);
    if task.state == TaskState::Todo {
        if let Some(state) = options.state {
            task.state = state;
        }
    }
    if task.priority.is_none() {
        task.priority = options.priority;
    }
    if let Some(projects) = &options.project {
        task.description.project.extend_from_slice(projects);
    }
    if let Some(contexts) = &options.context {
        task.description.context.extend_from_slice(contexts);
    }
    tracing::info!("Adding task: {:?}", task);
    let added = manager.add_async(task).await?;
    tracing::info!("Task added: {:?}", added);
    Ok(())
}
