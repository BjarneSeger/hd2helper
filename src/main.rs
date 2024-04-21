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

use adw::prelude::*;
use gtk::{glib, Application};
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

mod db;
mod gui_templates;
use gui_templates::AvailablePages::*;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.github.BjarneSeger.hd2helper")
        .build();

    application.connect_startup(|_| load_css());

    application.connect_activate(|app| {
        // #TODO:
        // Allow users to select only specific Stratagems
        let mut avail_stratagems: &mut Vec<db::Stratagem> = &mut Vec::new();
        for _ in 0..10 {
            avail_stratagems.push(rand::random());
        }

        let global_stratagem = 
            Rc::new(Cell::new(rand::random::<db::Stratagem>()));

        let current_stratagem = global_stratagem.get();
        let prompt = gtk::Label::new(Some(
            format!("Enter Keycode for {current_stratagem}").as_str())
        );

        // Graphical representation of the stratagem...
        let stratagem_scaled = gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(
            global_stratagem.get().get_image_path(), 3840, 2160, true).unwrap();
        let stratagem_image = gtk::Picture::for_paintable(&gtk::gdk::Texture::for_pixbuf(&stratagem_scaled));
        stratagem_image.set_can_shrink(true);
        let keycode = global_stratagem.get().get_keycode();
        // grid.attach(&stratagem_image, 0,2,1,1); 

        // ... and the keycode
        let keycode_visual = gtk::Label::new(Some(get_keycode_str(&keycode).as_str()));
        keycode_visual.add_css_class("arrows");
        // grid.attach(&keycode_visual, 0, 1, 1, 1);
        let right_page = gui_templates::create_main_page(KeycodeTrainer{
            stratagem_picture: &stratagem_image,
            prompt: &prompt,
            keycode: &keycode_visual,
        });

        let overview = gtk::ListBox::builder()
            .css_classes(*&["navigation-sidebar"])
            .build();
        overview.append(&gtk::ListBoxRow::builder()
                        .child(&gtk::Label::new(Some("Keycode Trainer")))
                        .build());
        overview.append(&gtk::ListBoxRow::builder()
                        .child(&gtk::Label::new(Some("Coming soon...")))
                        .build());

        let sidebar = gui_templates::create_sidebar(
            &gui_templates::create_adw_toolbar(&gtk::Label::new(Some("Helldivers 2 Helper"))),
            &overview
            );

        let main_content = adw::NavigationSplitView::builder()
            .content(&right_page)
            .sidebar(&sidebar)
            .build();

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Helldivers 2 Helper")
            .content(&main_content)
            .css_classes(*&["background", "csd"])
            .default_width(550)
            .default_height(550)
            .build();
        
        // Shortcuts for training Stratagem codes
        let input_buffer = Rc::new(RefCell::new(global_stratagem.get().get_keycode()));
        let event_controller = gtk::EventControllerKey::new();
        event_controller.connect_key_pressed( move |_, key, _, _| {
           keybinds_magic(key, input_buffer.clone(), &prompt, &stratagem_image, &keycode_visual, global_stratagem.clone())
        });

        window.add_controller(event_controller);
        window.present();
    });

    application.run()
}

fn build_ui(application: &Application) {

}

// Get CSS for some visual things
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

/// This refreshes the Label, picture, Arrows and Stratagem when the UI needs
/// to be refreshed
fn refresh_stratagem(current_stratagem: Rc<Cell<db::Stratagem>>,
                     current_input: Rc<RefCell<Vec<db::Code>>>,
                     label: &gtk::Label,
                     picture: &gtk::Picture,
                     arrows: &gtk::Label,
                     ) {
    current_stratagem.set(rand::random());
    let stratagem_scaled = gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(
        current_stratagem.get().get_image_path(), 3840, 2160, true).unwrap();

    current_input.replace(current_stratagem.get().get_keycode());

    picture.set_paintable(Some(&gtk::gdk::Texture::for_pixbuf(&stratagem_scaled)));
    
    let printable = current_stratagem.get().to_string();
    label.set_text(format!("Enter Keycode for {printable}").as_str());

    let keycode = current_stratagem.get().get_keycode();
    arrows.set_text(get_keycode_str(&keycode).as_str());
}

/// This function checks the combination and modifies the input buffer as well as
/// the ui if it needs to be cleared
fn keybinds_magic(key: gtk::gdk::Key,
                  input_buffer: Rc<RefCell<Vec<db::Code>>>,
                  label: &gtk::Label,
                  picture: &gtk::Picture,
                  arrows: &gtk::Label,
                  current_stratagem: Rc<Cell<db::Stratagem>>
                  ) -> glib::Propagation{
    use gtk::gdk::Key;
    use db::Code;
    let refresh = match key {
        Key::w => check_refresh(Code::Up, input_buffer.clone()),
        Key::a => check_refresh(Code::Left, input_buffer.clone()),
        Key::s => check_refresh(Code::Down, input_buffer.clone()),
        Key::d => check_refresh(Code::Right, input_buffer.clone()),
        Key::F5 => true,
        _ => false,
    };
    if refresh == true {
        refresh_stratagem(current_stratagem, input_buffer.clone(), label, picture, arrows);
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

fn check_refresh(current_input: db::Code,
                 keycode: Rc<RefCell<Vec<db::Code>>>) -> bool {
    if keycode.borrow().len() == 0 {
        println!("keycode buffer was empty!");
        return true;
    }
    if keycode.borrow_mut().remove(0) == current_input {
        if keycode.borrow().len() != 0 {
            return false;
        }
    }
    true
}
