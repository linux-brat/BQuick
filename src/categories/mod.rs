pub mod browsers;
pub mod dev_tools;
pub mod gaming;
pub mod media;
pub mod utilities;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Software {
    pub name: &'static str,
    pub powershell_command: &'static str,
    pub choco_command: &'static str,
    pub uninstall_command: &'static str,
    pub check_command: &'static str,
    pub repair_command: Option<&'static str>,
    pub description: &'static str,
    pub is_installed: bool,
    pub is_selected: bool,
}

impl Software {
    pub fn new(
        name: &'static str,
        powershell_command: &'static str,
        choco_command: &'static str,
        uninstall_command: &'static str,
        check_command: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            name,
            powershell_command,
            choco_command,
            uninstall_command,
            check_command,
            repair_command: None,
            description,
            is_installed: false,
            is_selected: false,
        }
    }

    pub fn with_repair(mut self, repair_command: &'static str) -> Self {
        self.repair_command = Some(repair_command);
        self
    }

    pub fn toggle_selection(&mut self) {
        self.is_selected = !self.is_selected;
    }
}

#[derive(Debug, Clone)]
pub struct Category {
    pub name: &'static str,
    pub software: Vec<Software>,
    pub icon: &'static str,
}

impl Category {
    pub fn new(name: &'static str, icon: &'static str) -> Self {
        Self {
            name,
            software: Vec::new(),
            icon,
        }
    }

    pub fn add_software(&mut self, software: Software) {
        self.software.push(software);
    }

    pub fn get_selected_count(&self) -> usize {
        self.software.iter().filter(|s| s.is_selected).count()
    }

    #[allow(dead_code)]
    pub fn get_installed_count(&self) -> usize {
        self.software.iter().filter(|s| s.is_installed).count()
    }
}

pub fn get_all_categories() -> Vec<Category> {
    vec![
        browsers::get_browsers_category(),
        dev_tools::get_dev_tools_category(),
        media::get_media_category(),
        utilities::get_utilities_category(),
        gaming::get_gaming_category(),
    ]
}
