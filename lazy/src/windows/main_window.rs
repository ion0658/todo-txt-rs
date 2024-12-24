use crate::{app::EventHander, DrawEvent};
use anyhow::Result;
use ratatui::{text::Text, Frame};
use std::pin::Pin;
use todo_txt_model::Task;

pub(crate) struct MainWindowState {
    todo_manager: Pin<Box<todo_txt_manager::TodoManager>>,
    tasks: Vec<(usize, todo_txt_model::Task)>,
    filter: todo_txt_manager::Filter,
    order: todo_txt_manager::Order,
}

impl MainWindowState {
    pub(crate) fn new(todo_manager: Pin<Box<todo_txt_manager::TodoManager>>) -> Self {
        Self {
            todo_manager,
            tasks: Vec::new(),
            filter: todo_txt_manager::Filter::default(),
            order: todo_txt_manager::Order::default(),
        }
    }

    pub(crate) async fn reload_tasks(&mut self) -> Result<()> {
        self.tasks = self
            .todo_manager
            .list_async(self.filter.clone(), self.order.clone())
            .await?;
        Ok(())
    }
}

impl EventHander for MainWindowState {
    async fn handle_key_event(
        &mut self,
        key_code: crossterm::event::KeyCode,
    ) -> Result<Option<DrawEvent>> {
        let e = match key_code {
            crossterm::event::KeyCode::Char('r') => {
                self.reload_tasks().await?;
                Some(DrawEvent::MainWindow(self.tasks.clone()))
            }
            _ => None,
        };
        Ok(e)
    }

    async fn tick(&self) -> Result<Option<DrawEvent>> {
        Ok(Some(DrawEvent::MainWindow(self.tasks.clone())))
    }
}

pub(crate) fn draw(frame: &mut Frame, tasks: Vec<(usize, Task)>) {
    let text = Text::raw(format!("Tasks: {}", tasks.len()));
    frame.render_widget(text, frame.area());
}
