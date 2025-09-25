use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Label, Orientation};

fn main() {
    let app = Application::builder()
        .application_id("dev.ultimate_garuda.eartrumpet")
        .build();

    app.connect_activate(|app| {
        let container = GtkBox::new(Orientation::Vertical, 12);
        let label = Label::new(Some(
            "EarTrumpet (Linux) — WIP\n\nTray‑first per‑app audio mixer for PipeWire.\n\nThis is a placeholder UI for the MVP scaffold.",
        ));
        container.append(&label);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("EarTrumpet (Linux) — WIP")
            .default_width(480)
            .default_height(320)
            .child(&container)
            .build();

        window.present();
    });

    app.run();
}
