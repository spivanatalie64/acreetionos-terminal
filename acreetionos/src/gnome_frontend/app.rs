use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::Application;

use crate::config::UiConfig;
use crate::gnome_frontend::window::AcreetionOSWindow;

const APP_ID: &str = "com.acreetionos.Terminal";

pub fn run() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_window);

    app.run();
}

fn build_window(app: &Application) {
    let config = Rc::new(UiConfig::default());
    let window = AcreetionOSWindow::new(app, config);
    window.present();
}
