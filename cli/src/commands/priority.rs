use anyhow::Result;
use todo_txt_manager::TodoManager;
use todo_txt_model::{prelude::TodoTxtRsError, TaskPriority};

#[derive(Debug, Default, clap::Parser)]
pub(crate) struct PriorityArgs {
    /// Task ID to set priority
    id: usize,
    /// Priority to set.
    #[clap(group = "set_priority", default_value = "A")]
    priority: TaskPriority,
    /// Delete priority, instead of setting it
    #[clap(short, long, group = "set_priority", default_value = "false")]
    delete: bool,
}

#[tracing::instrument(parent = None, skip(manager))]
pub(crate) async fn cmd_priority(manager: &TodoManager, options: PriorityArgs) -> Result<()> {
    let priority = if options.delete {
        None
    } else {
        Some(options.priority)
    };
    tracing::info!("Setting priority at: {} {:?}", options.id, priority);
    match manager.set_priority_async(options.id, priority).await {
        Ok((id, task)) => {
            tracing::info!("Priority setted at: {} {:?}", id, task);
            Ok(())
        }
        Err(e) => {
            tracing::error!("Error: {}", e);
            match &e {
                TodoTxtRsError::InvalidIndex => {
                    eprintln!("Invalid index: {}", options.id);
                }
                e => {
                    eprintln!("Error: {}", e);
                }
            }
            Err(e.into())
        }
    }
}
