// amdgpu_top_nvitop — nvitop-style TUI frontend for libamdgpu_top.
//
// Skeleton: enumerates AMDGPU/XDNA devices via libamdgpu_top::DevicePath and
// renders a nvitop-style shell (header / device list / footer). Sampling of
// utilization, VRAM, sensors, fdinfo and NPU metrics via AppAmdgpuTop is wired
// up in subsequent iterations.

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use libamdgpu_top::DevicePath;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Terminal;

fn main() -> io::Result<()> {
    let mut devices = DevicePath::get_device_path_list();
    for dp in devices.iter_mut() {
        dp.fill_amdgpu_device_name();
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal, &devices);

    // Restore terminal even on error.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    devices: &[DevicePath],
) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw(f, devices))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(k) = event::read()? {
                match k.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}

fn draw(f: &mut ratatui::Frame, devices: &[DevicePath]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(f.area());

    // nvitop-style green header.
    let header = Paragraph::new(" amdgpu-top-nvitop ")
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Device list.
    let items: Vec<ListItem> = devices
        .iter()
        .map(|dp| {
            let kind = if dp.is_xdna() {
                "[NPU]"
            } else if dp.is_amdgpu() {
                "[GPU]"
            } else {
                "[?]"
            };
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{kind} "),
                    Style::default().fg(Color::Cyan),
                ),
                Span::raw(dp.menu_entry()),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" AMDGPU Devices "));
    f.render_widget(list, chunks[1]);

    let footer = Paragraph::new(" q: quit ")
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(footer, chunks[2]);
}
