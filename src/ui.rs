use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FileChooserAction, FileChooserDialog};

pub fn build_ui(application; &Application ){
    let window= ApplicationWindow::new(application);
    window.set_title("TheAudioPlayer");
    window.set_default_size(300,100);
    let play_button= Button::with_label("Play");
    let pause_button=Button::with_label("Pause");
    let stop_button =Button::with_label("Stop");
    let vbox= gtk::Box::new(gtk::Orientation::Vertical,5);
    vbox.pack_start(&play_button, true, true, 0);
    vbox.pack_start(&pause_button, true, true, 0);
    vbox.pack_start(&stop_button, true, true, 0);
    window.add(&vbox);
    window.show_all();
    
}