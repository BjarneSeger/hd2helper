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

// Here we create some abstractions for the GUI, like creating a toolbar that
// looks like the one in the GNOME 46 settings. This also hopefully increases
// readability

/// This function creates a title similar to the one in the new GNOME 46
/// settings and returns it. Because this is a Label in a CenterBox in a
/// WindowHandle in a HeaderBar, it got a custom function.
/// It is also automatically added to the adwaita-sidebar-title css-class
///
/// It also takes a &gtk::Label instead of a simple string so you can retain
/// control over the text in there and change it at runtime. If you don't want
/// that, just use gtk::Label::new() as the title
pub fn create_adw_toolbar(title: &gtk::Label) -> gtk::WindowHandle {
    gtk::WindowHandle::builder()
        .child(&gtk::CenterBox::builder()
            .margin_top(15)
            .margin_bottom(15)
            .center_widget(&adw::Bin::builder()
                .css_classes(*&["adwaita-sidebar-title"])
                .child(title)
                .build())
            .build())
        .build() 
}

/// Creates a complete sidebar in the new style from the settings in GNOME 46
/// and returns it as a NavigationPage. It is wrapped in a Viewport in a
/// ScrolledWindow to enable scrolling for a ListBox.
pub fn create_sidebar(titlebar: &gtk::WindowHandle,
                      content: &impl gtk::prelude::IsA<gtk::Widget>)
                      -> adw::NavigationPage {
    let toolbar = adw::ToolbarView::new();
    toolbar.add_top_bar(titlebar);
    let toolbar_content = &gtk::ScrolledWindow::builder()
        .child(&gtk::Viewport::builder()
                                      .child(content)
                                      .build())
                               .build();
    toolbar.set_content(Some(toolbar_content));
    adw::NavigationPage::builder()
        .child(&toolbar)
        .title("Sidebar")
        .css_classes(*&["sidebar-pane"])
        .build()
}

/// This is used to determine the page to be displayed as the main page (for
/// example from a click on the sidebar)
pub enum AvailablePages<'a> {
    NotYetImplemented,
    KeycodeTrainer{
        stratagem_picture: &'a gtk::Picture,
        prompt: &'a gtk::Label,
        keycode: &'a gtk::Label
    }
}

pub fn create_main_page(page: AvailablePages) -> adw::NavigationPage{
    match page {
        AvailablePages::KeycodeTrainer{stratagem_picture, prompt, keycode} =>
            create_keycode_page(stratagem_picture, prompt, keycode),
        _ => create_wip_page(),
    }
}

fn create_keycode_page(stratagem_picture: &gtk::Picture,
                       prompt: &gtk::Label,
                       keycode: &gtk::Label)
                       -> adw::NavigationPage {
    let toolbar = adw::ToolbarView::new();
    let top_bar = gtk::CenterBox::builder()
        .margin_top(15)
        .margin_bottom(15)
        .margin_end(15)
        .margin_start(15)
        .center_widget(prompt)
        .css_classes(*&["keycode-title"])
        .end_widget(&gtk::WindowControls::new(gtk::PackType::End))
        .build();
    toolbar.add_top_bar(&top_bar);
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 6);
    gtk::prelude::BoxExt::append(&main_box, stratagem_picture);
    gtk::prelude::BoxExt::append(&main_box, keycode);
    toolbar.set_content(Some(&main_box));
    adw::NavigationPage::builder()
        .title("Keycode Trainer")
        .child(&toolbar)
        .build()
}

fn create_wip_page() -> adw::NavigationPage {
    adw::NavigationPage::builder()
        .title("Work in Progress...")
        .child(&adw::StatusPage::builder()
               .title("Nothing to see here.")
               .icon_name("help-contents")
               .build())
        .build()
}
