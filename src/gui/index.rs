use errors::*;
use gdk::enums::key;
use glib;
use glib::translate::ToGlibPtr;
use glib::Value;
use gobject_sys;
use gtk_sys;
use gtk;
use gtk::prelude::*;
use std::collections::HashSet;


struct KombisensorList {
    list_box: gtk::ListBox,
    rows: HashSet<gtk::ListBoxRow>,
}

impl KombisensorList {
    pub fn new(builder: &gtk::Builder) -> Self {
        let list_box: gtk::ListBox = build!(builder, "list_box_sensors");

        KombisensorList {
            list_box: list_box,
            rows: HashSet::new(),
        }
    }

    pub fn kombisensor_to_row(&mut self, builder: &gtk::Builder) {
        let modbus_slave_id = "100";
        let kombisensor_type = "CO/ NO2 Kombisensor";
        let sensor1_sensor_type = "NO2";
        let sensor1_average = "30.0";
        let sensor1_value = "20.1";
        let sensor1_si = "ppm";
        let sensor2_sensor_type = "CO";
        let sensor2_average = "280.0";
        let sensor2_value = "180.3";
        let sensor2_si = "ppm";

        let template_list_row_kombisensor_glade = include_str!("template_list_row_kombisensor.glade");
        builder.add_from_string(&template_list_row_kombisensor_glade);

        let template_list_row_kombisensor: gtk::ListBoxRow = build!(builder, "template_list_row_kombisensor");


        let textview_modbus_slave_id: gtk::TextView = build!(builder, "textview_modbus_slave_id");
        let textview_kombisensor_type: gtk::TextView = build!(builder, "textview_kombisensor_type");
        let textview_sensor1_sensor_type: gtk::TextView = build!(builder, "textview_sensor1_sensor_type");
        let textview_sensor1_average: gtk::TextView = build!(builder, "textview_sensor1_average");
        let textview_sensor1_value: gtk::TextView = build!(builder, "textview_sensor1_value");
        let textview_sensor1_si: gtk::TextView = build!(builder, "textview_sensor1_si");
        let textview_sensor2_sensor_type: gtk::TextView = build!(builder, "textview_sensor2_sensor_type");
        let textview_sensor2_average: gtk::TextView = build!(builder, "textview_sensor2_average");
        let textview_sensor2_value: gtk::TextView = build!(builder, "textview_sensor2_value");
        let textview_sensor2_si: gtk::TextView = build!(builder, "textview_sensor2_si");

        textview_modbus_slave_id.get_buffer().unwrap().set_text(&modbus_slave_id);
        textview_kombisensor_type.get_buffer().unwrap().set_text(&kombisensor_type);
        textview_sensor1_sensor_type.get_buffer().unwrap().set_text(&sensor1_sensor_type);
        textview_sensor1_average.get_buffer().unwrap().set_text(&sensor1_average);
        textview_sensor1_value.get_buffer().unwrap().set_text(&sensor1_value);
        textview_sensor1_si.get_buffer().unwrap().set_text(&sensor1_si);
        textview_sensor2_sensor_type.get_buffer().unwrap().set_text(&sensor2_sensor_type);
        textview_sensor2_average.get_buffer().unwrap().set_text(&sensor2_average);
        textview_sensor2_value.get_buffer().unwrap().set_text(&sensor2_value);
        textview_sensor2_si.get_buffer().unwrap().set_text(&sensor2_si);

        self.list_box.insert(&template_list_row_kombisensor, -1);
    }
}


fn window_main_setup(window: &gtk::Window) -> Result<()> {
    let window_title = format!("{} {}",
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_VERSION"));

    window.set_title(&window_title);

    if let Some(display) = window.get_display() {
        let screen = display.get_screen(0);
        screen.set_resolution(130.0);

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
    // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
    // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    unsafe{
        gobject_sys::g_object_set (gtk_sys::gtk_settings_get_default() as *mut gobject_sys::GObject,
        "gtk-enable-animations".to_glib_none().0, 0, 0);
    }

    let builder = gtk::Builder::new();

    let gui_glade = include_str!("gui.glade");
    builder.add_from_string(&gui_glade);

    let window_main: gtk::Window = build!(builder, "window_main");
    let infobar: gtk::InfoBar = build!(builder, "infobar");
    let label_infobar_msg: gtk::Label = build!(builder, "label_infobar_msg");

    label_infobar_msg.set_text("Anwendung erfolgreich gestartet!");

    // Basic setup des Haupt Fensters
    window_main_setup(&window_main);

    let mut kombisensor_list = KombisensorList::new(&builder);
    for i in 0..201 {
        kombisensor_list.kombisensor_to_row(&builder);
    }

    // Close Action der InfoBar
    infobar.connect_response(clone!(infobar => move |infobar, _| {
        infobar.hide();
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
