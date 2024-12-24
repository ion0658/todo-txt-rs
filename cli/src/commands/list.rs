use anyhow::Result;
use todo_txt_manager::*;
use todo_txt_model::{TaskPriority, TaskState};

#[derive(Debug, Default, clap::Parser)]
pub(crate) struct ListArgs {
    /// Filter by task status
    #[clap(short, long)]
    state: Option<TaskState>,
    /// Filter by task priority.
    #[clap(short, long, value_delimiter(','))]
    priority: Option<Vec<TaskPriority>>,
    /// Filter by project. When multiple projects are provided, the task must contain all of them.
    #[clap(long, value_delimiter(','))]
    project: Option<Vec<String>>,
    /// Filter by context. When multiple contexts are provided, the task must contain all of them.
    #[clap(long, value_delimiter(','))]
    context: Option<Vec<String>>,
    /// Task order options. The option on the left has the highest priority.
    #[clap(short, long, value_delimiter(','))]
    order: Option<Vec<OrderOption>>,
    /// Reverse order of tasks
    #[clap(short, long, requires = "order", default_value = "false")]
    reverse: bool,
}

#[tracing::instrument(parent = None, skip(manager))]
pub(crate) async fn cmd_list(manager: &TodoManager, options: ListArgs) -> Result<()> {
    let filter = Filter {
        state: options.state,
        priority: options.priority,
        project: options.project,
        context: options.context,
    };
    let order = Order {
        by: options.order,
        reverse: options.reverse,
    };

    tracing::info!(
        "Listing tasks with filter: {:?}, order: {:?}",
        filter,
        order
    );
    let tasks = manager.list_async(filter, order).await?;
    tracing::info!("Listed {} tasks", tasks.len());
    for (idx, task) in tasks {
        println!("{} {}", idx, todo_txt_serializer::to_string(&task));
    }
    Ok(())
}
