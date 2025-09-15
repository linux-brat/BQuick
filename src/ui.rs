use crate::app::{App, AppState, InstallMethod, OperationMode};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn ui(f: &mut Frame<'_>, app: &mut App) {
    let size = f.area(); // FIXED: Changed from f.size() to f.area()

    // Professional layout with status bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(1), // Status bar
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Footer
        ])
        .split(size);

    // Professional header
    let header_text = match app.selected_method {
        Some(InstallMethod::PowerShell) => format!(
            "BQuick v1.0 - Software Installer [PowerShell Mode] - {}",
            app.get_operation_text()
        ),
        Some(InstallMethod::Chocolatey) => format!(
            "BQuick v1.0 - Software Installer [Chocolatey Mode] - {}",
            app.get_operation_text()
        ),
        None => format!(
            "BQuick v1.0 - Software Installer - {}",
            app.get_operation_text()
        ),
    };

    let header = Paragraph::new(header_text)
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        );
    f.render_widget(header, chunks[0]);

    // Enhanced status bar with scanning indicator
    let status_text = if app.scanning_in_progress {
        app.get_scanning_status()
    } else {
        let selected_count = app.get_total_selected();
        let method_text = match app.selected_method {
            Some(InstallMethod::PowerShell) => "PowerShell",
            Some(InstallMethod::Chocolatey) => "Chocolatey",
            None => "No Method",
        };

        if selected_count > 0 {
            format!(
                "Ready | Selected: {} | Mode: {} | Operation: {} | Press Enter to proceed",
                selected_count,
                method_text,
                app.get_operation_text()
            )
        } else {
            format!(
                "Ready | Selected: {} | Mode: {} | Operation: {}",
                selected_count,
                method_text,
                app.get_operation_text()
            )
        }
    };

    let status_bar = Paragraph::new(status_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(status_bar, chunks[1]);

    // Enhanced footer with updated keybindings
    let footer_text = get_footer_text_for_state(app);

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(footer, chunks[3]);

    // Main content area
    match app.state {
        AppState::MethodSelection => render_method_selection(f, chunks[2], app),
        AppState::CategorySelection => render_category_selection(f, chunks[2], app),
        AppState::SoftwareSelection => render_software_selection(f, chunks[2], app),
        AppState::SearchMode => render_search_mode(f, chunks[2], app),
        AppState::Installing => render_installing_professional(f, chunks[2], app),
        AppState::ShowResult => render_result(f, chunks[2], app),
    }

    // Professional help overlay
    if app.show_help {
        render_professional_help(f, size);
    }
}

fn get_footer_text_for_state(app: &App) -> String {
    let selected_count = app.get_total_selected();

    match app.state {
        AppState::MethodSelection => {
            "↑↓: Navigate | Enter: Select | F1: Help | Q: Quit".to_string()
        }
        AppState::CategorySelection => {
            "↑↓: Navigate | Enter: Select | /: Search | Esc: Back | F1: Help | Q: Quit".to_string()
        }
        AppState::SoftwareSelection => {
            let base = match app.operation_mode {
                OperationMode::Install => {
                    if selected_count > 0 {
                        format!("↑↓: Navigate | Space: Toggle | A: All | Enter: Install ({}) | F5: Deep Scan | U: Uninstall Mode | I: Reinstall Mode | Esc: Back | F1: Help | Q: Quit", selected_count)
                    } else {
                        "↑↓: Navigate | Space: Toggle | A: All | F5: Deep Scan | U: Uninstall Mode | I: Reinstall Mode | Esc: Back | F1: Help | Q: Quit".to_string()
                    }
                }
                OperationMode::Uninstall => {
                    if selected_count > 0 {
                        format!("↑↓: Navigate | Space: Toggle | A: All | Enter: Uninstall ({}) | I: Reinstall Mode | N: Install Mode | Esc: Back | F1: Help | Q: Quit", selected_count)
                    } else {
                        "↑↓: Navigate | Space: Toggle | A: All | I: Reinstall Mode | N: Install Mode | Esc: Back | F1: Help | Q: Quit (Only installed software can be selected)".to_string()
                    }
                }
                OperationMode::Reinstall => {
                    if selected_count > 0 {
                        format!("↑↓: Navigate | Space: Toggle | A: All | Enter: Reinstall ({}) | U: Uninstall Mode | N: Install Mode | Esc: Back | F1: Help | Q: Quit", selected_count)
                    } else {
                        "↑↓: Navigate | Space: Toggle | A: All | U: Uninstall Mode | N: Install Mode | Esc: Back | F1: Help | Q: Quit (Only installed software can be selected)".to_string()
                    }
                }
            };
            base
        }
        AppState::SearchMode => {
            if selected_count > 0 {
                format!("Type to search | ↑↓: Navigate | Space: Toggle | Enter: {} ({}) | U: Uninstall Mode | Esc: Back | F1: Help | Q: Quit", 
                        app.get_operation_text(), selected_count)
            } else {
                format!("Type to search | ↑↓: Navigate | Space: Toggle | U: Uninstall Mode | Esc: Back | F1: Help | Q: Quit")
            }
        }
        AppState::Installing => {
            format!(
                "{} in progress... | F1: Help | Q: Quit",
                app.get_operation_text()
            )
        }
        AppState::ShowResult => "Enter: Continue | Esc: Back | F1: Help | Q: Quit".to_string(),
    }
}

