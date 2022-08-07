mod app;
mod file;

use app::{AppEvent, AppState};
use vizia::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    Application::new(|cx| {
        cx.add_stylesheet("static/styles.css")
            .expect("Failed to find file");

        AppState::default().build(cx);

        VStack::new(cx, |cx| {
            Button::new(
                cx,
                |cx| cx.emit(AppEvent::Test),
                |cx| Label::new(cx, "Press me"),
            );

            Binding::new(cx, AppState::file, move |cx, file| {
                if let Some(f) = file.get(cx) {
                    Label::new(cx, &f.data);
                }
            });
        });
    })
    .title("Tin")
    .inner_size((450, 600))
    .run();
}
