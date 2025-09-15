mod app;
mod categories;
mod installer;
mod ui;

use crossterm::{
    cursor::{Hide, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    io::{self, Write},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use app::{App, AppState, InstallMethod};
use installer::Installer;
use ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize terminal with proper setup
    setup_terminal()?;

    // Run the application
    let result = run_application();

    // Always cleanup terminal regardless of result
    cleanup_terminal()?;

    result
}

fn setup_terminal() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, Hide)?;
    Ok(())
}

fn cleanup_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture,
        Show,
        Clear(ClearType::All)
    )?;
    // Additional cleanup to ensure terminal is completely clear
    print!("\x1b[2J\x1b[H"); // Clear screen and move cursor to top-left
    stdout.flush()?;
    Ok(())
}

fn run_application() -> Result<(), Box<dyn Error>> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Show professional loading screen within TUI
    show_loading_screen(&mut terminal)?;

    let mut app = App::default();
    app.set_fast_startup_mode(true);

    let res = run_app(&mut terminal, app)?;
    Ok(res)
}

fn show_loading_screen(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    let loading_duration = Duration::from_millis(1500);
    let start_time = Instant::now();
    let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let mut frame_count = 0;

    while start_time.elapsed() < loading_duration {
        let progress =
            start_time.elapsed().as_millis() as f64 / loading_duration.as_millis() as f64;
        let spinner = spinner_chars[frame_count % spinner_chars.len()];

        terminal.draw(|f| {
            render_loading_screen(f, progress, spinner);
        })?;

        thread::sleep(Duration::from_millis(80));
        frame_count += 1;
    }

    // Final frame at 100%
    terminal.draw(|f| {
        render_loading_screen(f, 1.0, '✓');
    })?;

    thread::sleep(Duration::from_millis(300));
    Ok(())
}

