mod add;
mod append;
mod delete;
mod done;
pub(crate) mod list;
mod priority;
mod replace;

use todo_txt_model::prelude::*;
use todo_txt_serializer::prelude::*;

#[cfg(feature = "rt_async_std")]
use async_std::io::BufReadExt as AsyncRTBufReadExt;
#[cfg(feature = "rt_smol")]
use smol::io::{AsyncBufReadExt as AsyncRTBufReadExt, AsyncWriteExt, BufWriter};
#[cfg(feature = "rt_tokio")]
use tokio::io::{AsyncBufReadExt as AsyncRTBufReadExt, AsyncWriteExt, BufWriter};

#[tracing::instrument(parent = None, skip(reader))]
pub fn read_tasks<R>(reader: &mut R) -> Result<Vec<Task>>
where
    R: std::io::BufRead,
{
    let mut out = Vec::new();
    let mut line = String::new();
    while reader.read_line(&mut line).is_ok() {
        if line.is_empty() {
            tracing::debug!("empty line, breaking");
            break;
        }
        tracing::debug!("line: {:?}", line);
        let task = from_str(&line)?;
        tracing::debug!("task: {:?}", task);
        line.clear();
        out.push(task);
    }
    Ok(out)
}

#[cfg(any(feature = "rt_async_std", feature = "rt_tokio", feature = "rt_smol"))]
#[tracing::instrument(parent = None, skip(reader))]
pub async fn read_tasks_async<R>(reader: &mut R) -> Result<Vec<Task>>
where
    R: AsyncRTBufReadExt + Unpin,
{
    let mut out = Vec::new();
    let mut line = String::new();
    while reader.read_line(&mut line).await.is_ok() {
        if line.is_empty() {
            tracing::debug!("empty line, breaking");
            break;
        }
        tracing::debug!("line: {:?}", line);
        let task = from_str(&line)?;
        tracing::debug!("task: {:?}", task);
        line.clear();
        out.push(task);
    }
    Ok(out)
}

#[tracing::instrument(parent = None, skip(writer))]
pub(crate) fn write_task<W: std::io::Write>(
    writer: &mut std::io::BufWriter<W>,
    task: &Task,
) -> Result<()> {
    use std::io::Write;
    writeln!(writer, "{}", todo_txt_serializer::to_string(task))?;
    Ok(())
}

#[cfg(any(feature = "rt_tokio", feature = "rt_smol"))]
#[tracing::instrument(parent = None, skip(writer))]
pub(crate) async fn write_task_async<W>(writer: &mut BufWriter<W>, task: &Task) -> Result<()>
where
    W: AsyncWriteExt + Unpin,
{
    use std::io::Write;

    let mut buf = Vec::new();
    writeln!(buf, "{}", todo_txt_serializer::to_string(task))?;
    writer.write_all(&buf).await?;
    Ok(())
}

#[cfg(feature = "rt_async_std")]
#[tracing::instrument(parent = None, skip(writer))]
pub(crate) async fn write_task_async<W>(
    writer: &mut async_std::io::BufWriter<W>,
    task: &Task,
) -> Result<()>
where
    W: async_std::io::Write + Unpin,
{
    use async_std::io::WriteExt;
    writeln!(writer, "{}", todo_txt_serializer::to_string(task)).await?;
    Ok(())
}

#[tracing::instrument(parent = None)]
pub fn read_tasks_from_file(file: &std::path::Path) -> Result<Vec<Task>> {
    let mut reader = std::io::BufReader::new(
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(file)?,
    );
    crate::commands::read_tasks(&mut reader)
}

#[cfg(any(feature = "rt_async_std", feature = "rt_tokio", feature = "rt_smol"))]
#[tracing::instrument(parent = None)]
pub async fn read_tasks_from_file_async(file: &std::path::Path) -> Result<Vec<Task>> {
    #[cfg(feature = "rt_async_std")]
    use async_std::{fs::OpenOptions, io::BufReader};
    #[cfg(feature = "rt_smol")]
    use smol::{fs::OpenOptions, io::BufReader};
    #[cfg(feature = "rt_tokio")]
    use tokio::{fs::OpenOptions, io::BufReader};
    let mut reader = BufReader::new(
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(file)
            .await?,
    );
    crate::commands::read_tasks_async(&mut reader).await
}

#[tracing::instrument(parent = None, skip(tasks))]
pub fn write_tasks<'a>(file: &std::path::Path, tasks: &[Task]) -> Result<()> {
    use std::io::Write;
    let mut writer = std::io::BufWriter::new(
        std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file)?,
    );

    for task in tasks {
        write_task(&mut writer, task)?;
    }
    writer.flush()?;
    Ok(())
}

#[cfg(any(feature = "rt_async_std", feature = "rt_tokio", feature = "rt_smol"))]
#[tracing::instrument(parent = None, skip(tasks))]
pub async fn write_tasks_async(file: &std::path::Path, tasks: &[Task]) -> Result<()> {
    #[cfg(feature = "rt_async_std")]
    use async_std::{fs::OpenOptions, io::BufWriter};
    #[cfg(feature = "rt_smol")]
    use smol::{
        fs::OpenOptions,
        io::{AsyncWriteExt, BufWriter},
    };
    #[cfg(feature = "rt_tokio")]
    use tokio::{
        fs::OpenOptions,
        io::{AsyncWriteExt, BufWriter},
    };
    let mut writer = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file)
            .await?,
    );

    for task in tasks {
        write_task_async(&mut writer, task).await?;
    }
    writer.flush().await?;
    Ok(())
}
