use crate::categories::{get_all_categories, Category, Software};
use ratatui::widgets::ListState;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    MethodSelection,
    CategorySelection,
    SoftwareSelection,
    SearchMode,
    Installing,
    ShowResult,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstallMethod {
    PowerShell,
    Chocolatey,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OperationMode {
    Install,
    Uninstall,
    Reinstall,
}

pub struct App {
    pub state: AppState,
    pub method_index: usize,
    pub category_list_state: ListState,
    pub software_list_state: ListState,
    pub search_list_state: ListState,
    pub selected_method: Option<InstallMethod>,
    pub operation_mode: OperationMode,
    pub categories: Vec<Category>,
    pub current_category_index: usize,
    pub result_message: String,
    pub show_help: bool,
    pub installing_progress: usize,
    pub total_to_install: usize,
    pub search_input: String,
    pub search_results: Vec<(usize, usize, Software)>,
    pub installation_output: Vec<String>,
    pub current_installing_software: String,
    pub installation_complete: bool,
    pub log_receiver: Option<Receiver<String>>,
    pub fast_startup_mode: bool,
    pub scanning_in_progress: bool,
    pub scanning_spinner_frame: usize,
    pub last_scan_time: Option<Instant>,
    pub scan_receiver: Option<Receiver<(usize, usize, bool)>>,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            state: AppState::MethodSelection,
            method_index: 0,
            category_list_state: ListState::default(),
            software_list_state: ListState::default(),
            search_list_state: ListState::default(),
            selected_method: None,
            operation_mode: OperationMode::Install,
            categories: get_all_categories(),
            current_category_index: 0,
            result_message: String::new(),
            show_help: false,
            installing_progress: 0,
            total_to_install: 0,
            search_input: String::new(),
            search_results: Vec::new(),
            installation_output: Vec::new(),
            current_installing_software: String::new(),
            installation_complete: false,
            log_receiver: None,
            fast_startup_mode: false,
            scanning_in_progress: false,
            scanning_spinner_frame: 0,
            last_scan_time: None,
            scan_receiver: None,
        };
        app.category_list_state.select(Some(0));
        app.software_list_state.select(Some(0));
        app.search_list_state.select(Some(0));
        app
    }
}

impl App {
    pub fn set_fast_startup_mode(&mut self, enabled: bool) {
        self.fast_startup_mode = enabled;
    }

    pub fn is_fast_startup_mode(&self) -> bool {
        self.fast_startup_mode
    }

    pub fn start_comprehensive_scan(&mut self) {
        if self.scanning_in_progress {
            return;
        }

        self.scanning_in_progress = true;
        self.last_scan_time = Some(Instant::now());

        let (scan_sender, scan_receiver) = mpsc::channel();
        self.scan_receiver = Some(scan_receiver);

        let categories_clone = self.categories.clone();
        thread::spawn(move || {
            crate::installer::Installer::comprehensive_system_scan(categories_clone, scan_sender);
        });
    }

    pub fn start_uninstall_mode(&mut self) {
        self.operation_mode = OperationMode::Uninstall;
        // Clear selections for non-installed software
        for category in &mut self.categories {
            for software in &mut category.software {
                if !software.is_installed {
                    software.is_selected = false;
                }
            }
        }
    }

    pub fn start_reinstall_mode(&mut self) {
        self.operation_mode = OperationMode::Reinstall;
        // Clear selections for non-installed software
        for category in &mut self.categories {
            for software in &mut category.software {
                if !software.is_installed {
                    software.is_selected = false;
                }
            }
        }
    }

    pub fn start_install_mode(&mut self) {
        self.operation_mode = OperationMode::Install;
        // No need to clear selections for install mode
    }

    pub fn get_operation_text(&self) -> &str {
        match self.operation_mode {
            OperationMode::Install => "Install",
            OperationMode::Uninstall => "Uninstall",
            OperationMode::Reinstall => "Reinstall",
        }
    }

