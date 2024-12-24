use anyhow::Result;
use todo_txt_manager::TodoManager;
use todo_txt_model::prelude::*;

#[derive(Debug, Default, clap::Parser)]
pub(crate) struct AppendArgs {
    /// Task id to append to.
    id: usize,
    /// Task string to append.
    destenations: Vec<String>,
    /// Task Projects. If task string contains project, it will be added to the task.
    #[clap(short, long, value_delimiter(','))]
    project: Option<Vec<String>>,
    /// Task contexts. If task string contains context, it will be added to the task.
    #[clap(short, long, value_delimiter(','))]
    context: Option<Vec<String>>,
}

#[tracing::instrument(parent = None, skip(manager))]
pub(crate) async fn cmd_append(manager: &TodoManager, options: AppendArgs) -> Result<()> {
    tracing::debug!("Appending task with id: {}", options.id);
    let destenation = options.destenations.join(" ");
    tracing::debug!("Appending todo: {}", destenation);
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
    if let Some(projects) = &options.project {
        task.description.project.extend_from_slice(projects);
    }
    if let Some(contexts) = &options.context {
        task.description.context.extend_from_slice(contexts);
    }
    tracing::info!("Appending task at: {}, {:?}", options.id, task);
    let (id, appended) = manager.append_async(options.id, task.description).await?;
    tracing::info!("Task appended: {}, {:?}", id, appended);
    Ok(())
}
