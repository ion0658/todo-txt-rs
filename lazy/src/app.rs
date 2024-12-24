use std::{future::Future, pin::Pin};

use crate::{windows, DrawEvent};
use anyhow::Result;

pub(crate) enum CurrentWindow {
    Main,
    Add,
}

pub(crate) trait EventHander {
    fn handle_key_event(
        &mut self,
        key_code: crossterm::event::KeyCode,
    ) -> impl Future<Output = Result<Option<DrawEvent>>>;
    fn tick(&self) -> impl Future<Output = Result<Option<DrawEvent>>>;
}

pub(crate) struct App {
    current_window: CurrentWindow,
    main_window_state: windows::main_window::MainWindowState,
    exit: bool,

    _todo_manager: Pin<Box<todo_txt_manager::TodoManager>>,
}

impl App {
    pub(crate) fn new() -> Result<Self> {
        let todo_manager = Box::pin(todo_txt_manager::TodoManager::new()?);
        Ok(Self {
            current_window: CurrentWindow::Main,
            main_window_state: windows::main_window::MainWindowState::new(todo_manager.clone()),
            exit: false,
            _todo_manager: todo_manager,
        })
    }

    pub(crate) fn should_exit(&self) -> bool {
        self.exit
    }
}

impl EventHander for App {
    async fn handle_key_event(
        &mut self,
        key_code: crossterm::event::KeyCode,
    ) -> Result<Option<DrawEvent>> {
        let event = match self.current_window {
            CurrentWindow::Main => {
                let e = self.main_window_state.handle_key_event(key_code).await?;
                if e.is_some() {
                    e
                } else {
                    match key_code {
                        crossterm::event::KeyCode::Char('q') => {
                            self.exit = true;
                            Some(DrawEvent::Exit)
                        }
                        crossterm::event::KeyCode::Char(c) => Some(DrawEvent::Draw(c.to_string())),
                        _ => None,
                    }
                }
            }
            CurrentWindow::Add => todo!(),
        };

        Ok(event)
    }

    async fn tick(&self) -> Result<Option<DrawEvent>> {
        match self.current_window {
            CurrentWindow::Main => self.main_window_state.tick().await,
            CurrentWindow::Add => todo!(),
        }
    }
}