    pub fn process_scan_results(&mut self) {
        if let Some(ref receiver) = self.scan_receiver {
            while let Ok((cat_idx, soft_idx, is_installed)) = receiver.try_recv() {
                if cat_idx < self.categories.len()
                    && soft_idx < self.categories[cat_idx].software.len()
                {
                    self.categories[cat_idx].software[soft_idx].is_installed = is_installed;
                }
            }
        }
    }

    pub fn on_tick(&mut self) {
        self.scanning_spinner_frame = (self.scanning_spinner_frame + 1) % 4;
        self.process_scan_results();

        if self.scanning_in_progress {
            if let Some(ref receiver) = self.scan_receiver {
                if receiver.try_recv().is_err() {
                    if let Some(start_time) = self.last_scan_time {
                        if start_time.elapsed().as_secs() > 15 {
                            self.scanning_in_progress = false;
                        }
                    }
                }
            }
        }
    }

    pub fn get_scanning_status(&self) -> String {
        if self.scanning_in_progress {
            let spinner_chars = ['|', '/', '-', '\\'];
            let spinner = spinner_chars[self.scanning_spinner_frame];
            format!("{} Scanning installed software...", spinner)
        } else {
            "Ready".to_string()
        }
    }

    pub fn next_method(&mut self) {
        self.method_index = (self.method_index + 1) % 2;
    }

    pub fn previous_method(&mut self) {
        self.method_index = if self.method_index == 0 { 1 } else { 0 };
    }

    pub fn next_category(&mut self) {
        let i = match self.category_list_state.selected() {
            Some(i) => (i + 1) % self.categories.len(),
            None => 0,
        };
        self.category_list_state.select(Some(i));
        self.current_category_index = i;
        self.software_list_state.select(Some(0));
        // Reset to install mode when changing categories
        self.operation_mode = OperationMode::Install;
    }

    pub fn previous_category(&mut self) {
        let i = match self.category_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.categories.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.category_list_state.select(Some(i));
        self.current_category_index = i;
        self.software_list_state.select(Some(0));
        // Reset to install mode when changing categories
        self.operation_mode = OperationMode::Install;
    }

    pub fn next_software(&mut self) {
        let len = self.categories[self.current_category_index].software.len();
        if len > 0 {
            let i = match self.software_list_state.selected() {
                Some(i) => (i + 1) % len,
                None => 0,
            };
            self.software_list_state.select(Some(i));
        }
    }

    pub fn previous_software(&mut self) {
        let len = self.categories[self.current_category_index].software.len();
        if len > 0 {
            let i = match self.software_list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        len - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.software_list_state.select(Some(i));
        }
    }

    pub fn next_search_result(&mut self) {
        if !self.search_results.is_empty() {
            let i = match self.search_list_state.selected() {
                Some(i) => (i + 1) % self.search_results.len(),
                None => 0,
            };
            self.search_list_state.select(Some(i));
        }
    }

