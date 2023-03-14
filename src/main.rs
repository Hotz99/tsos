use std::{io, error::Error, thread::sleep, time::Duration};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{widgets::{Block, Borders, BarChart}, style::{Color, Style}, backend::CrosstermBackend, Terminal, layout::Alignment};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.show_cursor()?;

    let chart = BarChart::default()
    .block(Block::default().title(" TSOS ").title_alignment(Alignment::Center).borders(Borders::ALL))
    .bar_width(1)
    .bar_gap(0)
    .bar_style(Style::default().fg(Color::White))
    .label_style(Style::default().fg(Color::White))
    .data(&[("1", 2), ("2", 2), ("3", 4), ("4", 3)]);

    terminal.draw(|f| { f.render_widget(chart, f.size()) })?;

    sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}