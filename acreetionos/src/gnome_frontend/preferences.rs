use gtk4::prelude::*;
use gtk4::{Adjustment, Button, ComboBoxText, Label, SpinButton};
use libadwaita::prelude::*;
use libadwaita::{ActionRow, PreferencesGroup, PreferencesPage, PreferencesWindow};

pub struct PreferencesDialog {
    window: PreferencesWindow,
}

impl PreferencesDialog {
    pub fn new() -> Self {
        let window = PreferencesWindow::new();
        window.set_title(Some("Preferences"));
        window.set_default_size(600, 500);

        let appearance_page = Self::create_appearance_page();
        window.add(&appearance_page);

        let terminal_page = Self::create_terminal_page();
        window.add(&terminal_page);

        let shortcuts_page = Self::create_shortcuts_page();
        window.add(&shortcuts_page);

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }

    fn create_appearance_page() -> PreferencesPage {
        let page = PreferencesPage::new();
        page.set_title("Appearance");

        let group = PreferencesGroup::new();
        group.set_title("Appearance");

        let font_row = Self::create_font_row();
        group.add(&font_row);

        let theme_row = Self::create_theme_row();
        group.add(&theme_row);

        let opacity_row = Self::create_opacity_row();
        group.add(&opacity_row);

        page.add(&group);
        page
    }

    fn create_terminal_page() -> PreferencesPage {
        let page = PreferencesPage::new();
        page.set_title("Terminal");

        let group = PreferencesGroup::new();
        group.set_title("Terminal");

        let cursor_row = Self::create_cursor_row();
        group.add(&cursor_row);

        let scrollback_row = Self::create_scrollback_row();
        group.add(&scrollback_row);

        page.add(&group);
        page
    }

    fn create_shortcuts_page() -> PreferencesPage {
        let page = PreferencesPage::new();
        page.set_title("Shortcuts");

        let group = PreferencesGroup::new();
        group.set_title("Shortcuts");

        let label = Label::new(Some("Shortcuts configuration coming soon"));
        label.set_margin_top(12);
        label.set_margin_bottom(12);
        group.add(&label);

        page.add(&group);
        page
    }

    fn create_font_row() -> ActionRow {
        let row = ActionRow::new();
        row.set_title("Terminal Font");
        row.set_subtitle("Monospace");

        let font_btn = Button::with_label("Select Font...");
        row.add_suffix(&font_btn);
        row.set_activatable_widget(Some(&font_btn));

        row
    }

    fn create_theme_row() -> ActionRow {
        let row = ActionRow::new();
        row.set_title("Theme");
        row.set_subtitle("Color scheme");

        let combo = ComboBoxText::new();
        combo.append_text("Dark");
        combo.append_text("Light");
        combo.append_text("System Default");
        combo.set_active(Some(0));
        row.add_suffix(&combo);

        row
    }

    fn create_opacity_row() -> ActionRow {
        let row = ActionRow::new();
        row.set_title("Opacity");
        row.set_subtitle("Terminal background opacity");

        let adjustment = Adjustment::new(1.0, 0.1, 1.0, 0.05, 0.1, 0.0);
        let spinner = SpinButton::new(Some(&adjustment), 0.05, 2);
        row.add_suffix(&spinner);

        row
    }

    fn create_cursor_row() -> ActionRow {
        let row = ActionRow::new();
        row.set_title("Cursor Style");
        row.set_subtitle("Block, Beam, or Underline");

        let combo = ComboBoxText::new();
        combo.append_text("Block");
        combo.append_text("Beam");
        combo.append_text("Underline");
        combo.set_active(Some(0));
        row.add_suffix(&combo);

        row
    }

    fn create_scrollback_row() -> ActionRow {
        let row = ActionRow::new();
        row.set_title("Scrollback Lines");
        row.set_subtitle("Number of lines to keep in history");

        let adjustment = Adjustment::new(10000.0, 100.0, 100000.0, 100.0, 1000.0, 0.0);
        let spinner = SpinButton::new(Some(&adjustment), 100.0, 0);
        row.add_suffix(&spinner);

        row
    }
}
