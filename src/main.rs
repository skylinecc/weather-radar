use gio::prelude::*;
use gtk::prelude::*;
use std::env;
use gtk::AboutDialog;
use gtk::License::Gpl30;

fn main() {
    let uiapp = gtk::Application::new(Some("org.skylinecc.WeatherRadar"),
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);
        let header_bar = gtk::HeaderBar::new();

        header_bar.set_title(Some("Radar Viewer"));
        header_bar.set_show_close_button(true);

        let location_chooser = gtk::ComboBoxText::new();
        header_bar.add(&location_chooser);

        let popover_menu = gtk::PopoverMenu::new();
        let popover_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
        popover_menu.set_property_width_request(100);
        popover_menu.add(&popover_box);

        let about_button = gtk::ModelButton::new();
        let menu_separator = gtk::Separator::new(gtk::Orientation::Vertical);
        let quit_button = gtk::ModelButton::new();

        popover_box.pack_start(&about_button, false, false, 0);
        popover_box.pack_start(&menu_separator, false, false, 0);
        popover_box.pack_start(&quit_button, false, false, 0);

        let menu_btn = gtk::MenuButton::new();
        menu_btn.set_image(Some(&gtk::Image::new_from_icon_name(Some("open-menu-symbolic"), gtk::IconSize::SmallToolbar)));
        menu_btn.set_use_popover(true);
        menu_btn.set_popover(Some(&popover_menu));

        header_bar.pack_end(&menu_btn);

        let window_frame = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Then we set its size and a title.
        win.set_default_size(320, 200);
        win.set_titlebar(Some(&header_bar));

        win.add(&window_frame);

        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}

fn about() {

	let about_window = AboutDialog::new();
	about_window.set_website_label(Some("Weather Radar"));
	about_window.set_website(Some("https://github.com/skylinecc/weather-radar"));
	about_window.set_website_label(Some("Project Page"));
	about_window.set_comments(Some("Get Radar Information from the National Weather Service."));
	about_window.set_copyright(Some("Copyright © 2020 Skyline Coding Club"));
	about_window.set_license_type(Gpl30);
	about_window.set_wrap_license(false);
//	about_window.set_logo(Some(&Pixbuf::new_from_resource("/org/skylinecc/Gtkdd/org.skylinecc.Gtkdd.svg").unwrap()));
	about_window.set_title("About Weather Radar");
	about_window.set_authors(&["Grant Handy"]);
    about_window.add_credit_section(&"Club Members", &[
        "Nicholas Zhang",
		"Ethan Suresh",
		"Alex Ikeda",
		"Evan Ikeda",
		"Corrine Wang",
		"Miguel Oyarzun",
		"Grant Handy",
		"Michael Donnely",
		"Ayush Ranjan",
		"Alex Rose",
	]);
    about_window.show_all();

}
