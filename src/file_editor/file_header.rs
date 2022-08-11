use vizia::prelude::*;

use crate::{
    app::AppEvent,
    svg::{Svg, SvgIcon},
    *,
};

use super::file::File;

#[derive(Lens, Data, Clone)]
pub struct FileHeader {
    is_open: bool,
}

#[allow(dead_code)]
pub enum FileHeaderEvent {
    Open,
    Close,
    Toggle,
}

impl Model for FileHeader {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
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

            // file-header
            VStack::new(cx, |cx| {
                // file-header-content
                HStack::new(cx, |cx| {
                    // label
                    Label::new(cx, &file.name())
                        .color(if is_open { BG_6 } else { BG_5 })
                        .cursor(cursor)
                        .class("label");

                    // svg-wrapper
                    HStack::new(cx, |cx| {
                        Svg::new(cx, SvgIcon::X)
                            .size(Units::Pixels(18.0))
                            .cursor(cursor);
                    })
                    .on_press(|ex| ex.emit(AppEvent::CloseFile))
                    .hoverable(true)
                    .cursor(cursor)
                    .class("svg-wrapper");
                })
                .on_press(|ex| ex.emit(FileHeaderEvent::Toggle))
                .cursor(cursor)
                .class("content");

                // colored-strip
                HStack::new(cx, |_| {})
                    .background_color(if is_open { ACCENT } else { BG_3 })
                    .cursor(cursor)
                    .class("colored-strip");
            })
            .background_color(if is_open { BG_2 } else { BG_1 })
            .layout_type(LayoutType::Column)
            .cursor(cursor)
            .class("file-header");
        });
    }
}
