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
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use xmz_server::Kombisensor;



struct KombisensorList {
    list_box: gtk::ListBox,
}

impl KombisensorList {
    pub fn new(builder: &gtk::Builder) -> Self {
        let list_box: gtk::ListBox = build!(builder, "list_box_sensors");

        KombisensorList {
            list_box: list_box,
        }
    }

    pub fn get_list_box(&self) -> &gtk::ListBox {
        &self.list_box
    }

    pub fn kombisensor_to_row(&mut self, builder: &gtk::Builder, kombisensor: &Kombisensor) -> gtk::ListBoxRow {
        let glade_list_box_row_kombisensor = include_str!("list_box_row_kombisensor.glade");
        builder.add_from_string(&glade_list_box_row_kombisensor);

        let list_box_row_kombisensor: gtk::ListBoxRow = build!(builder, "list_box_row_kombisensor");


        let label_modbus_slave_id: gtk::Label = build!(builder, "label_modbus_slave_id");
        let label_kombisensor_type: gtk::Label = build!(builder, "label_kombisensor_type");
        let label_sensor1_sensor_type: gtk::Label = build!(builder, "label_sensor1_sensor_type");
        let label_sensor1_average: gtk::Label = build!(builder, "label_sensor1_average");
        let label_sensor1_value: gtk::Label = build!(builder, "label_sensor1_value");
        let label_sensor1_si: gtk::Label = build!(builder, "label_sensor1_si");
        let label_sensor2_sensor_type: gtk::Label = build!(builder, "label_sensor2_sensor_type");
        let label_sensor2_average: gtk::Label = build!(builder, "label_sensor2_average");
        let label_sensor2_value: gtk::Label = build!(builder, "label_sensor2_value");
        let label_sensor2_si: gtk::Label = build!(builder, "label_sensor2_si");

        label_modbus_slave_id.set_text(&kombisensor.get_modbus_slave_id().to_string());
        label_kombisensor_type.set_text(&kombisensor.get_kombisensor_type());

        if let Some(sensor1) = kombisensor.get_sensor(0) {
            label_sensor1_sensor_type.set_text(&sensor1.get_sensor_type());
            label_sensor1_average.set_text("n/a");
            label_sensor1_value.set_text(&sensor1.get_concentration().to_string());
            label_sensor1_si.set_text(&sensor1.get_si());
        }

        if let Some(sensor2) = kombisensor.get_sensor(1) {
            label_sensor2_sensor_type.set_text(&sensor2.get_sensor_type());
            label_sensor2_average.set_text("n/a");
            label_sensor2_value.set_text(&sensor2.get_concentration().to_string());
            label_sensor2_si.set_text(&sensor2.get_si());
        }

        list_box_row_kombisensor
    }
}


fn setup_kombisensors() -> Arc<Mutex<Vec<Kombisensor>>> {
    // Test Vector of Kombisensors
    let mut kombisensors: Vec<Kombisensor> = vec![];

    // For testing we add 10 Kombisensors, Modbus Slave ID from 50 to 60
    for id in 50..60 {
        let mut kombisensor = Kombisensor::new();
        kombisensor.set_modbus_slave_id(id);
        kombisensors.push(kombisensor);
    }

    let kombisensors = Arc::new(Mutex::new(kombisensors));

    kombisensors
}


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
    // http://stackoverflow.com/questions/39271852/infobar-only-shown-on-window-change/39273438#39273438
    // https://gitter.im/gtk-rs/gtk?at=57c8681f6efec7117c9d6b5e
    unsafe{
        gobject_sys::g_object_set (gtk_sys::gtk_settings_get_default() as *mut gobject_sys::GObject,
        "gtk-enable-animations".to_glib_none().0, 0, 0);
    }

    let builder = gtk::Builder::new();

    let gui_glade = include_str!("list_box.glade");
    builder.add_from_string(&gui_glade);

    let window_main: gtk::Window = build!(builder, "window_main");
    let infobar: gtk::InfoBar = build!(builder, "infobar");
    let label_infobar_msg: gtk::Label = build!(builder, "label_infobar_msg");

    label_infobar_msg.set_text("Anwendung erfolgreich gestartet!");

    // Basic setup des Haupt Fensters
    window_main_setup(&window_main);

    // This part of the application, holds the gtk Listbox
    let mut kombisensor_list = KombisensorList::new(&builder);

    let kombisensors = setup_kombisensors();

    // GUI Update Task
    gtk::idle_add(clone!(kombisensors => move || {
        if let Ok(kombisensors) = kombisensors.try_lock() {
            let lenght = kombisensors.len() as i32;
            for (i, kombisensor) in kombisensors.iter().enumerate() {
                // Remove all Rows for they we dont have a kombisensor. If there are less kombisensors then rows.
                while let Some(row) = kombisensor_list.get_list_box().get_row_at_index(lenght + 1) {
                    println!("lenght+1: {}", lenght + 1);
                    println!("Drop row: {}", i);
                    kombisensor_list.get_list_box().remove(&row);
                }

                // If we have a row overwrite
                if let Some(row) = kombisensor_list.get_list_box().get_row_at_index(i as i32) {
                    println!("update row: {}", i);
                    let row = kombisensor_list.kombisensor_to_row(&builder, &kombisensor);
                    kombisensor_list.get_list_box().insert(&row, i as i32);
                } else {
                    // If we dont have a row, create one
                    println!("CREATE row: {}", i);
                    let row = kombisensor_list.kombisensor_to_row(&builder, &kombisensor);
                    kombisensor_list.get_list_box().insert(&row, -1);
                }

            }
        }

        thread::sleep(Duration::from_millis(100));

        ::glib::Continue(true)
    }));

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
