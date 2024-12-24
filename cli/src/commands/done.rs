use anyhow::Result;
use todo_txt_manager::TodoManager;
use todo_txt_model::prelude::*;

#[derive(Debug, Default, clap::Parser)]
pub(crate) struct DoneArgs {
    /// Task id to mark as done.
    id: usize,
}

#[tracing::instrument(parent = None, skip(manager))]
pub(crate) async fn cmd_update_state(
    manager: &TodoManager,
    options: DoneArgs,
    done: bool,
) -> Result<()> {
    tracing::info!("Update Task State at: {} {}", options.id, done);
    match manager.update_state_async(options.id, done).await {
        Ok((id, task)) => {
            tracing::info!("Task {} updated: {:?}", id, task);
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
