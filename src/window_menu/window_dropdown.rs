use vizia::{modifiers::Press, prelude::*};

use crate::*;

pub struct WindowDropdown;

impl WindowDropdown {
    pub fn new<F>(cx: &mut Context, label: String, content: F) -> Handle<Self>
    where
        F: 'static + Fn(&mut Context),
    {
        Self {}.build(cx, move |cx| {
            PopupData::default().build(cx);

            Binding::new(cx, PopupData::is_open, move |cx, is_open| {
                let is_open = is_open.get(cx);

                let label = HStack::new(cx, |cx| {
                    let label = Label::new(cx, &label).color(BG_6).class("title");

                    if !is_open {
                        label.color(BG_5);
                    }
                })
                .on_press(|cx| cx.emit(PopupEvent::Switch))
                .background_color(BG_0)
                .class("label-wrapper");

                if is_open {
                    label.background_color(BG_1);
                }
            });

            Popup::new(cx, PopupData::is_open, move |cx| {
                content(cx);
            })
            .on_blur(|cx| cx.emit(PopupEvent::Close))
            .top(Percentage(100.0))
            .height(Auto)
            .border_color(BG_3)
            .background_color(BG_2);
        })
    }
}

impl View for WindowDropdown {
    fn element(&self) -> Option<&'static str> {
        Some("window-dropdown")
    }
}

pub struct WindowDropdownItem;

impl WindowDropdownItem {
    pub fn new<M>(cx: &mut Context, label: String, event: M) -> Handle<Press<Self>>
    where
        M: Message + Clone,
    {
        Self {}
            .build(cx, |cx| {
                Label::new(cx, &label).hoverable(false);
            })
            .on_press(move |ex| {
                ex.emit(PopupEvent::Close);
                ex.emit(event.clone());
            })
            .hoverable(true)
    }
}

impl View for WindowDropdownItem {
    fn element(&self) -> Option<&'static str> {
        Some("window-dropdown-item")
    }
}
