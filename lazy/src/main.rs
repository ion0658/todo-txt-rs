mod app;
mod windows;

use anyhow::Result;
use app::EventHander;
use futures::StreamExt;
use ratatui::{text::Text, Frame};

enum AppEvent {
    Key(crossterm::event::KeyCode),
    Tick,
    Ping,
}

enum DrawEvent {
    MainWindow(Vec<(usize, todo_txt_model::Task)>),
    Draw(String),
    Exit,
}

#[tokio::main]
async fn main() -> Result<()> {
    const QUEUE_SIZE: usize = 128;
    const TICK_RATE: std::time::Duration = std::time::Duration::from_millis(100);
    let (draw_tx, draw_rx) = tokio::sync::mpsc::channel::<DrawEvent>(QUEUE_SIZE);
    let (input_tx, input_rx) = tokio::sync::mpsc::channel::<AppEvent>(QUEUE_SIZE);
    let tasks = [
        tokio::spawn(event_loop(TICK_RATE, input_tx)),
        tokio::spawn(draw_loop(draw_rx)),
        tokio::spawn(app_loop(input_rx, draw_tx)),
    ];
    let _ = futures::future::join_all(tasks).await;
    Ok(())
}

// Event Handling Loop
async fn event_loop(
    max_tick_rate: std::time::Duration,
    tx: tokio::sync::mpsc::Sender<AppEvent>,
) -> Result<()> {
    let mut event_stream = crossterm::event::EventStream::new();

    // Send a reload event to the app
    tx.send(AppEvent::Key(crossterm::event::KeyCode::Char('r')))
        .await?;

    loop {
        let wait_result = tokio::time::timeout(max_tick_rate, event_stream.next()).await;
        if let Ok(event_result) = wait_result {
            // event stream invoked
            if let Some(Ok(event)) = event_result {
                match event {
                    crossterm::event::Event::Key(key_evt)
                        if key_evt.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        tx.send(AppEvent::Key(key_evt.code)).await?;
                    }
                    _ => {
                        tx.send(AppEvent::Tick).await?;
                    }
                };
            } else {
                break;
            }
        } else {
            // event stream timed out. Send a tick event
            tx.send(AppEvent::Ping).await?;
        }
    }
    Ok(())
}

// Application Update Loop
async fn app_loop(
    mut input_rx: tokio::sync::mpsc::Receiver<AppEvent>,
    draw_tx: tokio::sync::mpsc::Sender<DrawEvent>,
) -> Result<()> {
    let mut app = app::App::new()?;
    while let Some(event) = input_rx.recv().await {
        let draw_event = match event {
            AppEvent::Key(code) => app.handle_key_event(code).await?,
            AppEvent::Tick => app.tick().await?,
            AppEvent::Ping => None,
        };
        if let Some(draw_event) = draw_event {
            draw_tx.send(draw_event).await?;
        }
        if app.should_exit() {
            break;
        }
    }
    Ok(())
}

// Drawing UI Loop
async fn draw_loop(mut rx: tokio::sync::mpsc::Receiver<DrawEvent>) -> Result<()> {
    let mut terminal = ratatui::init();
    while let Some(event) = rx.recv().await {
        match event {
            DrawEvent::MainWindow(tasks) => {
                if let Err(e) = terminal.draw(|f| windows::main_window::draw(f, tasks)) {
                    eprintln!("Error: {:?}", e);
                    break;
                }
            }
            DrawEvent::Draw(i) => {
                if let Err(e) = terminal.draw(|f| draw(f, i)) {
                    eprintln!("Error: {:?}", e);
                    break;
                }
            }
            DrawEvent::Exit => break,
        }
    }
    ratatui::restore();
    Ok(())
}

fn draw(frame: &mut Frame, i: String) {
    let text = Text::raw(format!(
        "[{:?}] Hello, Ratatui! {:?} {}",
        std::time::SystemTime::now(),
        std::thread::current().id(),
        i
    ));
    frame.render_widget(text, frame.area());
}
