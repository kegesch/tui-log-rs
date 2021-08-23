use std::io;
use tui_log::{TuiLogger, Writable};
use log::LevelFilter;
use tui::backend::{CrosstermBackend, Backend};
use tui::Terminal;
use tui::layout::{Layout, Direction, Constraint, Rect};
use tui::widgets::{Block, Borders, StatefulWidget, Widget};
use tui::buffer::Buffer;
use tui::style::Style;
use std::sync::{Arc, Mutex};
use std::borrow::BorrowMut;
use log::{info};


#[derive(Default, Clone)]
pub struct LogWidgetState {
    pub history: Vec<String>,
}

#[derive(Default, Clone)]
pub struct LogWidget {}

impl StatefulWidget for LogWidget {
    type State = LogWidgetState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let max_lines = area.height - 1;

        let history_to_show = state.history.iter().rev().take(max_lines as usize).rev();

        for (y, line) in history_to_show.enumerate() {
            buf.set_string(area.left(), area.top() + y as u16, line, Style::default());
        }
    }
}

impl Widget for LogWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(self, area, buf, &mut LogWidgetState::default())
    }
}

impl Writable for LogWidgetState {
    fn write_line(&mut self, message: &str) {
        self.history.push(message.to_string())
    }

    fn flush(&mut self) {
        self.history.clear()
    }
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let state = Arc::new(Mutex::new(LogWidgetState::default()));
    TuiLogger::init(LevelFilter::Info, state.clone()).expect("Could not init logger");

    terminal.clear().expect("Could not clear terminal");
    let mut i = 0;
    loop {
        draw(&mut terminal, state.clone())?;
        info!("Logged an info {}", i);
        i += 1;
    }
}

fn draw<B: Backend>(terminal: &mut Terminal<B>, log_widget_state: Arc<Mutex<LogWidgetState>>) -> io::Result<()>{
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ].as_ref()
            )
            .split(f.size());
        let block = Block::default()
            .title("Block")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[0]);
        let block = Block::default()
            .title("Log")
            .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
        let inset_area = edge_inset(&chunks[1], 1);
        let log_widget = LogWidget::default();
        f.render_stateful_widget(log_widget, inset_area, log_widget_state.lock().unwrap().borrow_mut());
    })?;
    Ok(())
}

fn edge_inset(area: &Rect, margin: u16) -> Rect {
    let mut inset_area = *area;
    inset_area.x += margin;
    inset_area.y += margin;
    inset_area.height -= margin;
    inset_area.width -= margin;

    inset_area
}