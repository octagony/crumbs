use std::{
    fs,
    io::ErrorKind,
    path::{self, PathBuf},
};

use fltk::{prelude::*, *, input::{Input, FloatInput}, text::{TextDisplay, TextBuffer}};

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
    let binaries = read_dir();
    
    let mut wind = window::Window::default().with_size(400, 300).center_screen().with_label("Crumbs");
    

    let mut disp = TextDisplay::new(5, 5, 390, 250, None);

    let mut inp1 = FloatInput::new(0,0,50,50,"").center_x(&wind);

    let mut buffer = TextBuffer::default();
    for bin in binaries.iter(){
        buffer.append(&format!("{}\n", bin));
        disp.set_buffer(buffer.clone())
    }
    ;

    wind.end();
    wind.show();

    
    app.run().unwrap();
    
    // let mut i = 0;
    // while app.wait() {
    //     if i == binaries.len() {
    //         i = 0;
    //     }
    //     frame.set_label_color(enums::Color::White);
    //     frame.set_label(&format!("[{}]", binaries[i]));
    //     app::sleep(0.5);
    //     i += 1;
    // }
}
