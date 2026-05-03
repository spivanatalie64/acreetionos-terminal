use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Entry, Orientation, Revealer, Label,
};

pub struct SearchBar {
    revealer: Revealer,
    entry: Entry,
    match_count: Label,
}

impl SearchBar {
    pub fn new() -> Self {
        let match_count = Label::new(None);
        match_count.set_margin_start(8);
        match_count.set_margin_end(8);

        let entry = Entry::new();
        entry.set_placeholder_text(Some("Search..."));

        let up_btn = Button::with_label("▲");
        up_btn.set_tooltip_text(Some("Previous match"));

        let down_btn = Button::with_label("▼");
        down_btn.set_tooltip_text(Some("Next match"));

        let close_btn = Button::with_label("✕");
        close_btn.set_tooltip_text(Some("Close search"));

        let hbox = GtkBox::new(Orientation::Horizontal, 4);
        hbox.set_margin_start(8);
        hbox.set_margin_end(8);
        hbox.set_margin_top(4);
        hbox.set_margin_bottom(4);
        hbox.append(&entry);
        hbox.append(&match_count);
        hbox.append(&up_btn);
        hbox.append(&down_btn);
        hbox.append(&close_btn);

        let revealer = Revealer::new();
        revealer.set_child(Some(&hbox));
        revealer.set_transition_type(gtk4::RevealerTransitionType::SlideDown);
        revealer.set_reveal_child(false);

        let search = SearchBar { revealer, entry, match_count };

        close_btn.connect_clicked({
            let revealer = search.revealer.clone();
            move |_| {
                revealer.set_reveal_child(false);
            }
        });

        search
    }

    pub fn widget(&self) -> &Revealer {
        &self.revealer
    }

    pub fn is_visible(&self) -> bool {
        self.revealer.reveals_child()
    }

    pub fn toggle(&self) {
        let visible = self.revealer.reveals_child();
        self.revealer.set_reveal_child(!visible);
        if !visible {
            self.entry.grab_focus();
        }
    }

    pub fn set_match_count(&self, current: usize, total: usize) {
        self.match_count.set_text(&format!("{}/{}", current, total));
    }

    pub fn clear_match_count(&self) {
        self.match_count.set_text("");
    }
}
