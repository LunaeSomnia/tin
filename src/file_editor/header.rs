use vizia::prelude::*;

use crate::{ACCENT, BG_0, BG_1, BG_2, BG_5, BG_6};

use super::file::File;

#[derive(Lens, Data, Clone)]
pub struct FileHeader {
    is_open: bool,
}

pub enum FileHeaderEvent {
    Open,
    Close,
    Toggle,
}

impl Model for FileHeader {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|e, _| match e {
            FileHeaderEvent::Open => self.is_open = true,
            FileHeaderEvent::Close => self.is_open = false,
            FileHeaderEvent::Toggle => self.is_open = !self.is_open,
        })
    }
}

impl FileHeader {
    pub fn new(cx: &mut Context, file: File) {
        let cursor = CursorIcon::Hand;
        Self { is_open: true }.build(cx);

        Binding::new(cx, FileHeader::is_open, move |cx, is_open| {
            let is_open = is_open.get(cx);

            let header = Button::new(
                cx,
                |ex| ex.emit(FileHeaderEvent::Toggle),
                |cx| {
                    let label = Label::new(cx, &file.name())
                        .class("file-name")
                        .color(BG_5)
                        .cursor(cursor);
                    if is_open {
                        label.color(BG_6);
                    }

                    let color_strip = HStack::new(cx, |_| {})
                        .class("file-header-colored")
                        .background_color(ACCENT)
                        .cursor(cursor);

                    if is_open {
                        color_strip
                    } else {
                        color_strip.background_color(Color::rgba(0, 0, 0, 0))
                    }
                },
            )
            .background_color(BG_1)
            .class("file-header")
            // .display(Display::Flex)
            .layout_type(LayoutType::Column)
            .cursor(cursor);

            if is_open {
                header.background_color(BG_2);
            }
        });
    }
}
