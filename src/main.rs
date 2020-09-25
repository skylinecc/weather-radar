use gio::prelude::*;
use gtk::prelude::*;
use gtk::{License::Gpl30, Builder, AboutDialog};
use std::env;
use std::fs::File;
use std::io::copy;
use std::io::prelude::*;
use glib::clone;
use ureq;

fn main() {
	let uiapp = gtk::Application::new(Some("org.skylinecc.WeatherRadar"),
		gio::ApplicationFlags::FLAGS_NONE)
		.expect("Application::new failed");

	let gif = ureq::get("https://radar.weather.gov/ridge/Conus/Loop/northrockies_loop.gif").call().into_string().unwrap();

	let mut file = std::fs::File::create("map.gif").expect("create failed");
	file.write_all(gif.as_bytes()).expect("write failed");

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

        let location_chooser = gtk::ComboBoxText::new();
        location_chooser.append(Some("northrockies"), "Northern Rocky Mountains");
        location_chooser.append(Some("southrockies"), "Southern Rocky Mountains");

        let menu_btn = gtk::MenuButton::new();
        menu_btn.set_image(Some(&gtk::Image::new_from_icon_name(Some("open-menu-symbolic"), gtk::IconSize::SmallToolbar)));
        menu_btn.set_use_popover(true);
        menu_btn.set_popover(Some(&popover_menu));

        let header_bar = gtk::HeaderBar::new();
        header_bar.set_title(Some("Radar Viewer"));
        header_bar.set_show_close_button(true);
		header_bar.add(&location_chooser);
		header_bar.pack_end(&menu_btn);

        northrockies();
        let northrockies_gif = gtk::Image::new_from_file("/tmp/northrockies.gif");

        southrockies();
        let southrockies_gif = gtk::Image::new_from_file("/tmp/southrockies.gif");

        let map_stack = gtk::Stack::new();
        map_stack.add_named(&northrockies_gif, "north_rockies");
        map_stack.add_named(&southrockies_gif, "south_rockies");

        map_stack.set_visible_child(&northrockies_gif);

        location_chooser.connect_changed(clone!(@weak map_stack => move |location_chooser| {
            if location_chooser.get_active_id().unwrap() == "northrockies" {
                map_stack.set_visible_child(&northrockies_gif);
            } else if location_chooser.get_active_id().unwrap() == "southrockies" {
                map_stack.set_visible_child(&southrockies_gif);
            }
        }));

        let window_frame = gtk::Box::new(gtk::Orientation::Vertical, 0);
        window_frame.pack_start(&map_stack, true, true, 0);

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

fn northrockies() {

    std::fs::remove_file("/tmp/northrockies.gif");

	let mut file = std::fs::File::create("/tmp/northrockies.gif").expect("create failed");

    let mut gif_r = ureq::get("https://radar.weather.gov/ridge/Conus/Loop/northrockies_loop.gif").call().into_reader();

    let mut gif_w: Vec<u8> = vec![];

    std::io::copy(&mut gif_r, &mut gif_w);

	file.write_all(&gif_w).expect("write failed");
}

fn southrockies() {

    std::fs::remove_file("/tmp/southrockies.gif");

	let mut file = std::fs::File::create("/tmp/southrockies.gif").expect("create failed");

    let mut gif_r = ureq::get("https://radar.weather.gov/ridge/Conus/Loop/southrockies_loop.gif").call().into_reader();

    let mut gif_w: Vec<u8> = vec![];

    std::io::copy(&mut gif_r, &mut gif_w);

	file.write_all(&gif_w).expect("write failed");
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
