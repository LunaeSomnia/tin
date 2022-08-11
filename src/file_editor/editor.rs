use vizia::prelude::*;

#[derive(Lens)]
pub struct FileEditorData {
    text: String,
    sel_x: f32,
    re_sel_x: bool,
    edit: bool,
    transform: (f32, f32),
    line_height: f32,
    content_entity: Option<Entity>,
}

impl FileEditorData {
    pub fn new(text: String) -> Self {
        Self {
            text: text.clone(),
            sel_x: -1.0,
            re_sel_x: false,
            edit: false,
            transform: (0.0, 0.0),
            line_height: 0.0,
            content_entity: None,
        }
    }
}

pub enum TextEvent {
    // Helpers
    InitContent(Entity),
    GeometryChanged,
}

impl Model for FileEditorData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|text_event, _| match text_event {
            TextEvent::InitContent(content) => {
                self.content_entity = Some(*content);
            }

            TextEvent::GeometryChanged => {
                // self.set_caret(cx);
            }
        });
    }
}

pub struct FileEditor;

impl FileEditor {
    pub fn new(cx: &mut Context, text: String) -> Handle<Self> {
        let result = Self.build(cx, move |cx| {
            FileEditorData::new(text).build(cx);

            FileEditorContainer
                .build(cx, move |cx| {
                    let lbl = FileEditorLabel
                        .build(cx, |_| {})
                        .hoverable(false)
                        .class("FileEditor_content")
                        .text(FileEditorData::text)
                        // .text_selection(FileEditorData::selection)
                        .translate(FileEditorData::transform)
                        .on_geo_changed(|cx, _| cx.emit(TextEvent::GeometryChanged))
                        .entity;

                    cx.emit(TextEvent::InitContent(lbl));
                })
                .hoverable(false)
                .class("FileEditor_container");
        });

        result.cursor(CursorIcon::Text)
    }
}

impl View for FileEditor {
    fn element(&self) -> Option<&'static str> {
        Some("FileEditor")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        //let selection = cx.tree.get_child(cx.current, 0).unwrap();
        //let caret = cx.tree.get_child(cx.current, 1).unwrap();

        // event.map(|window_event, _| match window_event {
        //     WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
        //         if cx.is_over() {
        //             cx.emit(TextEvent::StartEdit);

        //             cx.focus();
        //             cx.capture();
        //             cx.set_checked(true);
        //             cx.lock_cursor_icon();

        //             cx.emit(TextEvent::Hit(cx.mouse.cursorx, cx.mouse.cursory));
        //         } else {
        //             cx.emit(TextEvent::Submit(false));
        //             if let Some(source) = cx.data::<L::Source>() {
        //                 let text = self.lens.view(source, |t| {
        //                     if let Some(t) = t {
        //                         t.to_string()
        //                     } else {
        //                         "".to_owned()
        //                     }
        //                 });

        //                 cx.emit(TextEvent::SelectAll);
        //                 cx.emit(TextEvent::InsertText(text));
        //             };
        //             cx.release();
        //             cx.set_checked(false);

        //             // Forward event to hovered
        //             cx.event_queue.push_back(
        //                 Event::new(WindowEvent::MouseDown(MouseButton::Left)).target(cx.hovered()),
        //             );
        //         }
        //     }

        //     WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
        //         cx.unlock_cursor_icon();
        //     }

        //     WindowEvent::MouseMove(_, _) => {
        //         if cx.mouse.left.state == MouseButtonState::Pressed {
        //             cx.emit(TextEvent::Drag(cx.mouse.cursorx, cx.mouse.cursory));
        //         }
        //     }

        //     WindowEvent::CharInput(c) => {
        //         if *c != '\u{1b}' && // Escape
        //                     *c != '\u{8}' && // Backspace
        //                     *c != '\u{7f}' && // Delete
        //                     *c != '\u{0d}' && // Carriage return
        //                     !cx.modifiers.contains(Modifiers::CTRL)
        //         {
        //             cx.emit(TextEvent::InsertText(String::from(*c)));
        //         }
        //     }

        //     WindowEvent::KeyDown(code, _) => match code {
        //         Code::Enter => {
        //             // Finish editing
        //             // self.edit = false;

        //             //cx.emit(TextEvent::EndEdit);

        //             if matches!(self.kind, FileEditorKind::SingleLine) {
        //                 cx.emit(TextEvent::Submit(true));
        //                 if let Some(source) = cx.data::<L::Source>() {
        //                     let text = self.lens.view(source, |t| {
        //                         if let Some(t) = t {
        //                             t.to_string()
        //                         } else {
        //                             "".to_owned()
        //                         }
        //                     });

        //                     cx.emit(TextEvent::SelectAll);
        //                     cx.emit(TextEvent::InsertText(text));
        //                 };