fn render_loading_screen(f: &mut Frame, progress: f64, spinner: char) {
    let size = f.area(); // FIXED: Changed from f.size() to f.area()

    // Create centered layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Length(8),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Percentage(35),
        ])
        .split(size);

    // Logo/Title block
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));

    let title = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            "╔══════════════════════════════════════════════╗",
            Style::default().fg(Color::Blue),
        )]),
        Line::from(vec![
            Span::styled("║                ", Style::default().fg(Color::Blue)),
            Span::styled(
                "BQuick v1.0",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("                ║", Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::styled("║            ", Style::default().fg(Color::Blue)),
            Span::styled("Software Installer", Style::default().fg(Color::Cyan)),
            Span::styled("            ║", Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![Span::styled(
            "╚══════════════════════════════════════════════╝",
            Style::default().fg(Color::Blue),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(title_block);

    f.render_widget(title, chunks[1]);

    // Progress bar
    let progress_percentage = (progress * 100.0) as u16;
    let progress_bar = Gauge::default()
        .block(
            Block::default()
                .title("Loading System Components")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(progress)
        .label(format!("{}%", progress_percentage));

    f.render_widget(progress_bar, chunks[2]);

    // Status text with spinner
    let status_text = if progress >= 1.0 {
        "✓ Ready to launch".to_string()
    } else {
        format!("{} Initializing application...", spinner)
    };

    let status = Paragraph::new(status_text)
        .style(
            Style::default()
                .fg(if progress >= 1.0 {
                    Color::Green
                } else {
                    Color::Yellow
                })
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray)),
        );

    f.render_widget(status, chunks[3]);
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        app.process_log_messages();

        if app.installation_complete {
            app.installation_complete = false;
            app.state = AppState::ShowResult;
            app.result_message = "Operation completed successfully".to_string();
        }

        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.state {
                        AppState::SearchMode => match key.code {
                            KeyCode::Backspace => {
                                app.search_input.pop();
                                app.update_search_results();
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                app.previous_search_result();
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                app.next_search_result();
                            }
                            KeyCode::Char(' ') => {
                                app.toggle_search_selection();
                            }
                            KeyCode::Enter => {
                                let selected_count = app.get_total_selected();
                                if selected_count > 0 {
                                    app.state = AppState::Installing;
                                    app.clear_installation_output();
                                    app.total_to_install = selected_count;
                                    app.installing_progress = 0;

                                    let (log_sender, log_receiver) = mpsc::channel();
                                    app.set_log_receiver(log_receiver);

                                    Installer::start_installation(&mut app, log_sender);
                                } else {
                                    app.result_message =
                                        "No software selected. Use Space to select software."
                                            .to_string();
                                }
                            }
                            KeyCode::Esc => {
                                app.state = AppState::CategorySelection;
                                app.search_input.clear();
                                app.search_results.clear();
                            }
                            KeyCode::F(1) => {
                                app.show_help = !app.show_help;
                            }
                            KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                            KeyCode::Char('u') | KeyCode::Char('U') => {
                                app.start_uninstall_mode();
                            }
                            KeyCode::Char('i') | KeyCode::Char('I') => {
                                app.start_reinstall_mode();
                            }
                            KeyCode::Char('n') | KeyCode::Char('N') => {
                                app.start_install_mode();
                            }
                            KeyCode::Char(c) => {
                                app.search_input.push(c);
                                app.update_search_results();
                            }
                            _ => {}
                        },
                        _ => {
                            match key.code {
                                KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                                KeyCode::Char('h') | KeyCode::Char('H') | KeyCode::F(1) => {
                                    app.show_help = !app.show_help;
                                }
                                KeyCode::Char('/') => {
                                    if matches!(
                                        app.state,
                                        AppState::CategorySelection | AppState::SoftwareSelection
                                    ) {
                                        app.state = AppState::SearchMode;
                                        app.search_input.clear();
                                        app.search_results.clear();
                                    }
                                }
                                KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::F(5) => {
                                    if matches!(app.state, AppState::SoftwareSelection) {
                                        app.start_comprehensive_scan();
                                    }
                                }
                                KeyCode::Char('a') | KeyCode::Char('A') => {
                                    if matches!(app.state, AppState::SoftwareSelection) {
                                        app.select_all_in_category();
                                    }
                                }
                                KeyCode::Char('u') | KeyCode::Char('U') => {
                                    if matches!(app.state, AppState::SoftwareSelection) {
                                        app.start_uninstall_mode();
                                    }
                                }
                                KeyCode::Char('i') | KeyCode::Char('I') => {
                                    if matches!(app.state, AppState::SoftwareSelection) {
                                        app.start_reinstall_mode();
                                    }
                                }
                                KeyCode::Char('n') | KeyCode::Char('N') => {
                                    if matches!(app.state, AppState::SoftwareSelection) {
                                        app.start_install_mode();
                                    }
                                }
                                KeyCode::Up | KeyCode::Char('k') => match app.state {
                                    AppState::MethodSelection => app.previous_method(),
                                    AppState::CategorySelection => app.previous_category(),
                                    AppState::SoftwareSelection => app.previous_software(),
                                    _ => {}
                                },
                                KeyCode::Down | KeyCode::Char('j') => match app.state {
                                    AppState::MethodSelection => app.next_method(),
                                    AppState::CategorySelection => app.next_category(),
                                    AppState::SoftwareSelection => app.next_software(),
                                    _ => {}
                                },
                                KeyCode::Char(' ') => {
                                    if matches!(app.state, AppState::SoftwareSelection) {
                                        app.toggle_software_selection();
                                    }
                                }
                                KeyCode::Esc => match app.state {
                                    AppState::SoftwareSelection => {
                                        app.state = AppState::CategorySelection
                                    }
                                    AppState::ShowResult => app.state = AppState::CategorySelection,
                                    AppState::CategorySelection => {
                                        app.state = AppState::MethodSelection
                                    }
                                    AppState::MethodSelection => return Ok(()),
                                    AppState::Installing => {}
                                    _ => {}
                                },
                                KeyCode::Enter => match app.state {
                                    AppState::MethodSelection => {
                                        app.selected_method = Some(if app.method_index == 0 {
                                            InstallMethod::PowerShell
                                        } else {
                                            InstallMethod::Chocolatey
                                        });
                                        app.state = AppState::CategorySelection;
                                    }
                                    AppState::CategorySelection => {
                                        app.state = AppState::SoftwareSelection;
                                        if app.is_fast_startup_mode() {
                                            app.start_comprehensive_scan();
                                        }
                                    }
                                    AppState::SoftwareSelection => {
                                        let selected_count = app.get_total_selected();
                                        if selected_count > 0 {
                                            app.state = AppState::Installing;
                                            app.clear_installation_output();
                                            app.total_to_install = selected_count;
                                            app.installing_progress = 0;

                                            let (log_sender, log_receiver) = mpsc::channel();
                                            app.set_log_receiver(log_receiver);

                                            Installer::start_installation(&mut app, log_sender);
                                        } else {
                                            app.result_message = "No software selected. Use Space to select software.".to_string();
                                        }
                                    }
                                    AppState::ShowResult => {
                                        for category in &mut app.categories {
                                            for software in &mut category.software {
                                                software.is_selected = false;
                                            }
                                        }
                                        app.result_message.clear();
                                        app.state = AppState::CategorySelection;
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}
