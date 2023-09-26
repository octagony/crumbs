use std::{
    fs,
    io::ErrorKind,
    path::{self, PathBuf},
};

use fltk::{
    enums::Color,
    input::{FloatInput, Input},
    prelude::*,
    text::{TextBuffer, TextDisplay},
    *,
};

use fltk_theme::{ThemeType, WidgetTheme};

fn read_dir() -> Vec<String> {
    let paths = match fs::read_dir("/usr/bin") {
        Err(e) if e.kind() == ErrorKind::NotFound => Vec::new(),

        Err(e) => panic!("Unexpected Error! {:?}", e),

        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.path().into_os_string().into_string().unwrap())
            .collect(),
    };
    paths
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let binaries = read_dir();

    let mut wind = window::Window::default()
        .with_size(400, 300)
        .with_label("Crumbs")
        .with_size(400, 300)
        .with_label("Crumbs");

    let mut disp = TextDisplay::default().with_size(1700, 100);

    let mut inp1 = FloatInput::new(0, 0, 50, 50, "").center_y(&wind);

    let mut btn = button::Button::default()
        .with_size(80, 30)
        .center_of_parent()
        .with_label("Click");
    theme_button(&mut btn);

    wind.end();
    wind.show();

    btn.set_callback(move |b| {
        b.set_label("Clicked");
        wind.set_label("Clicked")
    });

    let mut buffer = TextBuffer::default();
    for bin in binaries.iter() {
        buffer.append(&format!("{}\n", bin));
        disp.set_buffer(buffer.clone())
    }

    app.run().unwrap();

    // Helper function for find binary
    // let elems: Vec<String> = binaries.into_iter().filter(|e| e.contains("kin")).collect();
    // println!("{:?}", elems);
    //
    // let mut i = 0;
    // while app.wait() {
    //     if i == binaries.len() {
    //         i = 0;
    //     }
    //     frame.set_label_color(enums::Color::White);
    //     frame.set_label(&format!("[{}]", binaries[i]));
    //     app::sleep(0.5);
    //     i += 1;
    //}
}

fn theme_button(btn: &mut button::Button) {
    btn.set_color(Color::from_rgb(255, 0, 0).darker());
}