    pub fn previous_search_result(&mut self) {
        if !self.search_results.is_empty() {
            let i = match self.search_list_state.selected() {
                Some(i) => {
                    if i == 0 {
                        self.search_results.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            };
            self.search_list_state.select(Some(i));
        }
    }

    // FIXED: Only allow selection based on operation mode and installation status
    pub fn toggle_software_selection(&mut self) {
        if let Some(software_index) = self.software_list_state.selected() {
            if software_index < self.categories[self.current_category_index].software.len() {
                let software =
                    &mut self.categories[self.current_category_index].software[software_index];

                // Only allow selection based on operation mode and installation status
                match self.operation_mode {
                    OperationMode::Install => {
                        // Can always select for install
                        software.toggle_selection();
                    }
                    OperationMode::Uninstall | OperationMode::Reinstall => {
                        // Can only select if software is installed
                        if software.is_installed {
                            software.toggle_selection();
                        }
                        // Ignore selection attempt if not installed
                    }
                }
            }
        }
    }

    pub fn toggle_search_selection(&mut self) {
        if let Some(result_index) = self.search_list_state.selected() {
            let indices =
                if let Some((cat_idx, soft_idx, _)) = self.search_results.get(result_index) {
                    Some((*cat_idx, *soft_idx))
                } else {
                    None
                };

            if let Some((cat_idx, soft_idx)) = indices {
                if cat_idx < self.categories.len()
                    && soft_idx < self.categories[cat_idx].software.len()
                {
                    let software = &mut self.categories[cat_idx].software[soft_idx];

                    match self.operation_mode {
                        OperationMode::Install => {
                            software.toggle_selection();
                        }
                        OperationMode::Uninstall | OperationMode::Reinstall => {
                            if software.is_installed {
                                software.toggle_selection();
                            }
                        }
                    }
                    self.update_search_results();
                }
            }
        }
    }

    pub fn select_all_in_category(&mut self) {
        let category = &mut self.categories[self.current_category_index];

        let relevant_software: Vec<_> = category
            .software
            .iter()
            .filter(|s| match self.operation_mode {
                OperationMode::Install => true,
                OperationMode::Uninstall | OperationMode::Reinstall => s.is_installed,
            })
            .collect();

        if relevant_software.is_empty() {
            return;
        }

        let all_relevant_selected = relevant_software.iter().all(|s| s.is_selected);

        for software in &mut category.software {
            let should_toggle = match self.operation_mode {
                OperationMode::Install => true,
                OperationMode::Uninstall | OperationMode::Reinstall => software.is_installed,
            };

            if should_toggle {
                software.is_selected = !all_relevant_selected;
            }
        }
    }

    pub fn get_current_category(&self) -> &Category {
        &self.categories[self.current_category_index]
    }

    pub fn get_total_selected(&self) -> usize {
        self.categories
            .iter()
            .flat_map(|c| &c.software)
            .filter(|s| s.is_selected)
            .count()
    }

    pub fn update_search_results(&mut self) {
        self.search_results.clear();

        if self.search_input.trim().is_empty() {
            self.search_list_state.select(None);
            return;
        }

        let search_term = self.search_input.to_lowercase();

        for (cat_idx, category) in self.categories.iter().enumerate() {
            for (soft_idx, software) in category.software.iter().enumerate() {
                if software.name.to_lowercase().contains(&search_term)
                    || software.description.to_lowercase().contains(&search_term)
                {
                    self.search_results
                        .push((cat_idx, soft_idx, software.clone()));
                }
            }
        }

        if !self.search_results.is_empty() {
            self.search_list_state.select(Some(0));
        }
    }

    pub fn add_installation_output(&mut self, line: String) {
        self.installation_output.push(line);
        if self.installation_output.len() > 100 {
            self.installation_output.remove(0);
        }
    }

    pub fn clear_installation_output(&mut self) {
        self.installation_output.clear();
        self.current_installing_software.clear();
    }

    pub fn process_log_messages(&mut self) {
        let messages: Vec<String> = if let Some(ref receiver) = self.log_receiver {
            let mut msgs = Vec::new();
            while let Ok(message) = receiver.try_recv() {
                msgs.push(message);
            }
            msgs
        } else {
            Vec::new()
        };

        for message in messages {
            if message.starts_with("INSTALLING:") {
                self.current_installing_software =
                    message.replace("INSTALLING:", "").trim().to_string();
            } else if message.starts_with("UNINSTALLING:") {
                self.current_installing_software =
                    message.replace("UNINSTALLING:", "").trim().to_string();
            } else if message.starts_with("REINSTALLING:") {
                self.current_installing_software =
                    message.replace("REINSTALLING:", "").trim().to_string();
            } else if message.starts_with("PROGRESS:") {
                if let Ok(progress) = message.replace("PROGRESS:", "").trim().parse::<usize>() {
                    self.installing_progress = progress;
                }
            } else if message == "INSTALLATION_COMPLETE" {
                self.installation_complete = true;
                self.current_installing_software.clear();
            } else {
                self.add_installation_output(message);
            }
        }
    }

    pub fn set_log_receiver(&mut self, receiver: Receiver<String>) {
        self.log_receiver = Some(receiver);
    }
}
