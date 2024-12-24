mod commands;

use anyhow::Result;
use clap::Parser;
use commands::*;
use tracing::level_filters::LevelFilter;

fn init_log(todo_dir: &std::path::Path) -> Result<(), todo_txt_model::prelude::TodoTxtRsError> {
    if cfg!(debug_assertions) {
        const LOG_LEVEL: LevelFilter = LevelFilter::DEBUG;
        tracing_subscriber::fmt()
            .with_max_level(LOG_LEVEL)
            .with_file(true)
            .with_line_number(true)
            .init();
    } else {
        const LOG_LEVEL: LevelFilter = LevelFilter::INFO;
        let log_writer = tracing_appender::rolling::daily(todo_dir, "todo.log");
        tracing_subscriber::fmt()
            .with_writer(log_writer)
            .with_max_level(LOG_LEVEL)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(false)
            .init();
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let manager = todo_txt_manager::TodoManager::new()?;
    let data_path = manager.get_data_dir();
    init_log(data_path)?;
    tracing::info!("args: {:?}", args);
    tracing::debug!("manager: {:?}", manager);
    if !data_path.exists() {
        std::fs::create_dir_all(data_path)?;
    }

    let mut need_show_after = true;
    match args.subcmd {
        Some(SubCommand::List(options)) => {
            need_show_after = false;
            commands::cmd_list(&manager, options).await?
        }
        Some(SubCommand::Add(options)) => commands::cmd_add(&manager, options).await?,
        Some(SubCommand::Done(options)) => {
            commands::cmd_update_state(&manager, options, true).await?
        }
        Some(SubCommand::Undone(options)) => {
            commands::cmd_update_state(&manager, options, false).await?
        }
        Some(SubCommand::Delete(options)) => commands::cmd_delete(&manager, options).await?,
        Some(SubCommand::Priority(options)) => commands::cmd_priority(&manager, options).await?,
        Some(SubCommand::Append(options)) => commands::cmd_append(&manager, options).await?,
        Some(SubCommand::Replace(options)) => commands::cmd_replace(&manager, options).await?,
        _ => {
            need_show_after = false;
            commands::cmd_list(&manager, ListArgs::default()).await?
        }
    }

    if need_show_after {
        commands::cmd_list(&manager, ListArgs::default()).await?
    }
    Ok(())
}

#[derive(Debug, clap::Subcommand)]
enum SubCommand {
    /// List tasks
    #[clap(visible_alias = "ls")]
    List(ListArgs),
    /// Add a new task
    #[clap(visible_alias = "a")]
    Add(AddArgs),
    /// Done a task
    #[clap(visible_alias = "do")]
    Done(DoneArgs),
    /// Undone a task
    #[clap(visible_alias = "un")]
    Undone(DoneArgs),
    /// Delete a task
    #[clap(visible_alias = "de")]
    Delete(DeleteArgs),
    /// Set the priority of a task
    #[clap(visible_alias = "pri")]
    Priority(PriorityArgs),
    /// Append text to a task
    #[clap(visible_alias = "app")]
    Append(AppendArgs),
    /// Replace a task
    #[clap(visible_alias = "rep")]
    Replace(ReplaceArgs),
}

#[derive(Debug, clap::Parser)]
#[clap(name = "todo", author, about, version)]
struct Args {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}
