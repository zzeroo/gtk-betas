extern crate gtk_betas;
extern crate xmz_server;

use gtk_betas::errors::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use xmz_server::Kombisensor;


fn setup_kombisensors() -> Result<Arc<Mutex<Vec<Kombisensor>>>> {
    // Test Vector of Kombisensors
    let mut kombisensors: Vec<Kombisensor> = vec![];

    // For testing we add 10 Kombisensors, Modbus Slave ID from 50 to 60
    for id in 50..61 {
        let mut kombisensor = Kombisensor::new();
        kombisensor.set_modbus_slave_id(id);
        kombisensors.push(kombisensor);
    }

    let kombisensors = Arc::new(Mutex::new(kombisensors));

    Ok(kombisensors)
}

fn run() -> Result<()> {
    if let Ok(kombisensors) = setup_kombisensors() {
        gtk_betas::gui::index::launch(kombisensors);
    }

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