        //                 cx.set_checked(false);
        //                 cx.release();
        //             } else {
        //                 cx.emit(TextEvent::InsertText("\n".to_owned()));
        //             }
        //         }

        //         Code::ArrowLeft => {
        //             //if self.edit {
        //             let movement = if cx.modifiers.contains(Modifiers::CTRL) {
        //                 Movement::Word(Direction::Upstream)
        //             } else {
        //                 Movement::Grapheme(Direction::Upstream)
        //             };

        //             cx.emit(TextEvent::MoveCursor(
        //                 movement,
        //                 cx.modifiers.contains(Modifiers::SHIFT),
        //             ));

        //             //self.move_cursor(cx, movement, cx.modifiers.contains(Modifiers::SHIFT));

        //             //self.set_caret(cx, cx.current);
        //             //}
        //         }

        //         Code::ArrowRight => {
        //             //if self.edit {
        //             let movement = if cx.modifiers.contains(Modifiers::CTRL) {
        //                 Movement::Word(Direction::Downstream)
        //             } else {
        //                 Movement::Grapheme(Direction::Downstream)
        //             };

        //             cx.emit(TextEvent::MoveCursor(
        //                 movement,
        //                 cx.modifiers.contains(Modifiers::SHIFT),
        //             ));

        //             // self.move_cursor(cx, movement, cx.modifiers.contains(Modifiers::SHIFT));

        //             // self.set_caret(cx, cx.current);
        //             //}
        //         }

        //         Code::ArrowUp => {
        //             cx.emit(TextEvent::MoveCursor(
        //                 Movement::Line(Direction::Upstream),
        //                 cx.modifiers.contains(Modifiers::SHIFT),
        //             ));
        //         }

        //         Code::ArrowDown => {
        //             cx.emit(TextEvent::MoveCursor(
        //                 Movement::Line(Direction::Downstream),
        //                 cx.modifiers.contains(Modifiers::SHIFT),
        //             ));
        //         }

        //         Code::Backspace => {
        //             if cx.modifiers.contains(Modifiers::CTRL) {
        //                 //self.delete_text(cx, Movement::Word(Direction::Upstream));
        //                 cx.emit(TextEvent::DeleteText(Movement::Word(Direction::Upstream)));
        //             } else {
        //                 //self.delete_text(cx, Movement::Grapheme(Direction::Upstream));
        //                 cx.emit(TextEvent::DeleteText(Movement::Grapheme(
        //                     Direction::Upstream,
        //                 )));
        //             }

        //             //self.set_caret(cx, cx.current);
        //         }

        //         Code::Delete => {
        //             //if self.edit {
        //             if cx.modifiers.contains(Modifiers::CTRL) {
        //                 //self.delete_text(cx, Movement::Word(Direction::Downstream));
        //                 cx.emit(TextEvent::DeleteText(Movement::Word(Direction::Downstream)));
        //             } else {
        //                 //self.delete_text(cx, Movement::Grapheme(Direction::Downstream));
        //                 cx.emit(TextEvent::DeleteText(Movement::Grapheme(
        //                     Direction::Downstream,
        //                 )));
        //             }
        //             //self.set_caret(cx, cx.current);
        //             //}
        //         }

        //         Code::Escape => {
        //             //self.edit = false;
        //             cx.emit(TextEvent::EndEdit);
        //             cx.set_checked(false);
        //         }

        //         Code::Home => {
        //             cx.emit(TextEvent::MoveCursor(
        //                 Movement::ParagraphStart,
        //                 cx.modifiers.contains(Modifiers::SHIFT),
        //             ));
        //         }

        //         Code::End => {
        //             cx.emit(TextEvent::MoveCursor(
        //                 Movement::ParagraphEnd,
        //                 cx.modifiers.contains(Modifiers::SHIFT),
        //             ));
        //         }

        //         // TODO
        //         Code::PageUp => {}

        //         // TODO
        //         Code::PageDown => {}

        //         Code::KeyA => {
        //             //if self.edit {
        //             if cx.modifiers.contains(Modifiers::CTRL) {
        //                 // self.select_all(cx);
        //                 cx.emit(TextEvent::SelectAll);
        //             }
        //             //}
        //         }

        //         Code::KeyC => {
        //             cx.emit(TextEvent::Copy);
        //         }

        //         Code::KeyV => {
        //             cx.emit(TextEvent::Paste);
        //         }

        //         _ => {}
        //     },

        //     _ => {}
        // });
    }
}

// can't just be a stack because what if you've styled stacks
pub struct FileEditorContainer;
impl View for FileEditorContainer {}

// can't just be a label because what if you've styled labels
pub struct FileEditorLabel;
impl View for FileEditorLabel {}
