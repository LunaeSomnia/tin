pub mod window_dropdown;

use vizia::prelude::*;

use crate::{app::AppEvent, BG_0, BG_1, BG_2, BG_6};

use self::window_dropdown::{WindowDropdown, WindowDropdownItem};

pub struct WindowMenu;

impl View for WindowMenu {}

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

                    WindowDropdown::new(cx, "Edit".to_owned(), |cx| {});
                    WindowDropdown::new(cx, "Help".to_owned(), |cx| {});
                })
                .class("window-menu-wrapper");
            })
            .class("window-menu")
            .background_color(BG_0)
    }
}
