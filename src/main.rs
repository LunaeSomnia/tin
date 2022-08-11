mod app;
mod file_editor;
mod window_menu;

use app::AppState;
use vizia::prelude::*;

const TIN_ICONS: &[u8] = include_bytes!("../static/TinIcons.otf");
const BERGEN_MONO_REG: &[u8] = include_bytes!("../static/BergenMono-Regular.otf");
const BERGEN_MONO_SMB: &[u8] = include_bytes!("../static/BergenMono-SemiBold.otf");

pub const BG_0: Color = Color::rgb(13, 13, 13);
pub const BG_1: Color = Color::rgb(23, 23, 23);
pub const BG_2: Color = Color::rgb(36, 36, 36);
pub const BG_3: Color = Color::rgb(63, 63, 63);
pub const BG_4: Color = Color::rgb(91, 91, 91);
pub const BG_5: Color = Color::rgb(136, 136, 136);
pub const BG_6: Color = Color::rgb(255, 255, 255);
pub const ACCENT: Color = Color::rgb(36, 0, 255);

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use window_menu::WindowMenu;

    use crate::file_editor::header::FileHeader;

    Application::new(|cx| {
        cx.add_stylesheet("static/styles.css")
            .expect("Failed to find file");

        cx.add_font_mem("TinIcons", TIN_ICONS);
        cx.add_font_mem("Bergen Mono Regular", BERGEN_MONO_REG);
        cx.add_font_mem("Bergen Mono SemiBold", BERGEN_MONO_SMB);

        AppState::default().build(cx);

        VStack::new(cx, |cx| {
            WindowMenu::new(cx);
            // wrapper
            HStack::new(cx, |cx| {
                // app-content
                VStack::new(cx, |cx| {
                    Binding::new(cx, AppState::file, |cx, file| {
                        if let Some(file) = file.get(cx) {
                            let text = file.data.clone();

                            // files-header
                            HStack::new(cx, |cx| {
                                FileHeader::new(cx, file);
                                HStack::new(cx, |_| {})
                                    .class("files-header-fill")
                                    .background_color(BG_1);
                            })
                            .background_color(BG_3)
                            .class("files-header");

                            // file
                            HStack::new(cx, |cx| {
                                ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                                    Label::new(cx, &text).class("file-data");
                                });
                            })
                            .class("file")
                            .background_color(BG_1);
                        } else {
                        }
                    });
                })
                .class("app-content")
                .background_color(BG_0);

                // sidebar
                VStack::new(cx, |cx| {})
                    .class("sidebar")
                    .background_color(BG_0)
                    .width(Units::Pixels(32.0));
            })
            .class("wrapper")
            .border_color(BG_3)
            .background_color(BG_3);
        });

        // VStack::new(cx, |cx| {
        //     Button::new(
        //         cx,
        //         |cx| cx.emit(AppEvent::Test),
        //         |cx| Label::new(cx, "Press me"),
        //     );

        //     // TODO: Improve the font file
        //     Label::new(cx, "ABC").font("TinIcons").font_size(18.0);
        //     Label::new(cx, "abc").font("TinIcons").font_size(14.0);

        //     // ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
        //     HStack::new(cx, |cx| {
        //         Binding::new(cx, AppState::file, move |cx, file| {
        //             if let Some(f) = file.get(cx) {
        //                 // Label::new(cx, &f.data);
        //                 println!("{}", &f.data);
        //                 FileEditor::new(cx, f.data.clone())
        //                     .width(Units::Auto)
        //                     .height(Units::Auto);
        //             }
        //         });
        //     });
        //     // });
        // });
    })
    .background_color(Color::rgb(127, 127, 127))
    .title("Tin")
    .inner_size((1000, 800))
    .run();
}
