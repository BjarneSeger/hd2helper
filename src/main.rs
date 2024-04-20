// Copyright (C) 2024 Bjarne Seger
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/old-licenses/gpl-2.0.txt>.

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::Cell;

mod db;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.github.BjarneSeger.hd2helper")
        .build();

    application.connect_startup(|_| load_css());

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Helldivers 2 Helper")
            .default_width(350)
            .default_height(70)
            .build();

        let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();

        let refresh_button = gtk::Button::from_icon_name("view-refresh");
        let menubar = gtk::HeaderBar::new();
        menubar.pack_end(&refresh_button);
        window.set_titlebar(Some(&menubar));

        // Allow users to select only specific Stratagems
        let mut avail_stratagems: &mut Vec<db::Stratagem> = &mut Vec::new();
        for _ in 0..10 {
            avail_stratagems.push(rand::random());
        }

        let global_stratagem = 
            Rc::new(Cell::new(rand::random::<db::Stratagem>()));

        let current_stratagem = avail_stratagems[0];        

        let prompt = gtk::Label::new(Some(
            format!("Enter Keycode for {current_stratagem}").as_str())
        );
        grid.attach(&prompt, 0,0,1,1);

        // Graphical representation of the stratagem...
        let stratagem_scaled = gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(
            global_stratagem.get().get_image_path(), 3840, 2160, true).unwrap();
        let stratagem_image = gtk::Picture::for_paintable(&gtk::gdk::Texture::for_pixbuf(&stratagem_scaled));
        stratagem_image.set_can_shrink(true);
        let keycode = global_stratagem.get().get_keycode();
        grid.attach(&stratagem_image, 0,2,1,1); 

        // ... and the keycode
        let keycode_visual = gtk::Label::new(Some(get_keycode_str(&keycode).as_str()));
        keycode_visual.add_css_class("arrows");
        grid.attach(&keycode_visual, 0, 1, 1, 1);

        // Shortcuts for training Stratagem codes
        let input_buffer = Arc::new(Mutex::new(Vec::new()));
        let event_controller = gtk::EventControllerKey::new();
        event_controller.connect_key_pressed( move |_, key, _, _| {
           keybinds_magic(key, input_buffer.clone(), &prompt, &stratagem_image, &keycode_visual, global_stratagem.clone())
        });

        window.add_controller(event_controller);
        window.set_child(Some(&grid));
        window.present();
    });

    application.run()
}

fn build_ui(application: &Application) {

}

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_path("src/style.css");

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// This is what happens when a guess was made 
fn refresh_stratagem(/*avail_stratagems: &mut Vec<db::Stratagem>,*/
                     current_stratagem: Rc<Cell<db::Stratagem>>,
                     label: &gtk::Label,
                     picture: &gtk::Picture,
                     arrows: &gtk::Label,
                     ) {
    current_stratagem.set(rand::random());
    let stratagem_scaled = gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(
        current_stratagem.get().get_image_path(), 3840, 2160, true).unwrap();

    picture.set_paintable(Some(&gtk::gdk::Texture::for_pixbuf(&stratagem_scaled)));
    
    let printable = current_stratagem.get().to_string();
    label.set_text(format!("Enter Keycode for {printable}").as_str());

    let keycode = current_stratagem.get().get_keycode();
    arrows.set_text(get_keycode_str(&keycode).as_str());
}

/// This function checks the combination and modifies the input buffer as well as
/// the ui if it needs to be cleared
fn keybinds_magic(key: gtk::gdk::Key,
                  input_buffer: Arc<Mutex<Vec<db::Code>>>,
                  label: &gtk::Label,
                  picture: &gtk::Picture,
                  arrows: &gtk::Label,
                  current_stratagem: Rc<Cell<db::Stratagem>>
                  ) -> glib::Propagation{
    use gtk::gdk::Key;
    use db::Code;
    match key {
        Key::w => input_buffer.lock().unwrap().push(Code::Up),
        Key::a => input_buffer.lock().unwrap().push(Code::Left),
        Key::s => input_buffer.lock().unwrap().push(Code::Down),
        Key::d => input_buffer.lock().unwrap().push(Code::Right),
        // Would like to make this a seperate function, but the passing
        // around of things would be anything but nice
        Key::F5 => {
            refresh_stratagem(current_stratagem.clone(), label, picture, arrows);
            input_buffer.lock().unwrap().clear();
            },
        _ => (),
    }
    if input_buffer.lock().unwrap().to_vec() == current_stratagem.get().get_keycode() {
        refresh_stratagem(current_stratagem, label, picture, arrows);
        input_buffer.lock().unwrap().clear();
    }
    glib::Propagation::Proceed
}

fn get_keycode_str(keycode_vec: &Vec<db::Code>) -> String {
    if keycode_vec.len() == 0 {return "".to_string()}
    let mut string = keycode_vec[0].get_arrow().to_owned();
    for i in 1..keycode_vec.len() {
        string = string + keycode_vec[i].get_arrow();
    }
    string
}
