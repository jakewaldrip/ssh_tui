use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Terminal,
};

use crate::app::App;
use crate::terminal_handle::TerminalHandle;

pub fn render_ui(terminal: &mut Terminal<CrosstermBackend<TerminalHandle>>, app: &mut App) -> () {
    terminal
        .draw(|f| {
            let area = f.area();
            f.render_widget(Clear, area);
            let style = match app.counter % 3 {
                0 => Style::default().fg(Color::Red),
                1 => Style::default().fg(Color::Green),
                _ => Style::default().fg(Color::Blue),
            };
            let paragraph = Paragraph::new(format!("Counter: {}", app.counter))
                .alignment(ratatui::layout::Alignment::Center)
                .style(style);
            let block = Block::default()
                .title("Press 'c' to reset the counter!")
                .borders(Borders::ALL);
            f.render_widget(paragraph.block(block), area);
        })
        .unwrap();
}
