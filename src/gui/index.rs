use errors::*;
use gdk::enums::key;
use glib;
use glib::Value;
use gobject_sys;
use gtk_sys;
use gtk;
use gtk::prelude::*;
use xmz_server::*;
use glib::translate::ToGlibPtr;


pub fn launch() {
    if gtk::init().is_err() {
        println!("Failed to initalize GTK.");

        ::std::process::exit(1);
    }
    // Disable Animationen
    // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
    // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    unsafe{
        gobject_sys::g_object_set (gtk_sys::gtk_settings_get_default() as *mut gobject_sys::GObject,
        "gtk-enable-animations".to_glib_none().0, 0, 0);
    }

    let glade_str = include_str!("gui.glade");
    let builder = gtk::Builder::new();
    builder.add_from_string(&glade_str);

    let window_main: gtk::Window = build!(builder, "window_main");
    let infobar: gtk::InfoBar = build!(builder, "infobar");


    window_main.connect_delete_event(|_, _| {
        gtk::main_quit();

        Inhibit(false)
    });

    #[cfg(feature = "development")]
    window_main.connect_key_press_event(move |_, key| {
        if let key::Escape = key.get_keyval() {
            gtk::main_quit()
        }
        Inhibit(false)
    });

    window_main.show_all();

    gtk::main();
}
