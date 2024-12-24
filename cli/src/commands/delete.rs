use anyhow::Result;
use todo_txt_manager::TodoManager;
use todo_txt_model::prelude::TodoTxtRsError;

#[derive(Debug, Default, clap::Parser)]
pub(crate) struct DeleteArgs {
    /// Task ID to delete
    id: usize,
}

#[tracing::instrument(parent = None, skip(manager))]
pub(crate) async fn cmd_delete(manager: &TodoManager, options: DeleteArgs) -> Result<()> {
    tracing::info!("Deleting task at: {}", options.id);
    match manager.delete_async(options.id).await {
        Ok(deleted) => {
            tracing::info!("Deleted task: {:?}", deleted);
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
