mod app;
mod file;

use app::{AppEvent, AppState};
use vizia::prelude::*;

const TIN_ICONS: &[u8] = include_bytes!("../static/TinIcons.otf");

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("static/styles.css")
            .expect("Failed to find file");

        cx.add_font_mem("TinIcons", TIN_ICONS);

        AppState::default().build(cx);

        VStack::new(cx, |cx| {
            Button::new(
                cx,
                |cx| cx.emit(AppEvent::Test),
                |cx| Label::new(cx, "Press me"),
            );

            // TODO: Improve the font file
            Label::new(cx, "ABC").font("TinIcons").font_size(18.0);
            Label::new(cx, "abc").font("TinIcons").font_size(14.0);

            Binding::new(cx, AppState::file, move |cx, file| {
                if let Some(f) = file.get(cx) {
                    Label::new(cx, &f.data);
                }
            });
        });
    })
    .title("Tin")
    .inner_size((1000, 800))
    .run();
}
