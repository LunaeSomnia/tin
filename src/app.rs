use std::path::PathBuf;
use std::str::FromStr;

use tinyfiledialogs::open_file_dialog;
use vizia::prelude::*;
use vizia::state::Model;

use crate::file_editor::file::File;

#[derive(Lens)]
pub struct AppState {
    pub file: Option<File>,
    pub editing: Option<String>,
}

#[derive(Clone, Copy)]
pub enum AppEvent {
    NewFile,
    OpenFile,
    CloseFile,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            file: None,
            editing: None,
        }
    }
}

impl Model for AppState {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::NewFile => {}
            AppEvent::OpenFile => {
                if let Some(path) = open_file_dialog("Open File", "", None) {
                    let path = PathBuf::from_str(&path).unwrap().canonicalize().unwrap();
                    println!("{:?}", &path);
                    // TODO: Change this to allow multiple files in a future
                    let file = File::new(path).unwrap();
                    self.editing = Some(file.data.clone());
                    self.file = Some(file);
                }
            }
            AppEvent::CloseFile => self.file = None,
        })
    }
}
