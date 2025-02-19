use ratatui::{backend::CrosstermBackend, Terminal, widgets::{Block, Borders, Paragraph}, layout::{Layout, Constraint, Direction}, text::Span};
use crossterm::{event::{self, Event, KeyCode}, execute, terminal::*};
use std::io::{stdout, Write};

pub fn start_tui() -> std::io::Result<(String, String)> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input_buffer = String::new();
    let mut client_ip = String::new();
    let mut client_name = String::new();
    let mut step = 0; // 0 = IP input, 1 = Name input, 2 = Done

    loop {
        terminal.draw(|f| {
            let size = f.size();

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            let title = match step {
                0 => "Enter Client IP: ",
                1 => "Enter Client Name: ",
                _ => "Press [Enter] to generate config",
            };

            let input_display = Paragraph::new(Span::raw(format!("{title}{input_buffer}")))
                .block(Block::default().title("WireGuard TUI").borders(Borders::ALL));

            f.render_widget(input_display, layout[0]);
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Enter => {
                        if step == 0 {
                            client_ip = input_buffer.clone();
                            input_buffer.clear();
                            step = 1;
                        } else if step == 1 {
                            client_name = input_buffer.clone();
                            input_buffer.clear();
                            step = 2;
                        } else {
                            break;
                        }
                    }
                    KeyCode::Char(c) => {
                        input_buffer.push(c);
                    }
                    KeyCode::Backspace => {
                        input_buffer.pop();
                    }
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                        return Ok(("".to_string(), "".to_string())); // Exit if ESC is pressed
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok((client_ip, client_name))
}
