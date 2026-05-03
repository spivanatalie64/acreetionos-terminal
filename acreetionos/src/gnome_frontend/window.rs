use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{
    AboutDialog, Application, ApplicationWindow, Box as GtkBox, Button, HeaderBar,
    Label, MenuButton, Notebook, Orientation, Popover,
};

use crate::config::UiConfig;
use crate::gnome_frontend::terminal_widget::TerminalWidget;

pub struct AcreetionOSWindow {
    window: ApplicationWindow,
    notebook: Notebook,
    config: Rc<UiConfig>,
}

impl AcreetionOSWindow {
    pub fn new(app: &Application, config: Rc<UiConfig>) -> Self {
        let notebook = Notebook::new();
        notebook.set_show_tabs(true);
        notebook.set_scrollable(true);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("AcreetionOS Terminal")
            .default_width(960)
            .default_height(540)
            .child(&notebook)
            .build();

        let header_bar = Self::create_header_bar();
        window.set_titlebar(Some(&header_bar));

        let acreetionos_window = Self { window, notebook, config };

        acreetionos_window.add_terminal_tab();

        acreetionos_window
    }

    pub fn present(&self) {
        self.window.present();
    }

    fn create_header_bar() -> HeaderBar {
        let header_bar = HeaderBar::new();

        let menu_button = Self::create_menu_button();
        header_bar.pack_end(&menu_button);

        header_bar
    }

    fn create_menu_button() -> MenuButton {
        let menu_button = MenuButton::new();
        menu_button.set_icon_name("open-menu-symbolic");

        let popover = Popover::new();

        let preferences_btn = Button::with_label("Preferences");
        preferences_btn.connect_clicked(|_| {
            let prefs = super::preferences::PreferencesDialog::new();
            prefs.present();
        });

        let about_btn = Button::with_label("About AcreetionOS Terminal");
        about_btn.connect_clicked(|_| {
            let about = AboutDialog::new();
            about.set_program_name(Some("AcreetionOS Terminal"));
            about.set_version(Some("0.17.0"));
            about.set_comments(Some("GPU-accelerated terminal emulator for AcreetionOS"));
            about.set_license_type(gtk4::License::Apache20);
            about.present();
        });

        let vbox = GtkBox::new(Orientation::Vertical, 0);
        vbox.append(&preferences_btn);
        vbox.append(&about_btn);
        popover.set_child(Some(&vbox));
        menu_button.set_popover(Some(&popover));

        menu_button
    }

    fn add_terminal_tab(&self) {
        let terminal = TerminalWidget::new(self.config.clone());
        let label = Self::create_tab_label("Terminal");
        let page_index = self.notebook.append_page(&terminal.widget(), Some(&label));
        self.notebook.set_current_page(Some(page_index));
    }

    fn create_tab_label(title: &str) -> GtkBox {
        let label = Label::new(Some(title));
        let close_btn = Button::with_label("✕");
        close_btn.set_css_classes(&["close-button"]);

        let box_ = GtkBox::new(Orientation::Horizontal, 4);
        box_.append(&label);
        box_.append(&close_btn);
        box_
    }

    pub fn add_tab(&self) {
        self.add_terminal_tab();
    }

    pub fn close_current_tab(&self) {
        if let Some(page) = self.notebook.current_page() {
            if self.notebook.n_pages() > 1 {
                self.notebook.remove_page(Some(page));
            }
        }
    }
}