fn render_method_selection(f: &mut Frame<'_>, area: Rect, app: &App) {
    let methods = vec![
        ListItem::new(Line::from(vec![
            Span::styled(
                "PowerShell/WinGet ",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled("(Recommended)", Style::default().fg(Color::Green)),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("Chocolatey ", Style::default().add_modifier(Modifier::BOLD)),
            Span::styled("(Package Manager)", Style::default().fg(Color::Yellow)),
        ])),
    ];

    let mut list_state = ListState::default();
    list_state.select(Some(app.method_index));

    let methods_list = List::new(methods)
        .block(
            Block::default()
                .title("Select Installation Method")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    f.render_stateful_widget(methods_list, area, &mut list_state);
}

fn render_category_selection(f: &mut Frame<'_>, area: Rect, app: &mut App) {
    let categories: Vec<ListItem> = app
        .categories
        .iter()
        .map(|category| {
            let selected_count = category.get_selected_count();
            let total_count = category.software.len();
            let installed_count = category.software.iter().filter(|s| s.is_installed).count();

            let status_indicator = if selected_count > 0 {
                format!(" [{}]", selected_count)
            } else {
                String::new()
            };

            ListItem::new(Line::from(vec![
                Span::styled(category.icon, Style::default().fg(Color::Cyan)),
                Span::raw(" "),
                Span::styled(category.name, Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(status_indicator, Style::default().fg(Color::Yellow)),
                Span::styled(
                    format!(" ({} items, {} installed)", total_count, installed_count),
                    Style::default().fg(Color::DarkGray),
                ),
            ]))
        })
        .collect();

    let categories_list = List::new(categories)
        .block(
            Block::default()
                .title("Software Categories")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    f.render_stateful_widget(categories_list, area, &mut app.category_list_state);
}

fn render_software_selection(f: &mut Frame<'_>, area: Rect, app: &mut App) {
    let category = app.get_current_category();
    let software_items: Vec<ListItem> = category
        .software
        .iter()
        .map(|software| {
            let mut spans = Vec::new();

            // Selection indicator
            if software.is_selected {
                spans.push(Span::styled(
                    "[X] ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled("[ ] ", Style::default().fg(Color::DarkGray)));
            }

            // Installation status
            if software.is_installed {
                spans.push(Span::styled(
                    "INSTALLED ",
                    Style::default().fg(Color::Green),
                ));
            } else {
                spans.push(Span::styled(
                    "NOT INSTALLED ",
                    Style::default().fg(Color::Red),
                ));
            }

            // Software name with operation-specific styling
            let name_style = match app.operation_mode {
                OperationMode::Install => {
                    if software.is_selected {
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().add_modifier(Modifier::BOLD)
                    }
                }
                OperationMode::Uninstall | OperationMode::Reinstall => {
                    if !software.is_installed {
                        // Gray out non-installed software in uninstall/reinstall mode
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::DIM)
                    } else if software.is_selected {
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().add_modifier(Modifier::BOLD)
                    }
                }
            };
            spans.push(Span::styled(software.name, name_style));

            // Operation indicator
            match app.operation_mode {
                OperationMode::Uninstall if software.is_installed => {
                    spans.push(Span::styled(
                        " [Can Uninstall]",
                        Style::default().fg(Color::Red),
                    ));
                }
                OperationMode::Reinstall if software.is_installed => {
                    spans.push(Span::styled(
                        " [Can Reinstall]",
                        Style::default().fg(Color::Yellow),
                    ));
                }
                OperationMode::Uninstall | OperationMode::Reinstall if !software.is_installed => {
                    spans.push(Span::styled(
                        " [Not Available]",
                        Style::default().fg(Color::DarkGray),
                    ));
                }
                _ => {}
            }

            // Description
            spans.push(Span::raw(" - "));
            spans.push(Span::styled(
                software.description,
                Style::default().fg(Color::DarkGray),
            ));

            ListItem::new(Line::from(spans))
        })
        .collect();

    let selected_count = category.get_selected_count();
    let total_count = category.software.len();
    let installed_count = category.software.iter().filter(|s| s.is_installed).count();

    let title = format!(
        "{} {} - Selected: {}/{} (Installed: {}) - {} Mode",
        category.icon,
        category.name,
        selected_count,
        total_count,
        installed_count,
        app.get_operation_text()
    );

    let software_list = List::new(software_items)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    f.render_stateful_widget(software_list, area, &mut app.software_list_state);
}

fn render_search_mode(f: &mut Frame<'_>, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Search input with cursor indicator
    let search_text = if app.search_input.is_empty() {
        format!(
            "Search: (type to search software...) - {} Mode",
            app.get_operation_text()
        )
    } else {
        format!(
            "Search: {}_ - {} Mode",
            app.search_input,
            app.get_operation_text()
        )
    };

    let search_input = Paragraph::new(search_text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("Search Software")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );
    f.render_widget(search_input, chunks[0]);

    // Search results
    if app.search_results.is_empty() && !app.search_input.is_empty() {
        let no_results = Paragraph::new("No results found. Try different search terms.")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Results")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue)),
            );
        f.render_widget(no_results, chunks[1]);
    } else if app.search_input.is_empty() {
        let help_text = Paragraph::new("Start typing to search through all available software...")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Search Help")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue)),
            );
        f.render_widget(help_text, chunks[1]);
    } else {
        let search_items: Vec<ListItem> = app
            .search_results
            .iter()
            .map(|(cat_idx, _soft_idx, software)| {
                let category = &app.categories[*cat_idx];
                let mut spans = Vec::new();

                if software.is_selected {
                    spans.push(Span::styled("[X] ", Style::default().fg(Color::Green)));
                } else {
                    spans.push(Span::styled("[ ] ", Style::default().fg(Color::DarkGray)));
                }

                spans.push(Span::styled(
                    format!("[{}] ", category.name),
                    Style::default().fg(Color::Cyan),
                ));

                // Operation-specific styling
                let name_style = match app.operation_mode {
                    OperationMode::Install => Style::default().add_modifier(Modifier::BOLD),
                    OperationMode::Uninstall | OperationMode::Reinstall => {
                        if !software.is_installed {
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM)
                        } else {
                            Style::default().add_modifier(Modifier::BOLD)
                        }
                    }
                };

                spans.push(Span::styled(software.name, name_style));

                // Status indicators
                if software.is_installed {
                    spans.push(Span::styled(
                        " [INSTALLED]",
                        Style::default().fg(Color::Green),
                    ));
                } else {
                    spans.push(Span::styled(
                        " [NOT INSTALLED]",
                        Style::default().fg(Color::Red),
                    ));
                }

                spans.push(Span::raw(" - "));
                spans.push(Span::styled(
                    software.description,
                    Style::default().fg(Color::DarkGray),
                ));

                ListItem::new(Line::from(spans))
            })
            .collect();

        let search_list = List::new(search_items)
            .block(
                Block::default()
                    .title(format!(
                        "Search Results ({}) - {} Mode",
                        app.search_results.len(),
                        app.get_operation_text()
                    ))
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue)),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("► ");

        f.render_stateful_widget(search_list, chunks[1], &mut app.search_list_state);
    }
}

fn render_installing_professional(f: &mut Frame<'_>, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    // Current software being processed
    let current_software = if app.current_installing_software.is_empty() {
        format!("Preparing {}...", app.get_operation_text().to_lowercase())
    } else {
        format!(
            "{}: {}",
            app.get_operation_text(),
            app.current_installing_software
        )
    };

    let current = Paragraph::new(current_software)
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Current Operation")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        );
    f.render_widget(current, chunks[0]);

    // Enhanced progress bar
    let progress = if app.total_to_install > 0 {
        app.installing_progress as f64 / app.total_to_install as f64
    } else {
        0.0
    };

    let progress_percentage = (progress * 100.0) as u16;

    let progress_bar = Gauge::default()
        .block(
            Block::default()
                .title(format!(
                    "Progress ({}/{}) - {}%",
                    app.installing_progress, app.total_to_install, progress_percentage
                ))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(progress);
    f.render_widget(progress_bar, chunks[1]);

    // Installation log with enhanced formatting
    let log_lines: Vec<Line> = app
        .installation_output
        .iter()
        .map(|line| {
            if line.contains("Successfully")
                || line.starts_with("✓")
                || line.contains("completed successfully")
            {
                Line::from(Span::styled(line, Style::default().fg(Color::Green)))
            } else if line.contains("Failed") || line.contains("Error") || line.starts_with("✗") {
                Line::from(Span::styled(line, Style::default().fg(Color::Red)))
            } else if line.contains("Installing")
                || line.contains("Uninstalling")
                || line.contains("Reinstalling")
                || line.contains("Starting")
                || line.contains("Attempting")
            {
                Line::from(Span::styled(line, Style::default().fg(Color::Cyan)))
            } else if line.contains("Warning")
                || line.contains("Fallback")
                || line.contains("Trying alternate")
            {
                Line::from(Span::styled(line, Style::default().fg(Color::Yellow)))
            } else if line.contains("Update available") || line.contains("Checking for updates") {
                Line::from(Span::styled(line, Style::default().fg(Color::Magenta)))
            } else {
                Line::from(Span::raw(line))
            }
        })
        .collect();

    let log_output = Paragraph::new(log_lines)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title(format!("{} Log", app.get_operation_text()))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .wrap(Wrap { trim: true })
        .scroll((
            app.installation_output
                .len()
                .saturating_sub(chunks[2].height as usize - 2) as u16,
            0,
        ));

    f.render_widget(log_output, chunks[2]);
}

fn render_result(f: &mut Frame<'_>, area: Rect, app: &App) {
    let result_text = if app.result_message.is_empty() {
        format!(
            "{} completed successfully.\n\nPress Enter to continue or Esc to go back.",
            app.get_operation_text()
        )
    } else {
        app.result_message.clone()
    };

    let result = Paragraph::new(result_text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .title(format!("{} Results", app.get_operation_text()))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        );
    f.render_widget(result, area);
}

fn render_professional_help(f: &mut Frame<'_>, area: Rect) {
    let popup_area = centered_rect(80, 75, area);

    let help_text = Text::from(vec![
        Line::from(vec![
            Span::styled(
                "BQuick v1.0 ",
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "- Professional Help Guide",
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "NAVIGATION & CONTROL",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  ↑↓ / j k     Navigate up/down in lists"),
        Line::from("  Enter        Confirm selection or start operation"),
        Line::from("  Esc          Go back to previous screen"),
        Line::from("  Q            Quit application"),
        Line::from("  F1 / H       Toggle this help screen"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "SOFTWARE MANAGEMENT",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  Space        Toggle software selection"),
        Line::from("  A            Select/deselect all in category"),
        Line::from("  F5 / R       Deep scan for installed software"),
        Line::from("  /            Enter search mode"),
        Line::from("  U            Switch to Uninstall mode (installed software only)"),
        Line::from("  I            Switch to Reinstall mode (installed software only)"),
        Line::from("  N            Switch to Install mode (normal install)"),
        Line::from(""),
        Line::from(vec![Span::styled(
            "OPERATION MODES",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from("  Install      Install new software or update existing"),
        Line::from("  Uninstall    Remove installed software (requires software to be installed)"),
        Line::from(
            "  Reinstall    Uninstall and reinstall software (requires software to be installed)",
        ),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Press F1 or H to close this help",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )]),
    ]);

    let help_paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .title("Professional Help Guide")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        )
        .style(Style::default().fg(Color::White))
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, popup_area);
    f.render_widget(help_paragraph, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
