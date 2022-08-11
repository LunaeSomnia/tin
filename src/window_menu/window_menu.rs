use vizia::prelude::*;

use crate::{app::AppEvent, *};

use super::{WindowDropdown, WindowDropdownItem};

pub struct WindowMenu;

impl View for WindowMenu {
    fn element(&self) -> Option<&'static str> {
        Some("window-menu")
    }
}

impl WindowMenu {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        //window-menu
        WindowMenu {}
            .build(cx, move |cx| {
                HStack::new(cx, |cx| {
                    WindowDropdown::new(cx, "File".to_owned(), |cx| {
                        WindowDropdownItem::new(cx, "New File".to_owned(), AppEvent::NewFile);
                        WindowDropdownItem::new(cx, "Open File".to_owned(), AppEvent::OpenFile);
                    });

                    WindowDropdown::new(cx, "Edit".to_owned(), |_| {});
                    WindowDropdown::new(cx, "Help".to_owned(), |_| {});
                });
            })
            .background_color(BG_0)
    }
}
