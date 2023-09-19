use std::{
    fs,
    io::ErrorKind,
    path::{self, PathBuf},
};

use fltk::{prelude::*, *};

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
    let app = app::App::default();
    // To load a font by path, check the App::load_font() method
    let binaries = read_dir();
    // println!("{:?}", fonts);
    let mut wind = window::Window::default().with_size(400, 300);
    let mut frame = frame::Frame::default().size_of(&wind);
    frame.set_label_size(30);
    wind.set_color(enums::Color::White);
    wind.end();
    wind.show();
    println!(
        "The system has {} binaries!\nStarting slideshow!",
        binaries.len()
    );
    let mut i = 0;
    while app.wait() {
        if i == binaries.len() {
            i = 0;
        }
        frame.set_label(&format!("[{}]", binaries[i]));
        app::sleep(0.5);
        i += 1;
    }
}
