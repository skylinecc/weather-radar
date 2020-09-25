use gio::prelude::*;
use gtk::prelude::*;
use gtk::{License::Gpl30, Builder, AboutDialog};
use std::env;

fn main() {
	let uiapp = gtk::Application::new(Some("org.skylinecc.WeatherRadar"),
		gio::ApplicationFlags::FLAGS_NONE)
		.expect("Application::new failed");

//	Load the compiled resource bundle
	let resources_bytes = include_bytes!("../data/resources.gresource");
	let resource_data = glib::Bytes::from(&resources_bytes[..]);
	let res = gio::Resource::new_from_data(&resource_data).unwrap();
	gio::resources_register(&res);

//	Load the window UI
	uiapp.connect_activate( |app| {
        let win = gtk::ApplicationWindow::new(app);

        let builder = Builder::new_from_resource("/org/skylinecc/WeatherRadar/menu.ui");
		let popover_menu: gtk::PopoverMenu = builder.get_object("popover_menu").expect("Couldn't get popov);_menu");
		let about_button: gtk::ModelButton = builder.get_object("about_button").expect("Couldn't get about_button");

//        let location_chooser = gtk::ComboBoxText::new();

        let menu_btn = gtk::MenuButton::new();
        menu_btn.set_image(Some(&gtk::Image::new_from_icon_name(Some("open-menu-symbolic"), gtk::IconSize::SmallToolbar)));
        menu_btn.set_use_popover(true);
        menu_btn.set_popover(Some(&popover_menu));

        let header_bar = gtk::HeaderBar::new();
        header_bar.set_title(Some("Radar Viewer"));
        header_bar.set_show_close_button(true);
//		header_bar.add(&location_chooser);
		header_bar.pack_end(&menu_btn);

        let window_frame = gtk::Box::new(gtk::Orientation::Vertical, 0);

        // Then we set its size and a title.
        win.set_default_size(500, 500);
        win.set_titlebar(Some(&header_bar));

        win.add(&window_frame);

        about_button.connect_clicked(move |_| {
        	about();
        });

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
