use errors::*;
use gdk::enums::key;
use glib;
use glib::translate::ToGlibPtr;
use glib::Value;
use gobject_sys;
use gtk_sys;
use gtk;
use gtk::prelude::*;
use rand::Rng;
use std::thread;
use std::time::Duration;
use xmz_server::Kombisensor;


fn window_main_setup(window: &gtk::Window) -> Result<()> {
    let window_title = format!("{} {}",
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_VERSION"));

    window.set_title(&window_title);

    if let Some(display) = window.get_display() {
        let screen = display.get_screen(0);
        screen.set_resolution(150.0);

        // CSS Datei einbinden
        let css_style_provider = gtk::CssProvider::new();
        let css_gui = include_str!("gui.css");
        match css_style_provider.load_from_data(css_gui) {
            Ok(_) => {
                gtk::StyleContext::add_provider_for_screen(&screen, &css_style_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
            }
            Err(e) => {println!("Error: css_style_provider.load_from_data() failed: {}", e)}
        }
    }

    #[cfg(not(feature = "development"))]
    window.fullscreen();

    Ok(())
}


pub fn launch() {
    if gtk::init().is_err() {
        println!("Failed to initalize GTK.");

        ::std::process::exit(1);
    }
    // Disable Animationen
    // http://stackoverflow.com/questions/39271852/info_bar-only-shown-on-window-change/39273438#39273438
    // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    unsafe{
        gobject_sys::g_object_set (gtk_sys::gtk_settings_get_default() as *mut gobject_sys::GObject,
        "gtk-enable-animations".to_glib_none().0, 0, 0);
    }

    let builder = gtk::Builder::new();

    // let gui_glade = include_str!("tree_view.glade");
    let gui_glade = include_str!("tree_view_overlay.glade");
    builder.add_from_string(&gui_glade);

    let window_main: gtk::Window = build!(builder, "window_main");
    let info_bar: gtk::InfoBar = build!(builder, "info_bar");
    let label_info_bar_msg: gtk::Label = build!(builder, "label_info_bar_msg");

    label_info_bar_msg.set_text("Anwendung erfolgreich gestartet!");

    // Basic setup des Haupt Fensters
    window_main_setup(&window_main);

    // Fill Treestore with data
    let tree_store_kombisensors: gtk::TreeStore = build!(builder, "tree_store_kombisensors");
    for i in 1..10 {
        let tree_iter = tree_store_kombisensors.insert_with_values(
            None,
            None,
            &[0, 1],
            &[&i, &format!("Messstelle {}", i)]
        );
        // Sensor Messzellen
        for i in 1..3 {
            tree_store_kombisensors.insert_with_values(
                Some(&tree_iter),
                None,
                &[1, 2, 3, 4],
                &[&format!("Messzelle {}", i), &"0.0", &"0.0", &"ppm"]
            );
        }
    }

    // Get the treeview from the builder
    let tree_view_kombisensors: gtk::TreeView = build!(builder, "tree_view_kombisensors");
    // Expand all
    tree_view_kombisensors.expand_all();

    // connect model with view
    tree_view_kombisensors.set_model(Some(&tree_store_kombisensors));


    // Overlay
    let scolled_window_kombisensors: gtk::ScrolledWindow = build!(builder, "scolled_window_kombisensors");
    let overlay_wartung: gtk::Overlay = build!(builder, "overlay_wartung");
    let button_wartung: gtk::Button = build!(builder, "button_wartung");
    let box_test: gtk::Box = build!(builder, "box_test");
    overlay_wartung.add_overlay(&box_test);

    button_wartung.connect_clicked(move |_| {
        box_test.hide();
    });

    // // GUI Update Task idle
    // gtk::idle_add(clone!(builder => move || {
    //     println!("Ping");
    //
    //     thread::sleep(Duration::from_millis(100));
    //
    //     ::glib::Continue(true)
    // }));

    // GUI Update Task Timeout
    gtk::timeout_add(10000, clone!(builder => move || {
        let box_test: gtk::Box = build!(builder, "box_test");
        box_test.show();

        ::glib::Continue(true)
    }));

    // Close Action der InfoBar
    info_bar.connect_response(clone!(info_bar => move |info_bar, _| {
        info_bar.hide();
    }));

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
